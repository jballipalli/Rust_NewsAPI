use url;

pub struct UrlBuilder {
    api_key: String,
    base_url: String,
}

impl UrlBuilder {
    fn new(api_key: String, base_url: String) -> Self {
        Self { api_key, base_url }
    }
}
