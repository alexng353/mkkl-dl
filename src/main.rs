// custom modules
mod chapmanganato;
mod compress;
mod globals;
mod handlers;
mod mangakakalot;
mod utils;

// imports
use std::fs;
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
            fs::remove_dir_all("./output")?;
            fs::remove_dir_all("./zipped")?;
        }
        _ => {
            download().await.unwrap();

            if args.contains(&"--autocompress".to_string())
                || globals::Globals::new().auto_compress
                || args.contains(&"-a".to_string())
            {
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
    let url = &args[1];
    let parsable = Url::parse(url).unwrap();

    // search args for --skip [number]
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
    let site_name = parsable.host_str().unwrap();

    fs::create_dir_all(&g.output_dir)?;

    match site_name {
        "mangakakalot.com" => mangakakalot::downloader(parsable, skip).await.unwrap(),
        "chapmanganato.com" => chapmanganato::downloader(parsable, skip).await.unwrap(),
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

fn help() {
    let help_msg = r#"Usage: ./rust-mangakakalot [command] [url] [options]

Commands:
    download    Download manga from url
    compress    Compress downloaded manga into zip files
    help        Show this message

Options:
    -l, --list                      List chapters
    -f, --format                    Set the format of the zip file (default: .cbz)
    -h, --help                      Show this message
    -a, --autocompress              Automatically compress downloaded manga into zip files
    -s [number], --skip [number]    Start downloading from chapter [number]
    -c [n] or --chapter [n]         Download chapter by index (see --list)
    -n [n] or --name [n]            Download chapter by name in url (see --list)
    -r [n] [n], --range [n] [n]     Download chapters from [n] to [n]

    -v, --verbose                   Show detailed information about the download process
"#;

    println!("{}", help_msg);
}

async fn compress() {
    fs::create_dir_all("./zipped").expect("Failed to create \"zipped\" directory");
    let books = fs::read_dir("./output").unwrap();
    for book in books {
        println!(
            "Compressing {}",
            book.as_ref().unwrap().path().to_str().unwrap()
        );

        fs::create_dir_all(
            "./zipped/".to_string()
                + &book
                    .as_ref()
                    .unwrap()
                    .path()
                    .to_str()
                    .unwrap()
                    .replace("\\", "/")
                    .split("/")
                    .last()
                    .unwrap(),
        )
        .expect("Failed to create \"zipped\" directory");

        let chapters = fs::read_dir(book.as_ref().unwrap().path()).unwrap();
        for chapter in chapters {
            // regex for .DS_Store
            let re = regex::Regex::new(r"\.DS_Store").unwrap();

            if re.is_match(&chapter.as_ref().unwrap().path().to_str().unwrap()) {
                continue;
            }
            // split / get last
            compress::compress(
                &chapter.as_ref().unwrap().path().to_str().unwrap(),
                &format!(
                    "./zipped/{}/{}{}",
                    book.as_ref()
                        .unwrap()
                        .path()
                        .to_str()
                        .unwrap()
                        .replace("\\", "/")
                        .split("/")
                        .last()
                        .unwrap(),
                    &chapter
                        .as_ref()
                        .unwrap()
                        .path()
                        .to_str()
                        .unwrap()
                        .replace("\\", "/")
                        .split("/")
                        .last()
                        .unwrap(),
                    globals::Globals::new().zip_format
                ),
            )
            .await;
        }
    }
}
