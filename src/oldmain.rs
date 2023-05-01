// custom modules
mod chapmanganato;
mod compress;
mod globals;
mod handlers;
mod mangakakalot;
mod utils;

// imports
use std::fs::{self, DirEntry};
use tokio;
use url::Url;

use crate::{globals::Globals, utils::color::Color};

// object with ansii color codes
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let c = Color::new();

    println!(
        "{}rust-mangakakalot {}{} {}by {}alexng353 {}",
        c.cyan,
        c.magenta,
        env!("CARGO_PKG_VERSION"),
        c.blue,
        c.yellow,
        c.end
    );

    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        // println!("{}{}{}", c.red, "No arguments provided", c.end);
        help();
        return Ok(());
    }
    let command: &str = &args[1];

    match command {
        "-h" | "--help" | "help" | "h" => {
            help();
        }
        "compress" => compress().await,
        "clean" => {
            // check for -y, -F or --force
            let force = args
                .iter()
                .any(|x| x == "-y" || x == "-f" || x == "--force" || x == "-F");

            let exists = fs::metadata("./output").is_ok() || fs::metadata("./zipped").is_ok();

            if !exists {
                println!("{}{}{}", c.red, "No files to delete", c.end);
                return Ok(());
            }

            if !force {
                println!(
                    "{}Are you sure you want to delete all files? (y/n){}",
                    c.red, c.end
                );
                let mut input = String::new();

                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim() != "y" {
                    println!("{}{}{}", c.red, "Aborting", c.end);
                    return Ok(());
                }
            }

            fs::remove_dir_all("./output")?;
            fs::remove_dir_all("./zipped")?;
        }
        _ => {
            download().await.unwrap();

            let autocompress = args.iter().any(|x| x == "--autocompress" || x == "-a")
                || Globals::new().auto_compress;

            if autocompress {
                compress().await;
            }
        }
    }

    Ok(())
}

async fn download() -> std::io::Result<()> {
    let start = std::time::Instant::now();

    let g = Globals::new();
    let c = Color::new();

    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("{}{}{}", c.red, "No url given", c.end);
        return Ok(());
    }
    let url = Url::parse(&args[1]).unwrap();

    let mut iter = args.iter();

    let mut skip = 0;

    while let Some(arg) = iter.next() {
        if arg == "--skip" || arg == "-s" {
            if let Some(num) = iter.next() {
                skip = num.parse::<u32>().unwrap();
            }
        }
    }

    // split url into parts
    let site_name = url.host_str().unwrap();

    fs::create_dir_all(&g.output_dir)?;

    match site_name {
        "mangakakalot.com" => mangakakalot::downloader(url, skip).await.unwrap(),
        "chapmanganato.com" => chapmanganato::downloader(url, skip).await.unwrap(),
        _ => {
            println!("{}{}{}{}", c.red, site_name, " is not supported", c.end);
            return Ok(());
        }
    }

    let duration = start.elapsed();

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

fn help() {
    let c = Color::new();
    let help_msg = format!(include_str!("./help.txt"), c.red, c.end);

    println!("{}", help_msg);
}

async fn compress() {
    let force = std::env::args().any(|x| x == "--force" || x == "-F");

    fs::create_dir_all("./zipped").expect("Failed to create \"zipped\" directory");

    let books = fs::read_dir("./output").unwrap();
    for book in books {
        println!("\nCompressing {}", path_replacer(&book).unwrap());

        fs::create_dir_all(format!("./zipped/{}", get_last(&book).unwrap()))
            .expect("Failed to create \"zipped\" directory");

        let chapters = fs::read_dir(book.as_ref().unwrap().path()).unwrap();
        for chapter in chapters {
            // regex for .DS_Store
            let re = regex::Regex::new(r"\.DS_Store").unwrap();

            if re.is_match(&chapter.as_ref().unwrap().path().to_str().unwrap()) {
                continue;
            }

            let path = format!(
                "{}{}",
                path_replacer(&chapter).unwrap().replace("output", "zipped"),
                globals::Globals::new().zip_format
            );

            println!("\nCompressing {}", path_replacer(&chapter).unwrap());

            if !force && std::path::Path::new(&path).exists() {
                // get the size of both the zip and the input folder
                let zip_size = std::fs::metadata(&path).unwrap().len();
                let mut input_size: u64 = 0;

                for file in fs::read_dir(chapter.as_ref().unwrap().path()).unwrap() {
                    input_size += std::fs::metadata(file.unwrap().path()).unwrap().len();
                }

                if zip_size > input_size {
                    println!("{} already exists, skipping", path);
                    continue;
                }
            }

            compress::compress(&chapter.as_ref().unwrap().path().to_str().unwrap(), &path).await;
        }
    }
}

fn get_last(path: &Result<DirEntry, std::io::Error>) -> Result<String, &'static str> {
    match path {
        Ok(entry) => {
            let path_ = entry.path();
            let path_str = path_.to_str().unwrap();
            Ok(path_str
                .replace("\\", "/")
                .split("/")
                .last()
                .unwrap()
                .to_string())
        }
        Err(_) => Err("Error reading path"),
    }
}

fn path_replacer(path: &Result<DirEntry, std::io::Error>) -> Result<String, &'static str> {
    match path {
        Ok(entry) => {
            let path_ = entry.path();
            let path_str = path_.to_str().unwrap();
            Ok(path_str.replace("\\", "/"))
        }
        Err(_) => Err("Error reading path"),
    }
}
