use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewsApiResponse {
    articles: Vec<Article>,
}

impl NewsApiResponse {
    pub fn get_articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

#[derive(Debug, Deserialize)]
pub struct Article {
    title: String,
    #[serde(rename = "publishedAt")]
    published_at: String,
    url: String,
    author: Option<String>,
}

impl Article {
    pub fn title(&self) -> &str {
        &self.title.as_str()
    }

    pub fn published_at(&self) -> &str {
        &self.published_at.as_str()
    }

    pub fn url(&self) -> &str {
        &self.url.as_str()
    }

    pub fn author(&self) -> &Option<String> {
        &self.author
    }
}
