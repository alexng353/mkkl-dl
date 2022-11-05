// custom modules
mod chapmanganato;
mod globals;
mod mangakakalot;
mod util;

// imports
use std::fs;
use tokio;

use crate::{globals::Globals, util::Color};

// object with ansii color codes
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let start = std::time::Instant::now();

    let g = Globals::new();
    let c = Color::new();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("{}{}{}", c.red, "No url given", c.end);
        return Ok(());
    }
    let url = &args[1];

    // search args for --skip [number]
    let mut iter = args.iter();

    let mut skip = 0;

    while let Some(arg) = iter.next() {
        if arg == "--skip" {
            if let Some(num) = iter.next() {
                skip = num.parse::<u32>().unwrap();
            }
        }
    }

    println!(
        "{}{} {}{} {}{} {}{} {}",
        c.cyan,
        "rust-mangakakalot",
        c.magenta,
        "v0.1.0",
        c.blue,
        "by",
        c.yellow,
        "alexng353",
        c.end
    );

    // split url into parts
    let url_parts: Vec<&str> = url.split('/').collect();
    let site_name = url_parts[2];

    fs::create_dir_all(&g.output_dir)?;

    match site_name {
        "mangakakalot.com" => mangakakalot::downloader(url, skip).await.unwrap(),
        "chapmanganato.com" => chapmanganato::downloader(url, skip).await.unwrap(),
        _ => {
            println!("{}{}{}{}", c.red, site_name, " is not supported", c.end);
            return Ok(());
        }
    }

    // println!("{}{}{}{}", "Site name: ", c.green, site_name, c.end);
    let duration = start.elapsed();
    // println!("{}", duration.as_secs());

    println!(
        "{}{} ({}{} seconds) {}",
        c.green,
        "Done",
        c.cyan,
        duration.as_secs(),
        c.end
    );
    Ok(())
}
