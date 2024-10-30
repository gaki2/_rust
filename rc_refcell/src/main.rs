#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let n3 = Rc::new(RefCell::new(Node { val: 3, next: None }));
    let n2 = Node {
        val: 2,
        next: Some(Rc::clone(&n3)),
    };
    let n1 = Node {
        val: 1,
        next: Some(Rc::clone(&n3)),
    };
    dbg!(&n1);
    dbg!(&n2);
    n3.borrow_mut().val = 1;
    dbg!(&n1);
    dbg!(&n2);
}
// https://www.youtube.com/watch?v=KYJ95TxEC18
