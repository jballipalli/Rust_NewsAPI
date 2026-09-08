pub mod errors;
pub mod models;

pub use errors::NewsApiError;
pub use models::NewsApiResponse;

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
