use std::fs::read_to_string;
#[warn(unused_imports)]
use std::{
    fmt::Display,
    sync::{mpsc, Arc, Mutex},
    thread,
};


use tracing::{info, warn};

fn init() {
    tracing_subscriber::fmt::init();
}

const MAX_COUNT: usize = usize::MAX / 2;
static mut MAX_NUM: u64 = u64::MAX / 2;

fn main() {
    init();

    let (tx, rs) = mpsc::channel();

    thread::spawn(move || match tx.send(1) {
        Ok(_) => {
            info!("send success");
        }
        Err(_) => {
            warn!("send fail");
        }
    });

    info!("receive, {:?}", rs.recv().unwrap());

    let x = 1u8;
    let y = 2i32;
    let z = 3.14f32;
    let s = 6.66f64;

    to_draw1(&x);
    to_draw1(&y);
    to_draw1(&z);

    to_draw2(&x);
    // to_draw2(&y);
    // to_draw2(&z);

    to_draw3(&x);
    // to_draw3(&y);
    // to_draw3(&z);

    to_draw4(Box::new(x));
    to_draw4(Box::new(y));
    to_draw4(Box::new(z));

    let rc = Arc::new(Mutex::new(10));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = rc.clone();
        let t = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(t);
    }

    for ele in handles {
        ele.join().unwrap();
    }
    info!("result : {}", *rc.lock().unwrap());

    let x = 8u8;
    let ptr: *const u8 = &x;
    unsafe {
        info!("ptr: {}", *ptr);
    }
    // thread::spawn(move || {
    //     info!("ptr: {}", *ptr);
    // });
    let mb = MyBox(ptr);
    thread::spawn(move || {
        let ptr = mb.0;
        unsafe {
            info!("ptr: {:?}, v: {}", mb, *ptr);
        }
    })
    .join()
    .unwrap();

    unsafe {
        MAX_NUM += 1;
    }

    let a = Some(1);
    let b: Option<i32> = None;
    info!("Some.or, a = {:?}", a.or(Some(2)));
    info!("Some.or, b = {:?}", a.or(Some(2)));

    info!("Some.and, a = {:?}", a.and(Some(2)));
    info!("Some.and, b = {:?}", b.and(Some(2)));

    info!("Some.or_else, a = {:?}", a.or_else(|| Some(3)));
    info!("Some.or_else, b = {:?}", b.or_else(|| Some(3)));

    info!("Some.and_then, a = {:?}", a.and_then(|_| Some(2)));
    info!(
        "Some.and_then, a = {:?}",
        a.and_then(|_| None as Option<i32>)
    );
    info!("Some.and_then, b = {:?}", b.and_then(|_| Some(2)));

    info!("Some.map, a = {:?}", a.map(|_| 3));
    info!("Some.map, b = {:?}", b.map(|_| 4));

    info!("Some.map_or, a = {:?}", a.map_or(Some(3), |_| Some(4)));
    info!("Some.map_or, b = {:?}", b.map_or(Some(3), |_| Some(4)));

    info!(
        "Some.map_or_else, a = {:?}",
        a.map_or_else(|| Some(1), |_| Some(4))
    );
    info!(
        "Some.map_or_else, b = {:?}",
        b.map_or_else(|| Some(1), |_| Some(4))
    );

    let a: Result<&str, &str> = Ok("success");
    let b: Result<&str, &str> = Err("err");

    info!("Result.map a = {:?}", a.map(|_| "hello"));
    info!("Result.map b = {:?}", b.map(|_| "hello"));

    info!("Result.map_err a = {:?}", a.map_err(|_| "hello"));
    info!("Result.map_err b = {:?}", b.map_err(|_| "hello"));

    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    info!("ptr str: {}, pointer: {}, len: {}", message, pointer, length);

    let b = get_str_at_location(pointer, 10);
    info!("ptr str: {}, pointer: {}, len: {}", b, pointer, length);

    info!("finished");
}

use std::{slice::from_raw_parts, str::from_utf8_unchecked};

// 获取字符串的内存地址和长度
pub fn get_memory_location() -> (usize, usize) {
    let string = "Hello World!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

// 在指定的内存地址读取字符串
pub fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

pub mod thiserror {
    use std::{error, fs::read_to_string};
    use thiserror::Error;

    fn hello() -> Result<(), BusiError> {
        let html = render()?;
        println!("{}", html);
        Ok(())
    }

    fn render() -> Result<String, BusiError> {
        let file = std::env::var("MARKDOWN")?;
        let source = read_to_string(file)?;
        Ok(source)
    }

    #[derive(Error, Debug)]
    enum BusiError {
        #[error("env error")]
        EnvError(#[from] std::env::VarError),
        #[error("io error")]
        IoError(#[from] std::io::Error),
    }
}

#[derive(Debug)]
struct MyBox(*const u8);
unsafe impl Send for MyBox {}

fn to_draw1(draw: &dyn Draw) {
    let a = draw.draw();
    info!("to_draw1, str: {}", a);
}

fn to_draw2<T>(draw: &T)
where
    T: Draw + Add,
{
    let a = draw.draw();
    let b = draw.add();
    info!("to_draw2, str: {}, b = {}", a, b);
}

fn to_draw3<T: Draw + Add>(draw: &T) {
    let a = draw.draw();
    let b = draw.add();
    info!("to_draw3, str: {}, b = {}", a, b);
}

fn to_draw4(draw: Box<dyn Draw>) {
    let a = draw.draw();
    info!("to_draw4, str: {}", a);
}

struct Book {
    id: u64,
    name: String,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Add for Book {
    type Item = Book;

    fn add(&self) -> Self::Item {
        todo!()
    }
}

trait Draw {
    fn draw(&self) -> String;
}

trait Add {
    type Item: std::fmt::Display;
    fn add(&self) -> Self::Item;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Add for u8 {
    type Item = u8;

    fn add(&self) -> Self::Item {
        self + 10u8
    }
}

impl Draw for i32 {
    fn draw(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Draw for f32 {
    fn draw(&self) -> String {
        format!("f32: {}", *self)
    }
}
