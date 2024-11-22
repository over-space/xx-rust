use std::{fs::File, io::Read, path::Display};

struct Book{
    name: String,
    author: String,
}

impl Display for Book{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.name, self.author)
    }
}

fn main() {

    let mut file = File::open("jdbc.txt").unwrap();
    let mut contents = String::new();


    // 将文件内容读取到字符串中
    file.read_to_string(&mut contents).expect("无法读取文件内容");

    for line in contents.lines() {
        let parts: Vec<&str> = line.split('/').map(|x| x.trim()).collect();
        println!("{},{}", parts[0], parts[parts.len() - 1]);
    }

}
