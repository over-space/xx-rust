fn main(){
    let v = factorial(10);
    println!("v = {}", v)
}

fn factorial(n:u32) -> u32{
    if n == 0 || n == 1 {
        1
    }else {
        n * factorial(n - 1)
    }
}