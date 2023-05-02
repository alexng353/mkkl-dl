#[derive(Clone)]
pub(crate) struct Args {
    pub(crate) url: String,
    pub(crate) skip: Option<u32>,
    pub(crate) chapter: Option<u32>,
    pub(crate) range: Option<String>,
    pub(crate) verbose: bool,
    pub(crate) list: bool,
    pub(crate) name: Option<String>,
}

pub fn name(urls: Vec<&str>, name: String) -> Vec<&str> {
    let mut tmp = Vec::new();
    for url in urls.clone() {
        tmp.push(
            url.split("/").collect::<Vec<&str>>()[url.split("/").collect::<Vec<&str>>().len() - 1],
        );
    }

    if let Some(pos) = tmp.iter().position(|&x| &x.trim() == &name.trim()) {
        vec![urls[pos]]
    } else {
        vec![]
    }
}
