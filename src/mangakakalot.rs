use regex::Regex;
use reqwest::header;
use std::{fs, io::Write};

use crate::{
    globals::Globals,
    util::{supported_site, Color},
};

pub(crate) async fn downloader(url: &str, skip: u32) -> std::io::Result<()> {
    let g = Globals::new();
    let c = Color::new();
    let re = Regex::new(r"https://(mangakakalot).com/[a-zA-Z0-9_-]+").unwrap();

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

    let tmp: String;
    if url.split("/").collect::<Vec<&str>>()[3] == "manga" {
        tmp = format!(
            r#""https://{}/chapter/{}/chapter_[0-9]*\.?[0-9]?""#,
            site_name,
            url.split("/").collect::<Vec<&str>>()[4]
        );
    } else {
        tmp = format!(
            r#""https://{}/chapter/{}/chapter_[0-9]*\.?[0-9]?""#,
            site_name,
            title.to_lowercase()
        );
    }

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

    for (i, url) in urls.iter().enumerate() {
        if i < skip as usize {
            continue;
        }
        let tmp: String = "_([0-9]+\\.?[0-9]?)".to_string();

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
        mangakakalot_get_imgs(url, &format!("{}/chapter{}", &g.output_dir, chapter)).await;
        tokio::time::sleep(std::time::Duration::from_millis(g.chapter_delay.clone())).await;
    }

    Ok(())
}

pub(crate) async fn mangakakalot_get_imgs(url: &str, path: &str) {
    let g = Globals::new();
    fs::create_dir_all(path).unwrap();

    let res = reqwest::get(url).await;
    let html = res.unwrap().text().await.unwrap();

    let re: Regex = Regex::new(
        r#"<div class="container-chapter-reader">((.|\n)*)<div style="text-align:center;">"#,
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

    // get an image every 500 millis
    let mut i = 0;
    for url in urls.clone() {
        mangakakalot_fetch_img(url, &i.to_string(), &path).await;
        i += 1;
        tokio::time::sleep(std::time::Duration::from_millis(g.img_delay.clone())).await;
    }
}

async fn mangakakalot_fetch_img(url: &str, name: &str, path: &str) {
    let client = reqwest::Client::new();

    // Headers need to be here to trick the server into thinking we are a browser requesting from "https://mangakakalot.com/"
    let res = client
        .get(url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36")
        .header(header::REFERER, "https://mangakakalot.com/")
        .send()
        .await
        .unwrap();

    let num = format!("{:0>3}", name);
    let mut file = fs::File::create(format!("{}/{}.jpg", path, num)).unwrap();
    file.write_all(&res.bytes().await.unwrap()).unwrap();
}
