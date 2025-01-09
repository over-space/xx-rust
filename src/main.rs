use std::{
    borrow::BorrowMut, cell::RefCell, ops::Deref, rc::{Rc, Weak}, sync::{Mutex, MutexGuard}
};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use once_cell::sync::Lazy;
use tracing::info;

#[macro_export]
macro_rules! num_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u8> for $name {
            type Error = ();

            fn try_from(v: u8) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u8 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

static B: Lazy<Mutex<Point>> = Lazy::new(|| {
    info!("lazy init");
    Mutex::new(Point { x: 1, y: 1 })
});

fn init() {
    tracing_subscriber::fmt::init();
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

// 主人
struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

// 工具
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

fn main() {
    init();
    info!("start exector...");

    info!("p = {:?}", point());
    info!("p = {:?}", point());

    let a = Rc::new(String::from("hello world!!!"));
    info!("rc count: {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    info!("rc count: {}", Rc::strong_count(&a));
    {
        let b = Rc::clone(&a);
        info!("rc count: {}", Rc::strong_count(&a));
    }
    info!("rc count: {}", Rc::strong_count(&a));

    let owner = Rc::new(Owner {
        name: "lisi".to_owned(),
        gadgets: RefCell::new(Vec::new()),
    });

    let gadget1 = Rc::new(Gadget { id: 1, owner: owner.clone() });
    let gadget2 = Rc::new(Gadget { id: 2, owner: owner.clone() });

    owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
    owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

    for ele in owner.gadgets.borrow().iter() {
        match ele.upgrade() {
            Some(gadget) => {
                println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
            },
            None => {

            },
        }
    }

    // let rc = RefCell::new(String::from("hello world!!!"));
    // let c1 = rc.borrow();
    // let c2 = rc.borrow_mut();
    // info!("c1 = {}, c2 = {}", c1, "");
}

fn point() -> MutexGuard<'static, Point> {
    B.lock().unwrap()
}

#[derive(Debug, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u32)]
enum Type {
    A = 100,
    B = 200,
    C = 300,
    D = 400,
}

num_to_enum! {
    #[derive(Debug, PartialEq, Eq)]
    #[repr(u8)]
    enum Gender{
        A = 1,
        B = 2,
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> Self {
        MyBox(v)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::{Gender, MyBox, Type};

    #[test]
    fn into() {
        let n: u32 = Type::B.into();
        assert_eq!(n, 200);
    }

    #[test]
    fn try_from() {
        let n = Type::try_from(400u32);
        assert_eq!(n, Ok(Type::D));

        let b = Gender::try_from(2u8);
        assert_eq!(b, Ok(Gender::B));
    }

    #[test]
    fn try_into() {
        let n: Result<u32, _> = Type::A.try_into();
        assert_eq!(n, Ok(100));

        let a = Gender::A as u8;
        assert_eq!(a, 1u8);
    }

    #[test]
    fn deref() {
        let x = 10;
        let y = &x;
        let z = y + 1;
        assert_eq!(10, x);
        assert_eq!(10, *y);
        assert_eq!(11, z);

        let a = Box::new(10);
        let b = *a + 10;
        assert_eq!(*a, 10);
        assert_eq!(b, 20);
    }

    #[test]
    fn my_box() {
        let a = MyBox::new(11);
        let b = *a + 10;
        assert_eq!(*a, 11);
        assert_eq!(b, 21);
    }
    #[test]

    fn cell() {
        let a = Cell::new("hello world!!!");
        let s1 = a.get();
        a.set("world hello");
        let s2 = a.get();
        assert_eq!(s1, "hello world!!!");
        assert_eq!(s2, "world hello");
    }
}

