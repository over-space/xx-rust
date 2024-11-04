#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
enum Gender { 
    Unspecified = 0,
    Female = 1, 
    Male = 2,
}

// #[derive(Debug)]
#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    email: String,
    gender: Gender
}

fn main() {
    let mut user = User {
        id: UserId(1),
        name: String::from("John Doe"),
        email: String::from("johndoe@example.com"),
        gender: Gender::Male,
    };

    
    println!("user:{:?}", user);
    
    
    let a = String::from("hello");
    let b = a;
    println!("a:{}", b); // a is moved, not copied
    
    let data = vec![1, 2, 3];
    let data1 = &data;

    println!( "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}", 
              &data, data1, &&data, &data1 );
    
    let mut data2 = Vec::new();
    let x = 42;
    data2.push(&x);
    println!("data2:{:?}, x: {}", data2, x);    
}

