use crate::utils::color::Color;

pub fn supported_site(site_name: &str) -> () {
    let c = Color::new();
    println!("{}{}{}{}", "Site supported: ", c.green, site_name, c.end);
}
