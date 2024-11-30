use std::{any::Any, ops};

use clap::{Arg, Command};

#[derive(Debug, Default)]
struct Age(u32);

#[derive(Debug, Default)]
struct User {
    name: String,
    email: String,
    age: Age,
}

#[derive(Debug, Default)]
struct Book{
    name: String,
}


trait Builder {
    type ITEM;
    fn build() -> Self::ITEM;
}

impl Builder for User {
    type ITEM = User;

    fn build() -> Self::ITEM {
        User::default()
    }
}

impl Builder for Book {
    type ITEM = Book;

    fn build() -> Self::ITEM {
        Book::default()
    }
}

impl User {
    fn name(mut self, name:String) -> Self {
        self.name = name;
        self
    }
    fn email(mut self, email: String) -> Self {
        self.email = email;
        self
    }
    fn age(mut self, age: u32) -> Self {
        self.age = Age(age);
        self
    }
}

fn main() {

    let user1 = User::default();
    println!("{:?}", user1);

    let user = User::build()
        .name("lisi".to_string())
        .email("lisi@example.com".to_string())
        .age(10);
    println!("{:?}", user);

    let foo1 = Foo{ value: 1 };
    let foo2 = Foo{ value: 2 };
    let foo = foo1 + foo2;
    println!("{:?}", foo);


}

fn command() {
    let matches = Command::new("My CLI App")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A simple CLI app in Rust")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Sets your name"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enables verbose mode"),
        )
        .get_matches();

    if let Some(name) = matches.get_one::<String>("name") {
        println!("Hello, {}!", name);
    } else {
        println!("Hello, World!");
    }

    if matches.contains_id("verbose") {
        println!("Verbose mode is enabled.");
    }
}


struct Sheep {}
struct Cow {}

trait Animal {
    // 实例方法签名
    fn noise(&self) -> &'static str;
}

// 实现 `Sheep` 的 `Animal` trait。
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// 实现 `Cow` 的 `Animal` trait。
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体。
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

#[derive(Debug)]
struct Foo{
    value: u32,
}

impl ops::Add<Foo> for Foo {
    type Output = Foo;

    fn add(self, rhs: Foo) -> Self::Output {
        Foo{
            value: self.value + rhs.value
        }
    }
}