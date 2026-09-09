use url::{ParseError, Url};

static BASE_URL: &str = "https://newsapi.org/v2/";

pub struct NewsApiClient {
    api_key: String,

    url: Url,
}

impl NewsApiClient {
    pub fn everything(api_key: String) -> Result<Self, ParseError> {
        let base = Url::parse(BASE_URL).unwrap();
        let url = base.join("everything")?;

        Ok(Self { api_key, url })
    }

    pub fn top_headlines(api_key: String) -> Result<Self, ParseError> {
        let base = Url::parse(BASE_URL)?;
        let url = base.join("top-headlines")?;

        Ok(Self { api_key, url })
    }

    pub fn show_url(&self) -> &Url {
        &self.url
    }

    pub fn show_api_key(&self) -> &str {
        &self.api_key
    }
}
