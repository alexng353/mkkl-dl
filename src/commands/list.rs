use std::{fs, io::Write};

use anyhow::Ok;
use select::{
    document::Document,
    node::Node,
    predicate::{Class, Name},
};
use url::Url;

use crate::utils::util::supported_site;

use super::*;

/// list all chapters
#[derive(Parser)]
pub struct Args {
    #[clap(index = 1)]
    url: String,
}

pub async fn command(args: Args, _json: bool) -> Result<()> {
    let url = Url::parse(&args.url).unwrap();

    let site_name = url.host_str().unwrap();

    let chapter_list_classname = match site_name {
        "mangakakalot.com" => "chapter-list",
        "chapmanganato.com" => "row-content-chapter",
        _ => {
            println!("{} is not supported", site_name);
            return Ok(());
        }
    };

    supported_site(site_name);

    let res = reqwest::get(url).await.unwrap();
    let text = res.text().await.unwrap();
    let html = text.as_str();

    let doc = Document::from(html);
    let title = doc.find(Name("h1")).next().unwrap().text();

    println!("Title: {}", title.green());

    let mut file = fs::File::create(format!("{}.html", title))?;
    file.write_all(html.as_bytes())?;

    let chapter_list = doc
        .find(Class(chapter_list_classname))
        .collect::<Vec<Node>>();

    let mut chapter_blocks = chapter_list[0].find(Name("a")).collect::<Vec<Node>>();
    chapter_blocks.reverse();

    println!(
        "Found {} chapters",
        chapter_blocks.len().to_string().green()
    );

    let chapter_urls = chapter_blocks
        .iter()
        .map(|c| c.attr("href").unwrap())
        .collect::<Vec<_>>();

    crate::utils::list::list(chapter_urls.clone()).unwrap();

    Ok(())
}
