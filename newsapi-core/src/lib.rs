mod errors;

pub use errors::NewsApiError;

use serde::Deserialize;
use ureq;

use colour::{blue_ln, dark_cyan_ln, dark_red_ln, green_ln_bold, yellow_ln};

#[derive(Debug, Deserialize)]
pub struct Articles {
    pub articles: Vec<Article>,
}

#[derive(Debug, Deserialize)]
pub struct Article {
    pub title: String,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    pub url: String,
    pub author: Option<String>,
}

pub fn get_news(uri: &str) -> Result<Articles, NewsApiError> {
    let response: String = ureq::get(uri)
        .call()
        .map_err(|e| NewsApiError::BadRequest(e))?
        .body_mut()
        .read_to_string()
        .map_err(|e| NewsApiError::FailedToParseIntoString(e))?;

    let articles = serde_json::from_str::<Articles>(response.as_str())
        .map_err(|e| NewsApiError::FailedToParseIntoJSON(e))?;

    Ok(articles)
}

pub fn render_article(articles: &Articles) {
    for article in &articles.articles {
        dark_cyan_ln!("{}", article.title);
        match &article.author {
            Some(name) => {
                blue_ln!("{}", name)
            }
            None => {
                yellow_ln!("Unknown Author. Open link to check")
            }
        }
        dark_red_ln!("{}", article.url);
        green_ln_bold!("{}", article.published_at);

        println!("---");
    }
}
