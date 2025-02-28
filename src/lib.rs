


pub mod components{
    pub mod get_random_character;
    pub mod send_random_characters;
    pub mod command_handler;
    pub mod callback_handler;
}

pub mod lazy_chat_ids{
    use lazy_static::lazy_static;
    use tokio::sync::Mutex;
use std::collections::HashSet;
    lazy_static! {
        pub static ref CHAT_IDS: Mutex<HashSet<i64>> = Mutex::new(HashSet::new());
    }
}