use anyhow::anyhow;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let html = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    let document = Html::parse_document(&html);

    let selector = Selector::parse("body > main > header > div > div.w-30-l.flex-column.pl0-l.pr0-l.pl3.pr3 > p > a").unwrap();

    for element in document.select(&selector) {
        println!("{}", element.inner_html());
    }
    Ok(())
}
