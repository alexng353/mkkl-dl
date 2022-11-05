use regex::Regex;
use reqwest::header;
use std::{fs, io::Write};

use crate::{
    globals::{CHAPTER_DELAY, IMG_DELAY, OUTPUT_DIR},
    util::{supported_site, Color},
};

pub(crate) async fn downloader(url: &str, skip: u32) -> std::io::Result<()> {
    let c: Color = Color::new();
    // print all args
    // regex for https://chapmanganato.com/manga-<name>

    let re = Regex::new(r"https://(chapmanganato).com/[a-zA-Z0-9_-]+").unwrap();

    let url_parts: Vec<&str> = url.split('/').collect();

    let site_name = url_parts[2];

    supported_site(site_name);

    if !re.is_match(url) {
        println!("{}{}{}", c.red, "Invalid url", c.end);
        return Ok(());
    }

    let res = reqwest::get(url).await;
    let html = res.unwrap().text().await.unwrap();

    let re = Regex::new(r#"<h1>(.*)</h1>"#).unwrap();
    let title = re.captures(&html).unwrap().get(1).unwrap().as_str();

    println!("Title: {}{}{}", c.green, title, c.end);

    // write the html to file test.html

    let mut file = fs::File::create("test.html")?;
    file.write_all(html.as_bytes())?;

    let tmp: String;

    // https://chapmanganato.com/manga-oa952283/chapter-1 < regex

    tmp = format!(
        r#""https://{}/{}/chapter-[0-9]*\.?[0-9]?""#,
        site_name,
        url.split("/").collect::<Vec<&str>>()[3]
    );

    let re = Regex::new(&tmp).unwrap();
    let matches = re.find_iter(&html);

    let mut urls = Vec::new();
    for m in matches {
        urls.push(&m.as_str()[1..m.as_str().len() - 1]);
    }
    urls.reverse();

    println!(
        "Found {}{:?}{} chapters",
        c.green,
        urls.clone().len(),
        c.end
    );

    // chapmanganato_get_imgs(urls[0], &format!("{}/chapter{}", &OUTPUT_DIR, 0)).await;

    for (i, url) in urls.iter().enumerate() {
        if i < skip as usize {
            continue;
        }
        let tmp: String = "-([0-9]+\\.?[0-9]?)".to_string();

        let re = Regex::new(&tmp).unwrap();
        let chapter = re.find(url).unwrap().as_str();
        println!(
            "\nDownloading Chapter {}{}{} ({}/{}){}",
            c.green,
            &chapter[1..chapter.len()],
            c.yellow,
            i + 1,
            urls.len(),
            c.end
        );
        chapmanganato_get_imgs(url, &format!("{}/chapter{}", &OUTPUT_DIR, chapter)).await;
        tokio::time::sleep(std::time::Duration::from_millis(CHAPTER_DELAY.clone())).await;
    }

    println!("{}{:?}{}", c.green, urls, c.end);

    Ok(())
}

pub(crate) async fn chapmanganato_get_imgs(url: &str, path: &str) {
    fs::create_dir_all(path).unwrap();

    let res = reqwest::get(url).await;
    let html = res.unwrap().text().await.unwrap();

    let re: Regex = Regex::new(
        r#"<div class="container-chapter-reader">((.|\n)*)<div style="max-height: 380px; text-align: center; width: 810px; margin: 10px auto; overflow: hidden; max-width: 100%;">"#,
    )
    .unwrap();

    let html = re.captures(&html).unwrap().get(1).unwrap().as_str();

    let re = Regex::new(r#"<img src="([^"]*)"#).unwrap();

    let matches = re.find_iter(&html);

    let mut urls = Vec::new();
    for m in matches {
        urls.push(&m.as_str()[10..m.as_str().len()]);
    }

    println!(
        "Found {}{:?}{} images",
        Color::new().green,
        urls.clone().len(),
        Color::new().end
    );
    let mut i = 0;

    for url in urls.clone() {
        chapmanganato_fetch_img(url, &i.to_string(), &path).await;
        i += 1;
        tokio::time::sleep(std::time::Duration::from_millis(IMG_DELAY.clone())).await;
    }
}

async fn chapmanganato_fetch_img(url: &str, name: &str, path: &str) {
    let client = reqwest::Client::new();

    // Headers need to be here to trick the server into thinking we are a browser requesting from "https://mangakakalot.com/"
    let res = client
        .get(url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36")
        .header(header::REFERER, "referer: https://chapmanganato.com/")
        .send()
        .await
        .unwrap();

    let num = format!("{:0>3}", name);
    let mut file = fs::File::create(format!("{}/{}.jpg", path, num)).unwrap();
    file.write_all(&res.bytes().await.unwrap()).unwrap();
}
