use super::downloader::{Args, Downloader};
use anyhow::{Ok, Result};
use colored::Colorize;

pub(crate) async fn downloader(args: Args) -> Result<()> {
    let d = Downloader::new(
        args,
        "chapter-list".to_string(),
        "container-chapter-reader".to_string(),
        "_([0-9]+\\.?[0-9]?)".to_string(),
        "https://mangakakalot.com/".to_string(),
    );

    if let Err(e) = d.download().await {
        eprintln!("{}", e.to_string().red());
    }

    Ok(())
}
