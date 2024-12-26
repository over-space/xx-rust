use std::any::Any;

fn main() {
    let animal = random_animal(12);

    if let Some(dog) = animal.downcast_ref::<Dog>() {
        println!("i'm is a dog, animal color: {}", Color::get(dog)); 
        println!("i'm is a dog, animal sound: {}", Sound::get(dog)); 
    } else if let Some(cat) = animal.downcast_ref::<Cat>() {
        println!("i'm is a cat, animal color: {}", Color::get(cat)); 
        println!("i'm is a cat, animal sound: {}", Sound::get(cat)); 
    } else {
        println!("animal2 is not Dog or Cat");
    }
}

struct Dog {}
struct Cat {}

trait Animal {
    fn jump(&self) -> &str;
}

trait Color {
    fn get(&self) -> &str;
}

trait Sound {
    fn get(&self) -> &str;
}

impl Animal for Dog {
    fn jump(&self) -> &str {
        "hello dog!!!"
    }
}

impl Color for Dog {
    fn get(&self) -> &str {
        "黑色"
    } 
 }

 impl Sound for Dog {
    fn get(&self) -> &str {
        "旺旺旺~~~"
    } 
 }

impl Animal for Cat {
    fn jump(&self) -> &str {
        "hello cat!!!"
    }
}

impl Color for Cat {
    fn get(&self) -> &str {
        "橘色"
    } 
 }

 impl Sound for Cat {
    fn get(&self) -> &str {
        "喵喵喵~~~"
    } 
 }

fn random_animal(num: u32) -> Box<dyn Any> {
    if num < 10 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
    }
}
