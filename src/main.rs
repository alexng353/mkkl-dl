use regex::Regex;
use std::fs;
use tokio;

// object with ansii color codes
struct Color {
    red: &'static str,
    green: &'static str,
    yellow: &'static str,
    blue: &'static str,
    magenta: &'static str,
    cyan: &'static str,
    end: &'static str,
}

impl Color {
    fn new() -> Color {
        Color {
            red: "\x1b[31m",
            green: "\x1b[32m",
            yellow: "\x1b[33m",
            blue: "\x1b[34m",
            magenta: "\x1b[35m",
            cyan: "\x1b[36m",
            end: "\x1b[0m",
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let c = Color::new();

    let res = reqwest::get("https://mangakakalot.com/read-wg9rm158504883358").await;
    let html = res.unwrap().text().await.unwrap();

    // regex for <span itemprop="name">something</span>
    let re = Regex::new(r#"<h1>(.*)</h1>"#).unwrap();
    let title = re.captures(&html).unwrap().get(1).unwrap().as_str();

    println!("Title: {}{}{}", c.green, title, c.end);

    // exit
    // return Ok(());
    let tmp = format!(
        r#""https://mangakakalot.com/chapter/{}/chapter_[0-9]*\.?[0-9]?""#,
        title.to_lowercase()
    );

    let re = Regex::new(&tmp).unwrap();

    // grab the first link, seperate by / and grab the second last element
    let matches = re.find_iter(&html);

    let mut urls = Vec::new();
    for m in matches {
        urls.push(&m.as_str()[1..m.as_str().len() - 1]);
    }

    println!(
        "Found {}{:?}{} chapters",
        c.green,
        urls.clone().len(),
        c.end
    );

    // grab img with the last url
    // let res = reqwest::get(urls[0]).await;
    // println!("{}", urls[urls.clone().len() - 1]);
    grab_img(&urls[urls.clone().len() - 1]).await;

    Ok(())
}

async fn grab_img(url: &str) {
    let res = reqwest::get(url).await;
    let html = res.unwrap().text().await.unwrap();
    // regex for <div class="container-chapter-reader">{anything}</div>
    let re = Regex::new(
        r#"<div class="container-chapter-reader">((.|\n)*)<div style="text-align:center;">"#,
    )
    .unwrap();
    // make it single line
    // add /s flag

    let html = re.captures(&html).unwrap().get(1).unwrap().as_str();

    // let html = re.captures(&res).unwrap();
    println!("{:?}", html);
}
