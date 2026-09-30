use newsapi_core::{Country, NewsApiClient, NewsApiResponse, parser::CountryTrait};

use dotenvy;
use std::env;

use colour::{blue_ln, dark_cyan_ln, dark_red_ln, green_ln_bold, yellow_ln};

fn main() -> Result<(), newsapi_core::NewsApiError> {
    let _ = dotenvy::dotenv();

    let api_key = env::var("API_KEY").map_err(|e| newsapi_core::NewsApiError::APIKeyNotFound(e))?;

    let mut client = NewsApiClient::top_headlines(api_key);
    let response = client.country(Country::UnitedStates).fetch().unwrap();

    render_article(&response);

    println!("{client}");

    Ok(())
}

fn render_article(articles: &NewsApiResponse) {
    for article in articles.get_articles() {
        dark_cyan_ln!("{}", article.title());
        match &article.author() {
            Some(name) => {
                blue_ln!("{}", name)
            }
            None => {
                yellow_ln!("Unknown Author.")
            }
        }
        dark_red_ln!("{}", article.url());
        green_ln_bold!("{}", article.published_at());

        println!("---");
    }
}
