use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Episode information from Rick and Morty API
#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub id: i32,
    pub name: String,
    pub air_date: String,
    pub episode: String,
    pub characters: Vec<String>,
    pub url: String,
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EpisodeResponse {
    info: Info,
    results: Vec<Episode>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    count: i32,
    pages: i32,
    next: Option<String>,
    prev: Option<String>,
}

/// Fetches episodes from the first page of the Rick and Morty API
pub async fn get_episodes() -> Result<Vec<Episode>, Box<dyn Error>> {
    let url = "https://rickandmortyapi.com/api/episode/";
    let response = reqwest::get(url).await?.json::<EpisodeResponse>().await?;
    
    Ok(response.results)
}

/// Fetches episode details by URL
pub async fn get_episode_by_url(url: &str) -> Result<Episode, Box<dyn Error>> {
    let response = reqwest::get(url).await?.json::<Episode>().await?;
    Ok(response)
}

/// Fetches episode names for a list of episode URLs
pub async fn get_episode_names(episode_urls: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
    let mut episode_names = Vec::new();
    
    for url in episode_urls {
        match get_episode_by_url(url).await {
            Ok(episode) => episode_names.push(format!("{} ({})", episode.name, episode.episode)),
            Err(_) => episode_names.push("Unknown episode".to_string()),
        }
    }
    
    Ok(episode_names)
}
