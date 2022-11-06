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
