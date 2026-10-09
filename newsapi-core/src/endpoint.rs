pub struct Everything;
pub struct TopHeadlines;
pub struct Sources;

pub trait Endpoint {
    const PATH: &'static str;
    const BASE_URL: &'static str = "https://newsapi.org/v2/";
}

impl Endpoint for Everything {
    const PATH: &'static str = "everything";
}

impl Endpoint for TopHeadlines {
    const PATH: &'static str = "top-headlines";
}

impl Endpoint for Sources {
    const PATH: &'static str = "top-headlines/sources";
}
