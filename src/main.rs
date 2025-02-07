use chrono::{Duration, NaiveTime, Utc};
use dotenv::dotenv;
use lazy_static::lazy_static;
use reqwest;
use serde::Deserialize;

use teloxide::prelude::*;
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

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            if text == "/start" {
                bot.send_message(
                    msg.chat.id,
                    "¡Hola! Te enviaré un personaje de Rick y Morty cada día a las 9:00 AM UTC.",
                )
                .await?;
                
                CHAT_IDS.lock().await.insert(msg.chat.id.0);
            } else if text == "/random" {
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
                        bot.send_message(msg.chat.id, message).await?;
                    }
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Error: {}", e))
                            .await?;
                    }
                }
            }
        }
        Ok(())
    })
    .await;
}