use regex::Regex;
use reqwest::header;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use std::time::Instant;
use std::{fs, io::Write};

use url::Url;

use crate::{
    globals::Globals,
    utils::{color::Color, util::supported_site},
};

pub(crate) async fn downloader(url: Url, skip: u32) -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let g: Globals = Globals::new();
    let c: Color = Color::new();

    let re = Regex::new(r"https://(chapmanganato).com/[a-zA-Z0-9_-]+").unwrap();
    let site_name = url.host_str().unwrap();

    let url_str = url.to_string();

    if !re.is_match(&url_str) {
        println!("{}{}{}", c.red, "Invalid url", c.end);
        return Ok(());
    }

    supported_site(site_name);

    let res = reqwest::get(url).await;
    let text = res.unwrap().text().await.unwrap();
    let html = text.as_str();

    let doc = Document::from(html);
    let title = doc.find(Name("h1")).next().unwrap().text();

    println!("Title: {}{}{}", c.green, title, c.end);

    let chapter_list = doc
        .find(Class("row-content-chapter"))
        .collect::<Vec<Node>>();

    let mut chapter_blocks = chapter_list[0].find(Name("a")).collect::<Vec<Node>>();
    chapter_blocks.reverse();

    println!(
        "Found {}{:?}{} chapters",
        c.green,
        chapter_blocks.len(),
        c.end
    );

    let mut chapter_urls = Vec::new();

    for a in &chapter_blocks {
        let href = a.attr("href").unwrap();
        chapter_urls.push(href);
    }

    if args.contains(&"--list".to_string()) || args.contains(&"-l".to_string()) {
        let name_index = &url_str.split("/").collect::<Vec<&str>>().len();

        crate::handlers::list::list(chapter_urls.clone(), name_index).unwrap();

        return Ok(());
    }

    chapter_urls = crate::handlers::flags::chapter(chapter_urls);
    chapter_urls = crate::handlers::flags::range(chapter_urls);
    chapter_urls = crate::handlers::flags::name(chapter_urls);

    println!(
        "Downloading {}{}{} chapters",
        c.green,
        chapter_urls.len(),
        c.end
    );

    for (i, url) in chapter_urls.iter().enumerate() {
        if i < skip as usize {
            continue;
        }
        let re = Regex::new("-([0-9]+\\.?[0-9]?)").unwrap();
        let chapter = re.find(url).unwrap().as_str();
        println!(
            "\nDownloading Chapter {}{}{} ({}/{}){}",
            c.green,
            &chapter[1..chapter.len()],
            c.yellow,
            i + 1,
            &chapter_urls.len(),
            c.end
        );
        chapmanganato_get_imgs(
            url,
            &format!("{}/{}/chapter{}", &g.output_dir, title, chapter),
        )
        .await;
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }

    Ok(())
}

pub(crate) async fn chapmanganato_get_imgs(url: &str, path: &str) {
    let verbose = std::env::args()
        .collect::<Vec<String>>()
        .iter()
        .any(|arg| arg == "--verbose" || arg == "-v");

    let g: Globals = Globals::new();
    let c = Color::new();
    fs::create_dir_all(path).unwrap();

    let res = reqwest::get(url).await;
    let text = res.unwrap().text().await.unwrap();
    let html = text.as_str();

    let document = Document::from(html);
    for node in document.find(Class("container-chapter-reader")) {
        let start = Instant::now();

        let mut i = 0;

        // get the length of a list of all the imgs
        let imgs = node.find(Name("img")).collect::<Vec<Node>>();

        println!("Found {}{:?}{} images", c.green, imgs.len(), c.end);

        for img in imgs {
            let src = img.attr("src").unwrap();
            chapmanganato_fetch_img(src, &i.to_string(), &path).await;
            i += 1;
            tokio::time::sleep(std::time::Duration::from_millis(g.img_delay.clone())).await;
        }

        if verbose {
            println!(
                "Finished chapter {}{}{} in {}{}{} seconds",
                c.green,
                &url.split("/").collect::<Vec<&str>>()[5],
                c.end,
                c.green,
                start.elapsed().as_secs(),
                c.end
            );
        }
    }
}

async fn chapmanganato_fetch_img(url: &str, name: &str, path: &str) {
    let start = Instant::now();
    let verbose = std::env::args()
        .collect::<Vec<String>>()
        .iter()
        .any(|arg| arg == "--verbose" || arg == "-v");

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
    if verbose {
        let elapsed = start.elapsed();
        // format elapsed as 00.000 seconds.millis
        let elapsed = format!("{:02}.{:03}", elapsed.as_secs(), elapsed.subsec_millis());

        println!(
            "Downloaded image {}{}{} in {}{} s{}",
            Color::new().green,
            num,
            Color::new().end,
            Color::new().green,
            elapsed,
            Color::new().end
        );
    }
}
