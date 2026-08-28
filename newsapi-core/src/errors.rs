use thiserror::Error;

use serde_json::Error as SerdeError;
use ureq::Error as UreqError;
use std::env::VarError;

#[derive(Debug, Error)]
pub enum NewsApiError {
    #[error("Error occured while calling the uri.")]
    BadRequest(UreqError),

    #[error("Failed to parse response body to string.")]
    FailedToParseIntoString(UreqError),

    #[error("Failed to parse response body string to JSON")]
    FailedToParseIntoJSON(SerdeError),

    #[error("API_KEY variable not found")]
    APIKeyNotFound(VarError)
}
