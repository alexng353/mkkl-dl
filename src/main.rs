use regex::Regex;
use reqwest::header;
use std::{fs, io::Write};
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
    // get cmd line args
    let c = Color::new();

    let args: Vec<String> = std::env::args().collect();
    // if no length, print no length
    if args.len() == 1 {
        // println!(c.red + "No args given" + c.end);
        println!("{}{}{}", c.red, "No args given", c.end);
        return Ok(());
    }
    let url = &args[1];

    // let res = reqwest::get("https://mangakakalot.com/read-wg9rm158504883358").await; // this is the test url (that works 100%)
    let res = reqwest::get(url).await;
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
    // get_imgs(&urls[urls.clone().len() - 1], "./output/chapter_1").await;

    // get imgs for all chapters but wait 30 seconds between each chapter

    // reverse urls
    urls.reverse();

    // print all urls
    // for url in urls.clone() {
    //     // regex for chapter_{number}?.?{number}

    //     let re = Regex::new(r#"_([0-9]+\.?[0-9]?)"#).unwrap();
    //     let chapter = re.find(url).unwrap().as_str();

    //     println!("Chapter: {}{}{}", c.green, chapter, c.end);
    //     // println!("{}", url);
    // }

    for (i, url) in urls.iter().enumerate() {
        println!("Downloading chapter {}{}{}", c.green, i + 1, c.end);

        let re = Regex::new(r#"_([0-9]+\.?[0-9]?)"#).unwrap();
        let chapter = re.find(url).unwrap().as_str();

        get_imgs(url, &format!("./output/chapter{}", chapter)).await;
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    // test().await;
    Ok(())
}

async fn get_imgs(url: &str, path: &str) {
    // make folder path
    fs::create_dir_all(path).unwrap();

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

    let re = Regex::new(r#"<img src="([^"]*)"#).unwrap();

    // let re = Regex::new(r#"<img src="(.*)" alt="Devlichi (.*)" title="Devilchi (.*)" />"#).unwrap();

    let matches = re.find_iter(&html);

    let mut urls = Vec::new();
    for m in matches {
        urls.push(&m.as_str()[10..m.as_str().len()]);
    }

    // print all urls
    // for url in urls.clone() {
    //     println!("{}", url);
    // }

    // get an image every 1000 millis
    println!(
        "Found {}{:?}{} images",
        Color::new().green,
        urls.clone().len(),
        Color::new().end
    );

    let mut i = 0;
    for url in urls.clone() {
        // let res = reqwest::get(url).await;
        // let img = res.unwrap().bytes().await.unwrap();
        // let mut file = fs::File::create(format!("{}.jpg", i)).unwrap();
        // file.write_all(&img).unwrap();
        fetch_img(url, &i.to_string(), &path).await;
        i += 1;
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    // write html to file
    // fs::write("test.html", html).expect("Unable to write file");
}

async fn fetch_img(url: &str, name: &str, path: &str) {
    let client = reqwest::Client::new();

    let res = client
        .get(url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36")
        .header(header::REFERER, "https://mangakakalot.com/")
        .send()
        .await
        .unwrap();
    // let mut new = name.to_string();
    // while new.len() < 3 {
    //     new = format!("0{}", name);
    // }
    let num = format!("{:0>3}", name);
    let mut file = fs::File::create(format!("{}/{}.jpg", path, num)).unwrap();
    file.write_all(&res.bytes().await.unwrap()).unwrap();

    // println!("done");
}
