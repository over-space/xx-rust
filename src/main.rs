use std::{ops::Deref, thread};
use common::random_number;

struct Book<T>{
    title: String,
    author: String,
    price: T
}

fn main() {

    let a = |x| {x * 2};

    println!("a = {}", a(2));

    let x = 3;

    let b = |b|{b + x};

    println!("a = {}, x= {}", b(2), x);

    let add = |a, b, c|{a + b + c};

    println!("a = {}", add(1, 2, 3));

    let add = |a, b, c|{a + b + c};

    println!("a = {}", add(1.1, 2.2, 3.3));

    let t1 = thread::spawn(|| {
            let n = thread::current().name().unwrap_or("Unnamed").to_string();
            for i in 1..10 {
                println!("子线程1, name = {}, i = {}", n, i)
            }
        });

    thread::Builder::new().name("thread-1".to_string()).spawn(||{
            let n = thread::current().name().unwrap_or("Unnamed").to_string();
            for i in 1..10 {
                println!("子线程2, name = {}, i = {}", n, i)
            }
        }).unwrap();

    t1.join().unwrap();
    // thread::sleep(Duration::from_secs(10));

    let a1 = is_even(4);
    println!("a = {:?}", a1);

    let a1 = is_even(5);
    println!("a = {:?}", a1);

    let a = 32;
    let b = Box::new(a);
    let c = *b;
    println!("a: {}, b: {}", a, b);
    println!("{}", 32 == a);
    println!("{}", 32 == *b);

    let a = "hello rust!!!".to_string();
    let b = Box::new(a);
    println!("a: {}, b: {}", "none".to_owned(), b);
    
    let a = CustomBox{
            value: 100,
        };

    println!("a: {:?}", a);
    println!("100 == *a : {}", 100 == *a);    

    // let secret_number = rand::thread_rng().gen_range(10..20);
    // println!("secret_number = {}", secret_number);
    let a = random_number::gen_random_number(10, 1000);
    println!("secret_number = {}", a);

    println!("main方法执行完毕！");

}

#[derive(Debug)]
struct CustomBox<T>{
    value: T,
}

impl<T> Deref for CustomBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}


impl<T> Drop for CustomBox<T> {
    fn drop(&mut self) {
        println!("CustomBox drop");
    }
}


fn is_even(n : i32) -> Result<bool, String>{
    if n % 2 == 0{
        Ok(true)
    }else{
        // panic!("程序崩溃了");
        Err(format!("{} is not even", n))
    }
}
