use chrono::{Duration, NaiveTime, Utc};
use dotenv::dotenv;
use lazy_static::lazy_static;
use reqwest;
use serde::Deserialize;

use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message, MessageId};
use tokio::sync::Mutex;
use rand::Rng;
use std::collections::HashSet;

lazy_static! {
    static ref CHAT_IDS: Mutex<HashSet<i64>> = Mutex::new(HashSet::new());
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    info: Info,
    results: Vec<Character>,
}

#[derive(Debug, Deserialize)]
struct Info {   
    next: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Character {
    name: String,
    status: String,
    species: String,
    #[serde(rename = "type")]
    character_type: String,
    image: String,
}

async fn get_random_character() -> Result<Character, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let mut characters = Vec::new();
    let mut next_url = Some("https://rickandmortyapi.com/api/character".to_string());

    while let Some(url) = next_url {
        let response = client.get(&url).send().await?;
        let api_response: ApiResponse = response.json().await?;
        characters.extend(api_response.results);
        next_url = api_response.info.next;
    }

    if characters.is_empty() {
        return Err("No se encontraron personajes".into());
    }

    let mut rng = rand::rng();
    let index = rng.random_range(0..characters.len());
    Ok(characters[index].clone())
}

// Definir los comandos del bot
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "Iniciar el bot")]
    Start,
    #[command(description = "Mostrar menú principal")]
    Menu,
    #[command(description = "Obtener un personaje aleatorio")]
    Random,
}

// Función para crear un teclado inline con botones
fn get_main_menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            "🎲 Personaje Aleatorio",
            "random_character",
        )],
        vec![InlineKeyboardButton::callback(
            "📝 Suscribirse",
            "subscribe",
        )],
        vec![InlineKeyboardButton::callback(
            "❌ Cancelar Suscripción",
            "unsubscribe",
        )],
        vec![InlineKeyboardButton::callback(
            "ℹ️ Información",
            "info",
        )],
    ])
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot = Bot::from_env();

    let bot_clone = bot.clone();
    tokio::spawn(async move {
        loop {
            let now = Utc::now().naive_utc();
            let target_time = {
                let today = now.date();
                let time = NaiveTime::from_hms_opt(9, 0, 0).unwrap();
                let target = today.and_time(time);
                
                if target > now {
                    target
                } else {
                    (today + Duration::days(1)).and_time(time)
                }
            };

            let duration = target_time - now;
            let sleep_duration = match duration.to_std() {
                Ok(d) => d,
                Err(_) => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(86400)).await;
                    continue;
                }
            };

            tokio::time::sleep(sleep_duration).await;

            match get_random_character().await {
                Ok(character) => {
                    let message = format!(
                        "Personaje del día:\n\nNombre: {}\nEstado: {}\nEspecie: {}\nTipo: {}\n{}",
                        character.name,
                        character.status,
                        character.species,
                        character.character_type,
                        character.image
                    );

                    let chat_ids = CHAT_IDS.lock().await.clone();
                    if chat_ids.is_empty() {
                        eprintln!("No hay chats suscritos");
                        continue;
                    }

                    for chat_id in chat_ids {
                        if let Err(e) = bot_clone
                            .send_message(ChatId(chat_id), &message)
                            .await
                        {
                            eprintln!("Error enviando mensaje: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("Error obteniendo personaje: {}", e),
            }
        }
    });

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(commands_handler),
        )
        .branch(
            Update::filter_callback_query()
                .endpoint(callback_handler),
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn commands_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            CHAT_IDS.lock().await.insert(msg.chat.id.0);
            bot.send_message(
                msg.chat.id,
                "¡Hola! Te enviaré un personaje de Rick y Morty cada día a las 9:00 AM UTC.",
            )
            .reply_markup(get_main_menu_keyboard())
            .await?;
        }
        Command::Menu => {
            bot.send_message(
                msg.chat.id,
                "Menú principal:",
            )
            .reply_markup(get_main_menu_keyboard())
            .await?;
        }
        Command::Random => {
            send_random_character(&bot, msg.chat.id).await?;
        }
    }
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(cb_data) = q.data {
        // Manejar correctamente el MaybeInaccessibleMessage
        if let Some(message) = q.message {
            // Usando match en lugar de if let para manejar los diferentes casos
            match message {
                teloxide::types::MaybeInaccessibleMessage::Message(msg) => {
                    let chat_id = msg.chat.id;
                    
                    match cb_data.as_str() {
                        "random_character" => {
                            bot.answer_callback_query(q.id).await?;
                            send_random_character(&bot, chat_id).await?;
                        }
                        "subscribe" => {
                            CHAT_IDS.lock().await.insert(chat_id.0);
                            bot.answer_callback_query(q.id).await?;
                            bot.send_message(
                                chat_id,
                                "¡Te has suscrito a las notificaciones diarias!"
                            ).await?;
                        }
                        "unsubscribe" => {
                            CHAT_IDS.lock().await.remove(&chat_id.0);
                            bot.answer_callback_query(q.id).await?;
                            bot.send_message(
                                chat_id,
                                "Has cancelado tu suscripción a las notificaciones diarias."
                            ).await?;
                        }
                        "info" => {
                            bot.answer_callback_query(q.id).await?;
                            bot.send_message(
                                chat_id,
                                "Este bot te envía información sobre personajes de Rick y Morty.\n\nComandos disponibles:\n/start - Iniciar el bot\n/menu - Mostrar menú principal\n/random - Obtener un personaje aleatorio"
                            ).await?;
                        }
                        _ => {
                            bot.answer_callback_query(q.id).await?;
                        }
                    }
                }
                teloxide::types::MaybeInaccessibleMessage::InaccessibleMessage(_) => {
                    // El mensaje es inaccesible (demasiado antiguo o eliminado)
                    bot.answer_callback_query(q.id)
                        .text("No se puede procesar este botón, intenta enviar /menu para obtener un nuevo menú")
                        .await?;
                }
            }
        } else {
            // No hay mensaje asociado al callback
            bot.answer_callback_query(q.id)
                .text("No se pudo procesar la solicitud")
                .await?;
        }
    }
    Ok(())
}

async fn send_random_character(bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
    match get_random_character().await {
        Ok(character) => {
            let message = format!(
                "Personaje aleatorio:\n\nNombre: {}\nEstado: {}\nEspecie: {}\nTipo: {}\n{}",
                character.name,
                character.status,
                character.species,
                character.character_type,
                character.image
            );
            bot.send_message(chat_id, message).await?;
        }
        Err(e) => {
            bot.send_message(chat_id, format!("Error: {}", e))
                .await?;
        }
    }
    Ok(())
}