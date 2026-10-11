use crate::NewsApiClient;
use crate::NewsApiError;
use crate::NewsApiResponse;
use crate::Result;
use crate::endpoint::Endpoint;

use dotenvy;
use ureq;

use std::env;

pub fn get_api_key(name: impl Into<String>) -> Result<String> {
    let _ = dotenvy::dotenv();

    let name = env::var(name.into()).map_err(|err| NewsApiError::APIKeyNotFound(err));

    return name;
}

impl<S: Endpoint> NewsApiClient<S> {
    /// fetches `NewsApiResponse` using ureq crate.
    ///
    /// if `self.is_built = false`, calls `self.build()` method to update the url.
    pub fn fetch(&self) -> Result<NewsApiResponse> {
        let response: String = ureq::get(self.url())
            .header("Authorization", self.show_api_key())
            .call()
            .map_err(|e| NewsApiError::BadRequest(e))?
            .body_mut()
            .read_to_string()
            .map_err(|e| NewsApiError::FailedToParseIntoString(e))?;

        let articles = serde_json::from_str::<NewsApiResponse>(response.as_str())
            .map_err(|e| NewsApiError::FailedToParseIntoJSON(e))?;

        Ok(articles)
    }
}
