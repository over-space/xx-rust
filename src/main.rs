fn main(){

    let v = "hello world".to_string();

    let s1 = &v[..5];
    let s2 = &v[6..];

    println!("{:?}", s1);
    println!("{:?}", s2);


    let list = vec![1, 2, 3, 4, 5, 6];

    let l1 = &list[2..3];
    println!("{:?}", l1);
}
