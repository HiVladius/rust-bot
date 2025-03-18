use crate::ApiResponse;
use crate::Character;

fn fetch_all_characters<'a>(client: &'a reqwest::Client, url: &'a str, characters: Vec<Character>) -> std::pin::Pin<
    Box<
        dyn std::future::Future<
                Output = Result<Vec<Character>, Box<dyn std::error::Error + Send + Sync>>,
            > + Send
            + 'a,
    >,
> {
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

// Maximum number of characters to display in a single message
const MAX_DISPLAYED_CHARACTERS: usize = 20;

pub async fn found_character(name: &str,) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    
    let client = reqwest::Client::new();

    // Use the API endpoint with only name query parameter
    let url = format!("https://rickandmortyapi.com/api/character/?name={}", name);

    let characters = fetch_all_characters(&client, &url, Vec::new()).await?;

    if characters.is_empty() {
        Ok(format!(
            "No se encontraron personajes con el nombre '{}'",
            name
        ))
    } else {
        let total_characters = characters.len();
        let characters_to_display = std::cmp::min(total_characters, MAX_DISPLAYED_CHARACTERS);

        let mut message = format!(
            "Encontrados {} personajes que coinciden con '{}':\n\n",
            total_characters, name
        );

        for (_i, character) in characters.iter().enumerate().take(characters_to_display) {
           
            message.push_str(&format!(
                "📌 *{}*\n• Estado: {}\n\n",
                character.name, character.status
            ));
        }

        // Add a note if we're not displaying all characters
        if total_characters > MAX_DISPLAYED_CHARACTERS {
            message.push_str(&format!(
                "ℹ️ *Mostrando {} de {} personajes encontrados*",
                characters_to_display, total_characters
            ));

            
        }

        Ok(message)
    }
}
