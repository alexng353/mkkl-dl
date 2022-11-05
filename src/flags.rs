use crate::util::Color;
use std::vec;

pub fn list(urls: vec::Vec<&str>, name_index: usize) -> std::io::Result<()> {
    // let tmp = last part of url

    let c = Color::new();
    println!("{}{}{}", c.green, "Chapters:", c.end);

    // get average length of url to calculate padding
    let mut total = 0;
    for url in urls.clone() {
        // println!("{}", url.split("/").collect::<Vec<&str>>()[name_index]);
        total += url.split("/").collect::<Vec<&str>>()[name_index].len();
    }

    let avg = total / urls.clone().len();

    // figure out the width of the console
    let width = term_size::dimensions().unwrap().0;

    // calculate how many urls can fit on one line
    let mut num = 0;
    let mut tmp = 0;
    for url in urls.clone() {
        tmp += url.split("/").collect::<Vec<&str>>()[name_index].len() + 5;
        if tmp > width {
            break;
        }
        num += 1;
    }
    num = num / 2;
    // num -= 1;

    // print n urls per line, 5 characters of padding PER SIDE
    // println!("{}", avg);
    let mut i = 0;
    for url in urls.clone() {
        if i == num {
            println!();
            i = 0;
        }
        // let padding = avg - url.len() + 3;
        //     let num = format!("{:0>3}", name);

        // split url by / and get the last part
        let name =
            url.split("/").collect::<Vec<&str>>()[url.split("/").collect::<Vec<&str>>().len() - 1];

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

use -h or --help for more info{}",
        c.cyan, c.yellow, c.cyan, c.white, c.cyan, c.yellow, c.cyan, c.yellow, c.cyan, c.end
    );
    Ok(())
}
