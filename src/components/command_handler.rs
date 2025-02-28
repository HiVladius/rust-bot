use teloxide::{
    payloads::SendMessageSetters,
    prelude::{Requester, ResponseResult},
    types::Message,
    Bot,
    utils::command::BotCommands,
};

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::lazy_chat_ids::CHAT_IDS;
use crate::components::send_random_characters::send_random_character;


#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "Iniciar el bot")]
    Start,
    #[command(description = "Mostrar menú principal")]
    Menu,
    #[command(description = "Obtener un personaje aleatorio")]
    Random,
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
    }
    Ok(())
}
