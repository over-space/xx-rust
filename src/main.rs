fn main() {
    println!("Hello world!!!");

    let a = gift(None);
    let b = gift(Some("cat"));
    let c = gift(Some("snake"));
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    a.map(|f| println(f));

    let d = b.map_or("default", |f| "google.com");
    println(d);
    let d = b.map_or("default", |f| "google.com");
    println(d);

    let fun = |a| println(a);
    a.map(fun);
    b.map(fun);
    c.map(fun);
}

fn println(v:&str){
    println!("v = {}", v);
}

fn gift(gift: Option<&str>) -> Option<&str>{
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(v) => println!("{}? How nice.", v),
        None => println!("No gift selected."),
    }
    // gift.expect("not gift");
    let a = gift?;
    Some(a)
}
