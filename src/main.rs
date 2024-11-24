

#[derive(Debug)]
struct User{
    name: String,
    age: u8,
    email: String,
}

struct UserBuilder{
    name: String,
    age: u8,
    email: String,
}

impl User {

    fn new() -> Self{
        User{
            name: String::new(),
            age: 0,
            email: String::new(),
        }
    }

    fn name(mut self, name:String) -> Self{
        self.name = name;
        self
    }

    fn age(mut self, age:u32) -> Self{
        self.age = age as u8;
        self
    }

    fn email(mut self, email:String) -> Self{
        self.email = email;
        self
    }

    fn build(self) -> Self{
        self
    }
}

impl UserBuilder {

    fn new() -> Self{
        UserBuilder{
            name: String::new(),
            age: 0,
            email: String::new(),
        }
    }

    fn name(mut self, name:String) -> Self{
        self.name = name;
        self
    }

    fn age(mut self, age:u32) -> Self{
        self.age = age as u8;
        self
    }

    fn email(mut self, email:String) -> Self{
        self.email = email;
        self
    }

    fn build(self) -> User{
        User{
            name: self.name,
            age: self.age,
            email: self.email,
        } 
    }
}


fn main() {
    let user1 = User::new()
        .name("lisi".to_string())
        .age(20)
        .email("hello@gmail.com".to_string())
        .build();
    println!("{:?}", user1);

    let user2 = UserBuilder::new()
        .name("zhansan".to_string())
        .age(32)
        .email("zhansan@gmail.com".to_string())
        .build();
    println!("{:?}", user2);
}