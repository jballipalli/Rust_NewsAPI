use newsapi_core;

fn main() -> Result<(), newsapi_core::NewsApiError> {
    let api_key = String::from("681eb5fff60b4a5985ccb89a6ba8d4eb");
    let url = String::from("https://newsapi.org/v2/top-headlines?country=us");

    println!("{}", format!("{}&apiKey={}", url, api_key));

    let arti = newsapi_core::get_news(&format!("{}&apiKey={}", url, api_key))?;

    newsapi_core::render_article(&arti);

    Ok(())
}
