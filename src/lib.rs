pub mod components {
    pub mod characters {
        pub mod callback_handler;
        pub mod command_handler;
        pub mod get_random_character;
        pub mod send_episode;
        pub mod send_random_characters;
    }
    pub mod search {
        pub mod character_search;
    }
    pub mod game {
        pub mod trivia_game;
    }
}

pub mod lazy_chat_ids {
    use lazy_static::lazy_static;
    use std::collections::HashSet;
    use tokio::sync::Mutex;
    lazy_static! {
        pub static ref CHAT_IDS: Mutex<HashSet<i64>> = Mutex::new(HashSet::new());
    }
}

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub status: String,
    pub species: String,
    pub episode: Vec<String>, // Ensure this field is correctly deserialized
    #[serde(rename = "type")]
    pub character_type: String,
    pub image: String,
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
