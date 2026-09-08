use std::string::ToString;
use url;

pub struct UrlBuilder {
    api_key: String,
    base_url: String,
}

pub enum Endpoint {
    Topheadlines,
    Everything,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Endpoint::Everything => String::from("everything"),
            Endpoint::Topheadlines => String::from("top-headlines"),
        }
    }
}

pub enum Country {
    US,
    Canada,
    India,
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Country::Canada => String::from("ca"),
            Country::India => String::from("in"),
            Country::US => String::from("us"),
        }
    }
}

impl UrlBuilder {
    fn new(api_key: String, base_url: String) -> Self {
        Self { api_key, base_url }
    }
}
