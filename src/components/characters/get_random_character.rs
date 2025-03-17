use rand::Rng;

use crate::Character;
use crate::ApiResponse;

// Helper function to fetch all characters recursively
fn fetch_all_characters<'a>(
    client: &'a reqwest::Client,
    url: &'a str,
    characters: Vec<Character>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Character>, Box<dyn std::error::Error + Send + Sync>>> + Send + 'a>> {
    Box::pin(async move {
        let response = client.get(url).send().await?;
        let api_response: ApiResponse = response.json().await?;
        
        let mut updated_characters = characters;
        updated_characters.extend(api_response.results);
        
        match api_response.info.next {
            Some(next_url) => fetch_all_characters(client, &next_url, updated_characters).await,
            None => Ok(updated_characters),
        }
    })
}

pub async fn get_random_character(
) -> Result<(Character, usize), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let characters = fetch_all_characters(
        &client,
        "https://rickandmortyapi.com/api/character",
        Vec::new()
    ).await?;

    if characters.is_empty() {
        return Err("No se encontraron personajes".into());
    }

    let mut rng = rand::rng();
    let index = rng.random_range(0..characters.len());
    let character = characters[index].clone();
    let episode_count = character.episode.len();
    Ok((character, episode_count))
}
