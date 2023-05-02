use std::fs;

use anyhow::Context;
use anyhow::Result;

pub async fn compress(force: bool, format: String) -> Result<()> {
    fs::create_dir_all("./zipped").context("Failed to create \"zipped\" directory")?;

    let books =
        fs::read_dir("./output").context("Failed to read \"output\" directory.\nDoes it exist?")?;

    for book in books {
        println!(
            "\nCompressing {}",
            path_replacer(&book).unwrap().replace("output", "zipped")
        );

        fs::create_dir_all(format!("./zipped/{}", get_last(&book).unwrap()))
            .context("Failed to create \"zipped\" directory")?;

        let chapters = fs::read_dir(book.as_ref().unwrap().path())
            .context("Failed to read \"output\" directory.\nDoes it exist?")?;

        for chapter in chapters {
            let re = regex::Regex::new(r"\.DS_Store").unwrap();

            if re.is_match(&chapter.as_ref().unwrap().path().to_str().unwrap()) {
                continue;
            }

            let path = format!(
                "{}{}",
                path_replacer(&chapter).unwrap().replace("output", "zipped"),
                format
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

            crate::utils::zip::compress(&chapter.as_ref().unwrap().path().to_str().unwrap(), &path)
                .await;
        }
    }
    Ok(())
}

fn get_last(path: &Result<fs::DirEntry, std::io::Error>) -> Result<String, &'static str> {
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

fn path_replacer(path: &Result<fs::DirEntry, std::io::Error>) -> Result<String, &'static str> {
    match path {
        Ok(entry) => {
            let path_ = entry.path();
            let path_str = path_.to_str().unwrap();
            Ok(path_str.replace("\\", "/"))
        }
        Err(_) => Err("Error reading path"),
    }
}
