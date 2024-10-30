## Box 를 사용하는 경우

```rust
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let n3 = Box::new(Node { val: 3, next: None });
    let n2 = Node {
        val: 2,
        next: Some(n3),
    };
    let n1 = Node {
        val: 1,
        next: Some(n3),
            // ^^ value used here after move
    };
}

```

## Rc 를 사용하는 경우

```rust
use std::rc::Rc;

struct Node {
    val: i32,
    next: Option<Rc<Node>>,
}

fn main() {
    let n3 = Rc::new(Node { val: 3, next: None });
    let n2 = Node {
        val: 2,
        next: Some(Rc::clone(&n3)),
    };
    let n1 = Node {
        val: 1,
        next: Some(Rc::clone(&n3)),
    };
}
```

## Rc 개수 카운트

```rust
fn main() {
    let n3 = Rc::new(Node { val: 3, next: None });
    dbg!(Rc::strong_count(&n3));
    let n2 = Node {
        val: 2,
        next: Some(Rc::clone(&n3)),
    };
    dbg!(Rc::strong_count(&n3));
    {
        let n1 = Node {
            val: 1,
            next: Some(Rc::clone(&n3)),
        };
        dbg!(Rc::strong_count(&n3));
    }
    dbg!(Rc::strong_count(&n3));
}
```

## RefCell

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    dbg!(&x);

    *x.borrow_mut() += 1;
    dbg!(&x);
}
```