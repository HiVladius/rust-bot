use teloxide::{
    payloads::SendMessageSetters,
    prelude::{Requester, ResponseResult},
    types::{ChatAction, Message},
    utils::command::BotCommands,
    Bot,
};

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::components::characters::send_random_characters::send_random_character;
use crate::components::search::character_search::found_character;
use crate::lazy_chat_ids::CHAT_IDS;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "Iniciar el bot")]
    Start,
    #[command(description = "Mostrar menú principal")]
    Menu,
    #[command(description = "Obtener un personaje aleatorio")]
    Random,
    #[command(description = "Buscar un personaje por nombre")]
    Buscar,
}
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
            "🔍 Buscar Personaje",
            "buscar",
        )],
        vec![InlineKeyboardButton::callback("ℹ️ Información", "info")],
    ])
}

pub async fn commands_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
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
            bot.send_message(msg.chat.id, "Menú principal:")
                .reply_markup(get_main_menu_keyboard())
                .await?;
        }
        Command::Random => {
            send_random_character(&bot, msg.chat.id).await?;
        }
        Command::Buscar => {
            // Extract search term from message text
            let search_term = msg
                .text()
                .and_then(|text| text.strip_prefix("/buscar"))
                .map(|text| text.trim())
                .unwrap_or_default();

            if search_term.is_empty() {
                bot.send_message(
                    msg.chat.id,
                    "Por favor proporciona un nombre para buscar.\nEjemplo: /buscar Rick",
                )
                .await?;
            } else {
                // Show that the bot is typing

                bot.send_chat_action(msg.chat.id, ChatAction::Typing)
                    .await?;

                match found_character(search_term).await {
                    Ok(message) => {
                        bot.send_message(msg.chat.id, message)
                            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                            .await?;
                    }
                    Err(err) => {
                        bot.send_message(msg.chat.id, format!("Error al buscar: {}", err))
                            .await?;
                    }
                }
            }
        }
    }
    Ok(())
}
