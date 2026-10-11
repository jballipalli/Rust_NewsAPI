use serde::Deserialize;
use serde_with;
use std::fmt;

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct NewsApiResponse {
    status: String,

    #[serde(rename = "totalResults")]
    total_results: Option<usize>,

    articles: Option<Vec<Article>>,

    sources: Option<Vec<Source>>,
}

impl fmt::Display for NewsApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:#?}", self)?;
        Ok(())
    }
}

impl NewsApiResponse {
    pub fn get_status(&self) -> &str {
        self.status.as_ref()
    }

    pub fn get_total_result(&self) -> Option<&usize> {
        self.total_results.as_ref()
    }

    pub fn get_articles(&self) -> Option<&Vec<Article>> {
        self.articles.as_ref()
    }

    pub fn get_sources(&self) -> Option<&Vec<Source>> {
        self.sources.as_ref()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct Article {
    source: Option<Source>,

    author: Option<String>,

    title: Option<String>,

    description: Option<String>,

    url: Option<String>,

    #[serde(rename = "urlToImage")]
    url_to_image: Option<String>,

    #[serde(rename = "publishedAt")]
    published_at: Option<String>,

    content: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct Source {
    id: Option<String>,

    name: Option<String>,

    description: Option<String>,

    url: Option<String>,

    category: Option<String>,

    language: Option<String>,

    country: Option<String>,
}
