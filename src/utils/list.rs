use anyhow::Result;
use colored::Colorize;
use std::vec;

pub fn list(urls: vec::Vec<&str>) -> Result<()> {
    println!("{}", "Chapters:".green());

    let total: usize = urls
        .iter()
        .map(|url| url.rsplit("/").next().unwrap().len())
        .sum();

    let avg = total / urls.clone().len();

    let width = term_size::dimensions().unwrap().0;

    let mut num = 0;
    let mut tmp = 0;
    for url in urls.clone() {
        tmp += url.split("/").collect::<Vec<&str>>().last().unwrap().len() + 3;
        if tmp > width {
            break;
        }
        num += 1;
    }
    num = num / 2;

    let mut i = 0;
    for url in urls.clone() {
        if i == num {
            println!();
            i = 0;
        }

        let split_url = url.split("/").collect::<Vec<&str>>();
        let name = split_url.last().unwrap();

        print!(
            "{} {:width$}{}",
            format!("{:0>3}:", urls.iter().position(|x| x == &url).unwrap()).yellow(),
            name,
            " ".repeat(2),
            width = avg + 5
        );
        i += 1;
    }
    println!();

    // help message
    const YELLOW: &str = "\x1b[33m";
    const CYAN: &str = "\x1b[36m";
    const WHITE: &str = "\x1b[37m";
    const END: &str = "\x1b[0m";

    println!(
        "{}
use -c [n] or --chapter [n]         to download a chapter by index ({}yellow{} number)
use -n [n] or --name [n]            to download a chapter by name ({}white{} text)
use -s [n] or --skip [n]            to skip the first n by index ({}yellow{} number)
use -r [n]-[n], --range [n] [n]     Download chapters from [n] to [n] by index ({}yellow{} number)
use -v, --verbose                   to show more info

use -h or --help for more info{}",
        CYAN, YELLOW, CYAN, WHITE, CYAN, YELLOW, CYAN, YELLOW, CYAN, END
    );
    Ok(())
}
