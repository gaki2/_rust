use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    // deref 는 * 연산자를 이용하여 접근하려는 값의 참조자를 반환한다.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn print_something(text: &str) {
    println!("{text}");
}

fn main() {
    let my_box = MyBox::new(String::from("Hello"));
    print_something(&my_box);
}
