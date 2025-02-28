use teloxide::{
    prelude::{Requester, ResponseResult},
    types::ChatId,
    Bot,
};

use crate::components::get_random_character::get_random_character;

pub async fn send_random_character(bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
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
            bot.send_message(chat_id, format!("Error: {}", e)).await?;
        }
    }
    Ok(())
}
