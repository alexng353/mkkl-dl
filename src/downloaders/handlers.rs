use super::downloader::Args;

pub(crate) fn name(chapter_urls: Vec<&str>, args: Args) -> Vec<&str> {
    if let Some(name) = args.name.clone() {
        let mut tmp = Vec::new();

        for url in chapter_urls.clone() {
            let current = url.split("/").collect::<Vec<&str>>();
            tmp.push(current[current.len() - 1]);
        }

        if let Some(pos) = tmp.iter().position(|&x| &x.trim() == &name.trim()) {
            vec![chapter_urls[pos]]
        } else {
            vec![]
        }
    } else {
        chapter_urls
    }
}

pub(crate) fn skip(chapter_urls: Vec<&str>, args: Args) -> Vec<&str> {
    if let Some(skip) = args.skip.clone() {
        chapter_urls[skip as usize..].to_vec()
    } else {
        chapter_urls
    }
}

pub(crate) fn chapter_handler(chapter_urls: Vec<&str>, args: Args) -> Vec<&str> {
    if let Some(chapter) = args.chapter.clone() {
        vec![chapter_urls[chapter as usize]]
    } else {
        chapter_urls
    }
}

pub(crate) fn range(chapter_urls: Vec<&str>, args: Args) -> Vec<&str> {
    if let Some(range) = args.range.clone() {
        let range = range.split("-").collect::<Vec<_>>();
        let start = range[0].parse::<usize>().unwrap();
        let end = range[1].parse::<usize>().unwrap();

        chapter_urls[start..end].to_vec()
    } else {
        chapter_urls
    }
}
