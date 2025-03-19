use crate::components::characters::get_random_character::get_random_character;

use teloxide::{prelude::*, types::ChatId, Bot};
use crate::Character;

use std::collections::HashMap;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

struct TriviaState{
    character: Character,
    score: i32,
    question_index:usize,
}


lazy_static! {
    static ref TRIVIA_STATES: Mutex<HashMap<ChatId, TriviaState>> = Mutex::new(HashMap::new());
}

fn generate_question(character: &Character, question_index: usize) -> String{
    match question_index{
        0 => format!("¿Esta vivo {}?, Si o No", character.name),
        1 => format!("¿Cual es la especie de {}?", character.name),
        2 => format!("¿Cual es el tipo de {}?", character.name),
        _ => "Fin del juego".to_string(),
    }
}

fn check_answer(state: &TriviaState, answer: &str) -> bool {
    let correct_answer = match state.question_index {
        0 => if state.character.status == "Alive" {"si"} else {"no"},
        1 => state.character.character_type.as_str(),
        2 => state.character.species.as_str(),
        _ => "",
    };
    answer.trim().to_lowercase() == correct_answer.to_lowercase()
}
 
pub async fn trivia_game(bot: &Bot, chat_id: ChatId) -> ResponseResult<()> {
    match get_random_character().await {
        Ok((character, _)) => {
            let state = TriviaState {
                character,
                score: 0,
                question_index: 0,
            };      
            
            let question = generate_question(&state.character, state.question_index);
            bot.send_message(chat_id, &question).await?;
            


            TRIVIA_STATES.lock().await.insert(chat_id, state); // Store the state
        }
        Err(e) => {
            bot.send_message(chat_id, format!("Error al obtener el personaje para la trivia: {}", e)).await?;
        }
    }
    Ok(())
}

pub async fn process_trivia_answer(bot: Bot, msg: Message) -> ResponseResult<()>{
    let chat_id = msg.chat.id;
    let answer = msg.text().unwrap_or_default();

    let mut states = TRIVIA_STATES.lock().await;

    if let Some(mut state) = states.remove(&chat_id){
        if check_answer(&state, answer){
            state.score += 5;
            bot.send_message(chat_id, "Respuesta correcta!").await?;
        }else{
            bot.send_message(chat_id, "Respuesta incorrecta!").await?;
        }

        state.question_index += 1;

        if state.question_index < 3 {
            let question = generate_question(&state.character, state.question_index);
            bot.send_message(chat_id, &question).await?;
            states.insert(chat_id, state);
        }else{
            let final_message = format!("Fin del juego! \n\n Tu puntaje es: {} puntos", state.score);
            bot.send_message(chat_id, &final_message).await?;
        }

    }
    Ok(())
}