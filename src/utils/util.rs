use colored::Colorize;

pub fn supported_site(site_name: &str) -> () {
    println!("Site supported: {}", site_name.green());
}
