use dotenv::dotenv;
use std::env;

pub const IMG_DELAY: u64 = 500;
pub const OUTPUT_DIR: &str = "./output";
pub const CHAPTER_DELAY: u64 = 3000;

pub(crate) struct Globals {
    pub img_delay: u64,
    pub output_dir: String,
    pub chapter_delay: u64,
}

impl Globals {
    pub(crate) fn new() -> Globals {
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

        println!("img_delay: {}", img_delay);
        println!("output_dir: {}", output_dir);
        println!("chapter_delay: {}", chapter_delay);

        Globals {
            img_delay: img_delay,
            output_dir: output_dir,
            chapter_delay: chapter_delay,
        }
    }
}
