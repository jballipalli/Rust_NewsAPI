use newsapi_core::{Country, NewsApiClient, parser::CountryTrait};

use dotenvy;
use std::env;

fn main() -> Result<(), newsapi_core::NewsApiError> {
    let _ = dotenvy::dotenv();

    let api_key = env::var("API_KEY").map_err(|e| newsapi_core::NewsApiError::APIKeyNotFound(e))?;

    let mut client = NewsApiClient::top_headlines(api_key);
    let response = client.country(Country::UnitedStates).fetch().unwrap();

    newsapi_core::render_article(&response);

    Ok(())
}
