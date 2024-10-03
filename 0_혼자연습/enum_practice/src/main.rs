enum Color {
    Red,
    Green,
    Blue,
}

fn get_color(color: Color) {
    match color {
        Color::Red => todo!(),
        Color::Green => todo!(),
        Color::Blue => todo!(),
    }
}

struct Foo {
    name: String,
    age: usize,
}

enum Item {
    String(String),
    Number(usize),
    Foo(Foo),
}

fn push_item(vec: &mut Vec<Item>) {
    vec.push(Item::String("Fem".to_string()));
}

fn main2() {
    let mut v = vec![Item::Number(10)];

    push_item(&mut v);
}

fn main3() {
    let item = Item::String("hi".to_string());

    match item {
        Item::Foo(foo) => {
            println!("{}", foo.age);
        }
        _ => {}
    }

    match item {
        Item::Foo(Foo { name, .. }) => {
            println!("My name is {}", name);
        }
        Item::Number(num) => {
            println!("{}", num);
        }
        _ => {}
    }

    match item {
        Item::Foo(foo) if foo.age > 15 => {
            println!("Not child")
        }
        Item::Foo(foo) if foo.age <= 15 => {
            println!("child");
        }
        Item::Number(num) => {}
        Item::String(str) => {}
        _ => {}
    }
}

fn main() {}
