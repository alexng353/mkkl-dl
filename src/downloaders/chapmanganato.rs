use anyhow::Ok;
use anyhow::Result;
use colored::Colorize;
use regex::Regex;
use reqwest::header;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use std::time::Instant;
use std::{fs, io::Write};
use url::Url;

use crate::{globals::Globals, utils::util::supported_site};

use super::downloaders::Args;

pub(crate) async fn chapmanganato(args: Args) -> Result<()> {
    let g = Globals::new();

    let url = Url::parse(&args.url).unwrap();

    let site_name = url.host_str().unwrap();
    supported_site(site_name);

    let res = reqwest::get(url).await.unwrap();
    let text = res.text().await.unwrap();
    let html = text.as_str();

    let doc = Document::from(html);
    let title = doc.find(Name("h1")).next().unwrap().text();

    println!("Title: {}", title.green());

    let chapter_list = doc
        .find(Class("row-content-chapter"))
        .collect::<Vec<Node>>();

    let mut chapter_blocks = chapter_list[0].find(Name("a")).collect::<Vec<Node>>();
    chapter_blocks.reverse();

    println!(
        "Found {} chapters",
        chapter_blocks.len().to_string().green()
    );

    let mut chapter_urls = chapter_blocks
        .iter()
        .map(|c| c.attr("href").unwrap())
        .collect::<Vec<_>>();

    if args.list.clone() {
        crate::utils::list::list(chapter_urls.clone()).unwrap();
        return Ok(());
    }

    chapter_urls = if let Some(skip) = args.skip.clone() {
        chapter_urls[skip as usize..].to_vec()
    } else {
        chapter_urls
    };

    chapter_urls = if let Some(chapter) = args.chapter.clone() {
        vec![chapter_urls[chapter as usize]]
    } else {
        chapter_urls
    };

    chapter_urls = if let Some(range) = args.range.clone() {
        let range = range.split("-").collect::<Vec<_>>();
        let start = range[0].parse::<usize>().unwrap();
        let end = range[1].parse::<usize>().unwrap();

        chapter_urls[start..end].to_vec()
    } else {
        chapter_urls
    };

    if let Some(name) = args.name.clone() {
        chapter_urls = super::downloaders::name(chapter_urls, name);
    }

    println!(
        "Downloading {} chapters",
        chapter_urls.len().to_string().green()
    );

    for (i, url) in chapter_urls.iter().enumerate() {
        let re = Regex::new("-([0-9]+\\.?[0-9]?)").unwrap();
        let chapter = re.find(url).unwrap().as_str().replace("-", "");

        println!(
            "\nDownloading Chapter {} ({}/{})",
            chapter.green(),
            i + 1,
            chapter_urls.len()
        );

        chapmanganato_get_imgs(
            url,
            &format!("{}/{}/chapter_{}", &g.output_dir, title, chapter),
            args.verbose.clone(),
        )
        .await;
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }

    Ok(())
}

pub(crate) async fn chapmanganato_get_imgs(url: &str, path: &str, verbose: bool) {
    let g: Globals = Globals::new();
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

        println!("Found {} images", imgs.len().to_string().green());

        for img in imgs {
            let src = img.attr("src").unwrap();
            chapmanganato_fetch_img(src, &i.to_string(), &path, verbose.clone()).await;
            i += 1;
            tokio::time::sleep(std::time::Duration::from_millis(g.img_delay.clone())).await;
        }

        if verbose {
            println!(
                "Finished chapter {} in {} seconds",
                &url.split("/").collect::<Vec<&str>>()[5].green(),
                start.elapsed().as_secs().to_string().green(),
            );
        }
    }
}

async fn chapmanganato_fetch_img(url: &str, name: &str, path: &str, verbose: bool) {
    let start = Instant::now();

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
            "Downloaded image {} in {} {}",
            num.green(),
            elapsed.green(),
            "s".green()
        );
    }
}
