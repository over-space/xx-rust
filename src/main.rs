use std::rc::Rc;

#[derive(Debug)]
struct Node{
    id:usize,
    next:Option<Rc<Node>>,
}

impl Node {

    fn new(id: usize) -> Self {
        Self{
            id,
            next: None,
        }
    }    

    fn update_next(&mut self, next: Rc<Node>){
        self.next = Some(next);
    }

    fn get_next(&self) -> Option<Rc<Node>>{
        self.next.as_ref().map(|v| v.clone())
    }

}

fn main() {
    
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);

    let node4 = Node::new(4);
    node3.update_next(Rc::new(node4));
    node1.update_next(Rc::new(node3));
    node2.update_next(node1.get_next().unwrap());
    println!("node1: {:?}", node1);
    println!("node2: {:?}", node2);
}
