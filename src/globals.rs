use dotenv::dotenv;
use std::env;

const IMG_DELAY: u64 = 100;
const OUTPUT_DIR: &str = "./output";
const CHAPTER_DELAY: u64 = 1000;
const ZIP_FORMAT: &str = ".zip";
const AUTO_COMPRESS: bool = false;

pub(crate) struct Globals {
    pub img_delay: u64,
    pub output_dir: String,
    pub chapter_delay: u64,
    pub zip_format: String,
    pub auto_compress: bool,
}

impl Globals {
    pub(crate) fn new() -> Globals {
        let args = std::env::args().collect::<Vec<String>>();
        dotenv().ok();
        // let img_delay = env::var("IMG_DELAY").unwrap().parse::<u64>().unwrap();
        // if no img_delay is set, use the default
        let img_delay = env::var("IMG_DELAY")
            .unwrap_or_else(|_| IMG_DELAY.to_string())
            .parse::<u64>()
            .unwrap();
        // let output_dir = env::var("OUTPUT_DIR").unwrap();
        let output_dir = env::var("OUTPUT_DIR").unwrap_or_else(|_| OUTPUT_DIR.to_string());
        // let chapter_delay = env::var("CHAPTER_DELAY").unwrap().parse().unwrap();
        let chapter_delay = env::var("CHAPTER_DELAY")
            .unwrap_or_else(|_| CHAPTER_DELAY.to_string())
            .parse::<u64>()
            .unwrap();

        let zip_format = env::var("ZIP_FORMAT").unwrap_or_else(|_| ZIP_FORMAT.to_string());
        // search args for -f or --format
        let zip_format =
            if args.contains(&"-f".to_string()) || args.contains(&"--format".to_string()) {
                // get the index of the format flag
                let index = args
                    .iter()
                    .position(|x| x == "-f" || x == "--format")
                    .unwrap();
                // get the format from the next index
                // if args[index + 1] has no leading ".", add one
                if args[index + 1].starts_with(".") {
                    args[index + 1].to_string()
                } else {
                    format!(".{}", args[index + 1])
                }
            } else {
                zip_format
            };

        let auto_compress = env::var("AUTO_COMPRESS")
            .unwrap_or_else(|_| AUTO_COMPRESS.to_string())
            .parse::<bool>()
            .unwrap();

        Globals {
            img_delay: img_delay,
            output_dir: output_dir,
            chapter_delay: chapter_delay,
            zip_format: zip_format,
            auto_compress: auto_compress,
        }
    }
}
