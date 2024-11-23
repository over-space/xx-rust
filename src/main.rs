use std::thread;

#[derive(Debug)]
struct User {
    name: String,
    score: u32,
}

impl User {
    fn new(name: String, score: u32) -> Self {
        User { name, score }
    }
}

fn main() {
    println!("Hello world!!!");

    let add = |a, b| 2 * (a + b);

    let a = add(1, 2);

    println!("a = {}", a);

    let a = fun_closure(add);

    println!("a = {}", a);

    let parse = |s: &str| s.parse::<i32>().unwrap();

    let s = "5".to_owned();
    let a = parse(&s);
    println!("a = {}", a);

    let u1 = User::new("lisi".to_owned(), 80);
    let u2 = User::new("lisi".to_owned(), 60);
    let u3 = User::new("lisi".to_owned(), 90);
    let u4 = User::new("lisi".to_owned(), 50);

    let mut users = vec![u1, u2, u3, u4];

    // sort_user(&mut users);
    sort_user_closure(&mut users);

    println!("users : {:?}", users);

    {
        let s1 = "hello".to_owned();
        let s2 = "world".to_owned();

        let fn_closure = || {
            println!("fn_closure s1 = {}", s1);
            println!("fn_closure s2 = {}", s2);
        };
        fn_closure();

        println!("s1 = {}", s1);
        println!("s2 = {}", s2);
    }

    {
        let mut s1 = "hello".to_string();
        let mut s2 = "world".to_owned();

        let mut fn_closure = |x: String| {
            println!("fn_closure s1 = {}", s1);
            println!("fn_closure s2 = {}", s2);
            println!("fn_closure x = {}", x);
            s1.push_str("🤣");
            s2.push_str(&x);
        };

        fn_closure(" abc".to_string());

        println!("s1 = {}", s1);
        println!("s2 = {}", s2);
    }

    {
        let  s1 = "hello".to_string();
        let fn_once_closure = |x| {
            println!("fn_closure s1 = {}", s1);
            std::mem::drop(s1);
            println!("fn_closure x = {}", x);
        };
        fn_once_closure("closure".to_owned());
        // fn_once_closure("closure".to_owned());
    }

    {
        let  s1 = "hello".to_string();
        thread::spawn(move || {
            println!("thread s1 = {}", s1);
        }).join();
    }
}

fn sort_user_closure(users: &mut Vec<User>) {
    users.sort_by_key(|user| user.score)
}

fn sort_user(users: &mut Vec<User>) {
    users.sort_by_key(sort_user_key);
}
fn sort_user_key(user: &User) -> u32 {
    user.score
}

fn fun_closure<F>(closure: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    closure(5, 7)
}
