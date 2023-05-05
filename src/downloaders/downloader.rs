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

#[derive(Clone)]
pub(crate) struct Args {
    pub(crate) url: String,
    pub(crate) skip: Option<u32>,
    pub(crate) chapter: Option<u32>,
    pub(crate) range: Option<String>,
    pub(crate) verbose: bool,
    pub(crate) list: bool,
    pub(crate) name: Option<String>,
}

pub(crate) struct Downloader {
    pub(crate) args: Args,
    pub(crate) chapter_list: String,
    pub(crate) get_imgs_class: String,
    pub(crate) chapter_url_regex: String,
    pub(crate) referrer: String,
    g: Globals,
}

impl Downloader {
    pub(crate) fn new(
        args: Args,
        chapter_list: String,
        get_imgs_class: String,
        chapter_url_regex: String,
        referrer: String,
    ) -> Self {
        Self {
            args: args,
            chapter_list: chapter_list,
            get_imgs_class: get_imgs_class,
            chapter_url_regex: chapter_url_regex,
            referrer: referrer,
            g: Globals::new(),
        }
    }

    pub(crate) async fn download(&self) -> Result<()> {
        let url = Url::parse(&self.args.url).unwrap();

        let site_name = url.host_str().unwrap();
        supported_site(site_name);

        let res = reqwest::get(url).await.unwrap();
        let text = res.text().await.unwrap();
        let html = text.as_str();

        let doc = Document::from(html);
        let title = doc.find(Name("h1")).next().unwrap().text();

        println!("Title: {}", title.green());

        let chapter_list = doc
            .find(Class(self.chapter_list.as_str()))
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

        if self.args.list.clone() {
            crate::utils::list::list(chapter_urls.clone()).unwrap();
            return Ok(());
        }

        chapter_urls = super::handlers::skip(chapter_urls, self.args.clone());
        chapter_urls = super::handlers::chapter_handler(chapter_urls, self.args.clone());
        chapter_urls = super::handlers::range(chapter_urls, self.args.clone());
        chapter_urls = super::handlers::name(chapter_urls, self.args.clone());

        println!(
            "Downloading {} chapters",
            chapter_urls.len().to_string().green()
        );

        for (i, url) in chapter_urls.iter().enumerate() {
            let re = Regex::new(&self.chapter_url_regex).unwrap();
            let chapter = re.find(url).unwrap().as_str()[1..].to_string();

            println!(
                "\nDownloading Chapter {} ({}/{})",
                chapter.green(),
                i + 1,
                chapter_urls.len()
            );

            self.get_imgs(
                url,
                &format!("{}/{}/chapter_{}", &self.g.output_dir, title, chapter),
                self.args.verbose.clone(),
            )
            .await;
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }

        Ok(())
    }

    pub(crate) async fn get_imgs(&self, url: &str, path: &str, verbose: bool) {
        let g: Globals = Globals::new();
        fs::create_dir_all(path).unwrap();

        let res = reqwest::get(url.clone()).await;
        let text = res.unwrap().text().await.unwrap();
        let html = text.as_str();

        let document = Document::from(html);
        for node in document.find(Class(self.get_imgs_class.as_str())) {
            let start = Instant::now();

            let mut i = 0;

            // get the length of a list of all the imgs
            let imgs = node.find(Name("img")).collect::<Vec<Node>>();

            println!("Found {} images", imgs.len().to_string().green());

            for img in imgs {
                let src = img.attr("src").unwrap();
                self.fetch_img(src, &i.to_string(), &path, verbose.clone())
                    .await;
                i += 1;
                tokio::time::sleep(std::time::Duration::from_millis(g.img_delay.clone())).await;
            }

            if verbose {
                println!(
                    "Finished chapter {} in {} seconds",
                    url.split("/")
                        .collect::<Vec<&str>>()
                        .last()
                        .unwrap()
                        .green(),
                    start.elapsed().as_secs().to_string().green(),
                );
            }
        }
    }

    pub(crate) async fn fetch_img(&self, url: &str, name: &str, path: &str, verbose: bool) {
        let start = Instant::now();

        let client = reqwest::Client::new();

        // Headers need to be here to trick the server into thinking we are a browser requesting from "https://mangakakalot.com/"
        let res = client
        .get(url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36")
        .header(header::REFERER, &self.referrer)
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
                "Downloaded image {} in {}{}",
                num.green(),
                elapsed.green(),
                "s".green()
            );
        }
    }
}
