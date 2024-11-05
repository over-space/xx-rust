use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node{
    id: usize,
    next: Option<Rc<Node>>,
}

fn main() {
    let v = RefCell::new(1);
    {
        let mut ref_mut = v.borrow_mut();
        
        // 解引用
        *ref_mut += 1;
    }
    println!("{:?}", v.borrow()); // Output: 2
}

