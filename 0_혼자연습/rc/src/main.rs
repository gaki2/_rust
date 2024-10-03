enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// rc 는 불변성을 유지하는 조건 하에 여러 변수가 Rc 를 동시에 참조할 수 있다.
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating a = {}", Rc::strong_count(&a));
    }
    println!("count after creating a = {}", Rc::strong_count(&a));
}
