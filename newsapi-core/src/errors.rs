use thiserror::Error;

use serde_json::Error as SerdeError;
use std::env::VarError;
use ureq::Error as UreqError;

#[derive(Debug, Error)]
pub enum NewsApiError {
    #[error("Error occured while calling the uri.")]
    BadRequest(UreqError),

    #[error("Failed to parse response body to string.")]
    FailedToParseIntoString(UreqError),

    #[error("Failed to parse response body string to JSON")]
    FailedToParseIntoJSON(SerdeError),

    #[error("API_KEY variable not found")]
    APIKeyNotFound(VarError),

    #[error("Parameter '{param}' error: {message}")]
    ParamError { param: String, message: String },

    #[error("Parameters country or category used along with sources. Please use sources endpoint.")]
    BuildError,
}
