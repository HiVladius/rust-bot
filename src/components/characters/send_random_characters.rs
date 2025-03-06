use teloxide::{
    prelude::{Requester, ResponseResult},
    types::ChatId,
    Bot,
};

use crate::components::characters::get_random_character::get_random_character;
use crate::components::characters::send_episode::get_episode_names;


pub async fn send_random_character(bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
    match get_random_character().await {
        Ok((character, episode_count)) => {
            let mut message = format!(
                "Personaje aleatorio:\n\nNombre: {}\nEstado: {}\nEspecie: {}\nTipo: {}\nImagen: {}\nAparece en {} {}\n",
                character.name,
                character.status,
                character.species,
                character.character_type,
                character.image,
                episode_count,
                if episode_count == 1 { "episodio" } else { "episodios" },
            );

            match get_episode_names(&character.episode).await {
                Ok(episode_names) => {
                    message.push_str("\nEpisodios:\n");
                    for(i, name) in episode_names.iter().enumerate() {
                        message.push_str(&format!("{}. {}\n", i + 1, name));
                    }
                },
                Err(_) => {
                    message.push_str("\nNo se pudieron cargar los nombres de los episodios.");
                }
            }
            bot.send_message(chat_id, message).await?;
        }
        Err(e) => {
            bot.send_message(chat_id, format!("Error: {}", e)).await?;
        }
    }
    Ok(())
}
