pub(crate) struct Color {
    pub red: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub magenta: &'static str,
    pub cyan: &'static str,
    pub white: &'static str,
    pub end: &'static str,
}

impl Color {
    pub(crate) fn new() -> Color {
        Color {
            red: "\x1b[31m",
            green: "\x1b[32m",
            yellow: "\x1b[33m",
            blue: "\x1b[34m",
            magenta: "\x1b[35m",
            cyan: "\x1b[36m",
            white: "\x1b[37m",

            end: "\x1b[0m",
        }
    }
}

pub fn supported_site(site_name: &str) -> () {
    let c = Color::new();
    println!("{}{}{}{}", "Site supported: ", c.green, site_name, c.end);
}
