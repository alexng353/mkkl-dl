pub fn name(urls: Vec<&str>) -> Vec<&str> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut iter = args.iter();
    let mut name: Option<String> = None;
    while let Some(arg) = iter.next() {
        if arg == "--name" || arg == "-n" {
            if let Some(n) = iter.next() {
                name = Some(n.to_string());
            }
        }
    }
    if let Some(value) = name {
        let mut tmp = Vec::new();
        for url in urls.clone() {
            tmp.push(
                url.split("/").collect::<Vec<&str>>()
                    [url.split("/").collect::<Vec<&str>>().len() - 1],
            );
        }
        let pos = urls
            .iter()
            .position(|x| x.split("/").collect::<Vec<&str>>()[4] == value)
            .unwrap();
        vec![urls[pos]]
    } else {
        urls
    }
}

pub fn chapter(urls: Vec<&str>) -> Vec<&str> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut iter = args.iter();
    let mut number: Option<usize> = None;
    while let Some(arg) = iter.next() {
        if arg == "--chapter" || arg == "-c" {
            if let Some(num) = iter.next() {
                number = Some(num.parse::<usize>().unwrap());
            }
        }
    }
    if let Some(value) = number {
        vec![urls[value]]
    } else {
        urls
    }
}

pub fn range(urls: Vec<&str>) -> Vec<&str> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut iter = args.iter();
    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;
    while let Some(arg) = iter.next() {
        if arg == "--range" || arg == "-r" {
            if let Some(num) = iter.next() {
                start = Some(num.parse::<usize>().unwrap());
            }
            if let Some(num) = iter.next() {
                end = Some(num.parse::<usize>().unwrap());
            }
        }
    }

    if let Some(start) = start {
        if let Some(end) = end {
            urls[start..end].to_vec()
        } else {
            urls[start..].to_vec()
        }
    } else {
        urls
    }
}
