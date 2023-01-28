use crate::utils::color::Color;
use std::vec;

pub fn list(urls: vec::Vec<&str>) -> std::io::Result<()> {
    // work around for now

    // let tmp = last part of url

    let c = Color::new();
    println!("{}{}{}", c.green, "Chapters:", c.end);

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
            "{}{}: {}{:width$}{}",
            c.yellow,
            format!("{:0>3}", urls.iter().position(|x| x == &url).unwrap()),
            c.end,
            name,
            " ".repeat(2),
            width = avg + 5
        );
        i += 1;
    }
    println!();

    // help message
    println!(
        "{}
use -c [n] or --chapter [n]         to download a chapter by index ({}yellow{} number)
use -n [n] or --name [n]            to download a chapter by name ({}white{} text)
use -s [n] or --skip [n]            to skip the first n by index ({}yellow{} number)
use -r [n] [n] or --range [n] [n]   to download a range of chapters by index ({}yellow{} number)
use -r [n] [n], --range [n] [n]     Download chapters from [n] to [n] by index ({}yellow{} number)
use -v, --verbose                   to show more info

use -h or --help for more info{}",
        c.cyan,
        c.yellow,
        c.cyan,
        c.white,
        c.cyan,
        c.yellow,
        c.cyan,
        c.yellow,
        c.cyan,
        c.yellow,
        c.cyan,
        c.end
    );
    Ok(())
}
