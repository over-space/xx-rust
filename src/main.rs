use std::fs;

fn main() {
    println!("apply square : {}", apply(2, square));
    println!("apply cube : {}", apply(3, cube));
}

fn apply(v: i32, fun: fn(i32) -> i32) -> i32{
    fun(v)
}

fn square(v: i32) -> i32{
    v * v
}

fn cube(v: i32) -> i32{
    v * v * v
}
