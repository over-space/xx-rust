use std::fs;

fn main() {

    let url = "https://rust-lang.com/";

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("body = {body:?}");

    let md = html2md::parse_html(&body);

    let result = fs::write("rust.md", md.as_bytes());
    
    match result { 
        Ok(_) => println!("File written successfully"),
        Err(e) => println!("Error writing file: {}", e),
    }
}
