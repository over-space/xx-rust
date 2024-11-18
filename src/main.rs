use core::prelude::v1;
use std::fmt::Debug;



struct BufBuilder{
    buf: Vec<u8>,
    size: i32,
}

impl BufBuilder {
    fn new(size: i32) -> Self {
        BufBuilder { 
            buf: Vec::with_capacity(1024), 
            size,
        }
    }
    fn add(x: i32){
        println!("x = {}", x)
    }
    fn fun_add(v1: Self, v2: Self) -> i32{
        v1.size + v2.size
    }

    fn fun_add2(&self, v2: Self) -> i32{
        self.size + v2.size
    }
}

impl Debug for BufBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

#[derive(Debug, Clone)]
struct Book{
    pages: i32,
    author: String,
    title: String,
}

impl Book {
    fn print(&self){
        println!("Book Title: {}, Author: {}, Pages: {}", self.title, self.author, self.pages);
    }
}
fn print_book(book:Book){
    println!("Book Title: {}, Author: {}, Pages: {}", book.title, book.author, book.pages);
}

fn main(){
    let buf = BufBuilder::new(1);
    BufBuilder::add(33);
    println!("{:?}", buf);

    let v1 = BufBuilder::new(2);

    let v2 = BufBuilder::new(5);

    let v3 = BufBuilder::fun_add(v1, v2);
    println!("{:?}", v3);

    let v4 = BufBuilder::new(2);
    let v5 = BufBuilder::new(5);
    let v6 = v4.fun_add2(v5);
    println!("{:?}", v6);

    let book = Book{
            pages: 100,
            author: "John Doe".to_string(),
            title: "Book Title".to_string(),
        };

    book.print();

    print_book(book.clone());

    println!("{:?}", book)

}
