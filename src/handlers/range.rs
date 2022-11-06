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
