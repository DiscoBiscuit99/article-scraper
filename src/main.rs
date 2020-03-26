use std::fs::File; 
use std::io::{Write, BufReader, BufRead, Error}; 
use std::env;
use scraper::{Selector, Html}; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let folder = "./article";

    let article = format!("{}/article.html", folder);
    let style = format!("{}/article.css", folder);

    let mut html = File::create(article)?; 
    let mut css = File::create(style)?; 

    // HTML
    let top_html = r#"<!DOCTYPE html>
<html>
<head>
     <meta charset="utf-8">
     <link rel="stylesheet" type="text/css" href="article.css">
     <title>Hello, world!</title>
</head>
<body>
     <div class="div-block">
"#;

    write!(html, "{}", top_html);

    let article_url = get_url(env::args().collect());

    let body = reqwest::get(&article_url)
        .await?.text().await?;

    let fragment = Html::parse_document(&body);

    let h1_selector = Selector::parse("h1").unwrap();
    let p_selector = Selector::parse("p").unwrap();

    let img_selector = Selector::parse("img").unwrap();

    for img in fragment.select(&img_selector) {
        let img_url = img.text().collect::<Vec<_>>();
        for url in img_url.iter() {
            write!(html, "\t\t<img src=\"{}\">\n", url);
        }
    }

    for h1 in fragment.select(&h1_selector) {
        let h1_raw = h1.text().collect::<Vec<_>>(); 
        for text in h1_raw.iter() {
            write!(html, "\t\t<h1>{}</h1>\n", text);
        }
    }

    for text in fragment.select(&p_selector){
        let text_raw = text.text().collect::<Vec<_>>(); 
        for text in text_raw.iter() {
            write!(html, "\t\t<p>{}</p>\n", text); 
        }
        //write!(html, "\t<p>{:?}</p>\n", text_raw); 
    }

    let end_html = r#"
     </div>
</body>
</html>
"#;
    write!(html, "{}", end_html);

    // CSS
    let styling_info = r#"html, body {
    background-color: #2d2f2e;
    color: #fffff9;

    font-family: Times New Roman;

    margin: 0;
}

.div-block {
    width: 50%;
    margin: auto;
    margin-bottom: 3em;
}

.div-block h1 {
    text-align: center;
    color: #ff926e;
    margin: 2em;
}

.div-block p {
    font-size: 16px;
    text-align: justify;
}
"#;

    write!(css, "{}", styling_info);

    Ok(())
}

fn get_url(args: Vec<String>) -> String {
    args[1].clone()
}
