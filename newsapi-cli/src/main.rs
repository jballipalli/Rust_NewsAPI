use newsapi_core;

use dotenvy;
use std::env;

fn main() -> Result<(), newsapi_core::NewsApiError> {
    let _ = dotenvy::dotenv();

    let api_key = env::var("API_KEY").map_err(|e| newsapi_core::NewsApiError::APIKeyNotFound(e))?;
    let url = String::from("https://newsapi.org/v2/top-headlines?country=us");

    println!("{url}&apiKey={api_key}");

    let arti = newsapi_core::get_news(&format!("{}&apiKey={}", url, api_key))?;

    newsapi_core::render_article(&arti);

    Ok(())
}
