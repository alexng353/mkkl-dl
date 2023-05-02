use std::fs;

use crate::{downloaders::chapmanganato, downloaders::mangakakalot, utils::compress};

use super::*;

/// Download a manga from mangakakalot or chapmanganato
#[derive(Parser)]
pub struct Args {
    /// The URL of the manga to download
    #[clap(index = 1)]
    url: String,

    /// Skip the first n chapters
    #[clap(short, long)]
    skip: Option<u32>,

    /// Autocompress
    #[clap(short, long)]
    autocompress: bool,

    /// Download a specific chapter n
    #[clap(short, long)]
    chapter: Option<u32>,

    /// Download a specific range of chapters n-m
    #[clap(short, long)]
    range: Option<String>,

    /// Verbose output
    #[clap(short, long)]
    verbose: bool,

    /// List all chapters
    #[clap(short, long)]
    list: bool,

    /// Name of the chapter
    #[clap(short, long)]
    name: Option<String>,
}

pub async fn command(args: Args, _json: bool) -> Result<()> {
    let start = std::time::Instant::now();

    let url = url::Url::parse(&args.url).context("Failed to parse URL")?;

    let site_name = url.host_str().context("Failed to get hostname")?;

    fs::create_dir_all("./output").context("Failed to create \"output\" directory")?;

    let new_args = crate::downloaders::downloaders::Args {
        url: args.url,
        skip: args.skip,
        chapter: args.chapter,
        range: args.range,
        verbose: args.verbose,
        list: args.list,
        name: args.name,
    };

    match site_name {
        "mangakakalot.com" => mangakakalot::mangakakalot(new_args).await?,
        "chapmanganato.com" => chapmanganato::chapmanganato(new_args).await?,
        _ => {
            println!("{} is not supported", site_name);
            return Ok(());
        }
    };

    let duration = start.elapsed();

    println!(
        "{} ({})",
        "Done".green(),
        duration.as_secs().to_string().cyan()
    );

    if args.autocompress.clone() {
        compress::compress(false, ".cbz".to_string()).await?;
    }

    Ok(())
}
