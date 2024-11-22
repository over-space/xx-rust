use std::thread;

use common::{self, common::gen_rand_number};

type I32 = i32;

fn main() {
    let hander = thread::spawn(|| {
        for i in 1..=10 {
            let a = gen_rand_number(i, 100);
            println!("随机数：{} = {}", i, a)
        }
    });


    hander.join().unwrap();

    let a = 3 as I32;

    let b = Book::from(a);

    println!("b = {:?}", b);

    let a = 55;

    let b : Book = a.into();

    println!("b = {:?}", b);

    // let a: i32 = "5x".parse().expect("不能转换成i32类型");
    // println!("a = {}", a);

    let a: i32 = "5".parse().expect("不能转换成i32类型");
    println!("a = {}", a);

    let a: i32 = "6".parse().unwrap_or(-1);
    println!("a = {}", a);

    let a: i32 = "6x".parse().unwrap_or(-1);
    println!("a = {}", a);

    let s = "100x".to_string();

    let a: Option<i32> = s.parse().ok();

    match a {
        Some(a) => println!("a = {}", a),
        None =>  println!("{} is not i32", s),
    }

    if let Some(a) = a{
        println!("a = {}", a);
    }else{
        println!("{} is not i32", s);
    }

    let mut a = Some(0);
    while let Some(i) = a {
        if(i > 9){
            println!("finished while let!");
            a = None;
        }else{
            println!("try i + 1, current i = {}", i);
            a = Some(i + 1);
        }
    }

}

#[derive(Debug)]
struct Book{
    price:f64,
}

impl From<i32> for Book{
    fn from(value: i32) -> Self {
        Book{
            price: value as f64,
        }
    }
}
