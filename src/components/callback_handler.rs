use teloxide::{ prelude::{Requester, ResponseResult}, Bot};
use teloxide::types::CallbackQuery;
use crate::components::send_random_characters::send_random_character;
use crate::lazy_chat_ids::CHAT_IDS;

pub async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    // Manejo de MaybeInaccessibleMessage corregido
    let chat_id = if let Some(message) = q.message {
        match message {
            teloxide::types::MaybeInaccessibleMessage::Regular(msg) => Some(msg.chat.id),
            teloxide::types::MaybeInaccessibleMessage::Inaccessible(_) => None,
        }
    } else {
        None
    };

    if let (Some(chat_id), Some(cb_data)) = (chat_id, q.data) {
        match cb_data.as_str() {
            "random_character" => {
                // Notificar que el bot está procesando
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
    } else {
        // Simplemente responde a la callback query sin mensaje
        bot.answer_callback_query(q.id).await?;
    }
    
    Ok(())
}