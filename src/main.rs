// //! External dependencies
use chrono::{Duration, NaiveTime, Utc};
use dotenv::dotenv;
use teloxide::prelude::*;

// //! Internal dependencies
use bot_rust::components::characters::callback_handler::callback_handler;
use bot_rust::components::characters::command_handler::commands_handler;
use bot_rust::components::characters::command_handler::Command;
use bot_rust::components::characters::get_random_character::get_random_character;
use bot_rust::components::game::trivia_game::process_trivia_answer;
use bot_rust::lazy_chat_ids::CHAT_IDS;

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
                Ok((character, episodes)) => {
                    let message = format!(
                        "Personaje del día:\n\nNombre: {}\nEstado: {}\nEspecie: {}\nTipo: {}\n{}\n{}",
                        character.name,
                        character.status,
                        character.species,
                        character.character_type,
                        character.image,
                        episodes
                    );

                    let chat_ids = CHAT_IDS.lock().await.clone();
                    if chat_ids.is_empty() {
                        eprintln!("No hay chats suscritos");
                        continue;
                    }

                    for chat_id in chat_ids {
                        if let Err(e) = bot_clone.send_message(ChatId(chat_id), &message).await {
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
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_message().endpoint(process_trivia_answer));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
