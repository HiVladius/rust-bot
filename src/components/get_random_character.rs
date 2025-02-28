use rand::Rng;
use serde::Deserialize;

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
pub struct Character {
    pub name: String,
    pub status: String,
    pub species: String,
    #[serde(rename = "type")]
    pub character_type: String,
    pub image: String,
}

pub async fn get_random_character() -> Result<Character, Box<dyn std::error::Error + Send + Sync>> {
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
