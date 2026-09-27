pub mod errors;
pub mod models;
pub mod options;
pub mod parser;

pub use errors::NewsApiError;
pub use models::NewsApiResponse;
pub use options::{Category, Country, Language, SearchIn};
pub use parser::NewsApiClient;

use ureq;

use colour::{blue_ln, dark_cyan_ln, dark_red_ln, green_ln_bold, yellow_ln};

pub fn get_news(uri: &str) -> Result<NewsApiResponse, NewsApiError> {
    let response: String = ureq::get(uri)
        .call()
        .map_err(|e| NewsApiError::BadRequest(e))?
        .body_mut()
        .read_to_string()
        .map_err(|e| NewsApiError::FailedToParseIntoString(e))?;

    let articles = serde_json::from_str::<NewsApiResponse>(response.as_str())
        .map_err(|e| NewsApiError::FailedToParseIntoJSON(e))?;

    Ok(articles)
}

pub fn render_article(articles: &NewsApiResponse) {
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

#[cfg(test)]
mod tests {
    use crate::parser::CommonTrait;

    use super::*;
    use dotenvy;
    use std::env;

    fn api_key() -> String {
        let _ = dotenvy::dotenv();
        env::var("API_KEY").unwrap()
    }

    #[test]
    fn test_evereything() -> Result<(), NewsApiError> {
        let mut client = NewsApiClient::everything(api_key());
        client.q("rustlang")?.domains("bbc.com")?.build();

        assert_eq!(client.get_url(), "https://newsapi.org/v2/everything");
        Ok(())
    }

    #[test]
    fn test_top_headlines() {
        let mut client = NewsApiClient::top_headlines(api_key());
        client.build();

        assert_eq!(client.get_url(), "https://newsapi.org/v2/top-headlines")
    }

    #[test]
    fn test_source() {
        let mut client = NewsApiClient::sources(api_key());
        client.build();

        assert_eq!(
            client.get_url(),
            "https://newsapi.org/v2/top-headlines/sources"
        )
    }
}
