use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use scraper::{Selector, Html};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let article = "article.html";

    let mut output = File::create(article)?;

    let top_html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8">
        <title>Hello, world!</title>
    </head>
    <body>
    "#;

    write!(output, "{}", top_html);

    let body = reqwest::get("https://medium.com/@joydeepubuntu/rust-and-webscraping-ebecc9ae536c").await?
        .text().await?;
    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("p").unwrap();

    for paragraph in fragment.select(&selector) {
        let paragraph_text = paragraph.text().collect::<Vec<_>>();
        write!(output, "<p>{:?}</p>", paragraph_text);
    }

    let last_html = r#"
    </body>
    </html>
    "#;

    write!(output, "{}", last_html);

    Ok(())
}
