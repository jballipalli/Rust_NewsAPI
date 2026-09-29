pub mod errors;
pub mod models;
pub mod options;
pub mod parser;

pub use errors::NewsApiError;
pub use models::NewsApiResponse;
pub use options::{Category, Country, Language, SearchIn, SortBy};
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
    use crate::parser::{CategoryTrait, CommonTrait, CountryTrait, LanguageTrait, SourcesTrait};

    use super::*;
    use dotenvy;
    use std::env;
    use url::Url;

    fn api_key() -> String {
        let _ = dotenvy::dotenv();
        env::var("API_KEY").unwrap()
    }

    #[test]
    fn test_evereything() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = NewsApiClient::everything(api_key());
        client.q("rustlang")?.domains("bbc.com")?.build()?;

        let actual: std::collections::HashMap<String, String> = Url::parse(client.get_url())?
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        assert_eq!(actual, *client.get_parameters());
        Ok(())
    }

    #[test]
    fn test_top_headlines() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = NewsApiClient::top_headlines(api_key());
        client
            .country(Country::India)
            .category(Category::Technology)
            .build()?;

        let actual: std::collections::HashMap<String, String> = Url::parse(client.get_url())?
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        assert_eq!(actual, *client.get_parameters());
        Ok(())
    }

    #[test]
    fn test_source() -> Result<(), Box<dyn std::error::Error>> {
        let mut client = NewsApiClient::sources(api_key());
        client
            .language(Language::English)
            .country(Country::Canada)
            .category(Category::Health)
            .build()?;

        let actual: std::collections::HashMap<String, String> = Url::parse(client.get_url())?
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();

        assert_eq!(actual, *client.get_parameters());
        Ok(())
    }

    #[test]
    fn test_source_with_country() -> Result<(), NewsApiError> {
        let mut client = NewsApiClient::top_headlines(api_key());

        let c = client
            .sources(vec!["bbc.com", "techcrunch.com"])
            .unwrap()
            .category(Category::Business)
            .country(Country::Japan)
            .build();

        assert!(matches!(c, Err(NewsApiError::BuildError)));

        Ok(())
    }

    #[test]
    fn test_page_size_error() -> Result<(), NewsApiError> {
        let mut client = NewsApiClient::top_headlines(api_key());

        let result = client.country(Country::Russia).page_size(200);

        assert!(matches!(
            result,
            Err(NewsApiError::ParamError {
                param,
                message
            })
            if param == "pageSize" && message.contains("1 and 100")
        ));

        Ok(())
    }
}
