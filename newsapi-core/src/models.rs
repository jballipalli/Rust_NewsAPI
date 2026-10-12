use serde::Deserialize;
use serde_with;
use std::fmt;

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct NewsApiResponse {
    pub status: String,

    #[serde(rename = "totalResults")]
    pub total_results: Option<usize>,

    pub articles: Option<Vec<Article>>,

    pub sources: Option<Vec<Source>>,
}

impl fmt::Display for NewsApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:#?}", self)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub source: Option<Source>,

    pub author: Option<String>,

    pub title: Option<String>,

    pub description: Option<String>,

    pub url: Option<String>,

    pub url_to_image: Option<String>,

    pub published_at: Option<String>,

    pub content: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct Source {
    pub id: Option<String>,

    pub name: Option<String>,

    pub description: Option<String>,

    pub url: Option<String>,

    pub category: Option<String>,

    pub language: Option<String>,

    pub country: Option<String>,
}

#[derive(Debug)]
pub struct ResponseError {
    pub id: String,

    pub code: String,

    pub message: String,
}
