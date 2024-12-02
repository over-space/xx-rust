use std::{sync::{Arc, Mutex, RwLock}, thread};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    {
        // 能正常执行，
        // Rust 1.82.0 的捕获优化（字段捕获）允许闭包只捕获 p.x 而不是整个 p，
        // 因此避免了所有权冲突的问题。这是编译器优化的一部分，而非运行时的特殊行为。
        let p = Point { x: 11, y: 10 };

        let t1 = thread::spawn(move || {
            println!("x = {}", p.x);
        });

        let t2 = thread::spawn(move || {
            println!("y = {}", p.y);
        });

        t2.join().unwrap();
        t1.join().unwrap();
    }

    {
        // Arc 引用计数法
        // 多线程共享读
        let p = Arc::new(Point { x: 12, y: 10 });

        let p1 = Arc::clone(&p);
        let t1 = thread::spawn(move || {
            println!("p = {:?}", p1);
        });

        let p2 = Arc::clone(&p);
        let t2 = thread::spawn(move || {
            println!("p = {:?}", p2);
        });

        t2.join().unwrap();
        t1.join().unwrap();
    }
    {
        let p = Arc::new(Mutex::new(Point{x: 13, y: 10 }));

        let p1 = Arc::clone(&p);
        let t1 = thread::spawn(move || {
            let mut p1 = p1.lock().unwrap();
            p1.x += 1;
            println!("p = {:?}", p1);
        });

        let p2 = Arc::clone(&p);
        let t2 = thread::spawn(move || {
            let mut p1 = p2.lock().unwrap();
            p1.x += 1;
            println!("p = {:?}", p1);
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }

    {
        let p = Arc::new(RwLock::new(Point{x: 20, y: 10}));

        let p1 = Arc::clone(&p);
        let t1 = thread::spawn(move || {
            let mut p1 = p1.try_write().unwrap();
            p1.x += 1;
            println!("p = {:?}", p1);
        });

        let p2 = Arc::clone(&p);
        let t2 = thread::spawn(move || {
            let p1 = p2.try_read().unwrap();
            println!("p = {:?}", p1);
        });

        let p3 = Arc::clone(&p);
        let t3 = thread::spawn(move || {
            let mut p1 = p3.try_write().unwrap();
            p1.x += 1; 
            println!("p = {:?}", p1);
        });
        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();
    }
}
