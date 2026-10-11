use newsapi_core::{NewsApiClient, NewsApiResponse, get_api_key};

use colour::{blue_ln, dark_cyan_ln, dark_red_ln, green_ln_bold, yellow_ln};

fn main() -> Result<(), newsapi_core::NewsApiError> {
    let api_key = get_api_key("API_KEY");

    let client = NewsApiClient::everything(api_key)
        .from("2026-10-10")?
        .q("protest in india")?;
    let response = client.fetch().unwrap();

    render_article(&response);

    println!("{client}");

    Ok(())
}

fn render_article(articles: &NewsApiResponse) {
    for article in articles.get_articles() {
        dark_cyan_ln!("{}", article.title());
        match &article.author() {
            Some(name) => {
                blue_ln!("{}", name)
            }
            None => {
                yellow_ln!("Unknown Author.")
            }
        }
        dark_red_ln!("{}", article.url());
        green_ln_bold!("{}", article.published_at());

        println!("---");
    }
}
