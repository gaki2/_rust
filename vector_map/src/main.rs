use std::collections::HashSet;

fn iter_and_map() {
    let a = vec![1, 2, 3];
    /**
     * iter() 은 a 를 참조하는 Iterator 를 만들어 리턴한다.
     * map() 은 Iterator 의 값을 clone 하여 해당 값으로 closure 를 실행해 새로운 Iterator 를 만든다.
     * 즉, a.iter() 의 Iterator 와  a.iter().map() 이후의 Iterator 는 다른 Iterator 이다.
     */
    let mut iter = a.iter().map(|x| x + 1);

    assert_eq!(iter.next(), Some(2));
    println!("{:?}", a);
}

#[cfg(test)]
fn iter_and_map_for_wrong() {
    // 이렇게하면 안되는 이유는, iter 라는 변수는 Iterator 를 가지고 있다.
    // 즉 원재료인 vec![1,2,3] 은 아무에게도 소유되고 있지 않아 free 되어 에러가 발생한다.
    let iter = vec![1, 2, 3].iter().map(|x| x + 1);

    while let Some(x) = iter.next() {
        println!("{}", x);
    }
}

fn iter_and_map_for_good() {
    let a = vec![1, 2, 3];
    // x 는 &i32 인데 + 연산이 가능한건지 궁금했었다.
    // 찾아보니 Rust 에서 &i32 타입에 대해서 Add Trait 을 구현해놓았기 때문에
    // &i32 와 i32 가 덧셈연산이 가능한 것이였다.
    let mut iter = a.iter().map(|x| x + 1);

    while let Some(x) = iter.next() {
        println!("{}", x);
    }
}

fn collect() {
    let a = vec![1, 2, 3, 3];

    let b: HashSet<i32> = a.into_iter().collect();

    for item in &b {
        println!("{}", item);
    }
}

fn add_with_ref() {
    let a = 5;
    let b = &a;

    let c = b + 1;
    println!("{}", c);
}

fn fs_iterator() {
    let text = std::fs::read_to_string("text").unwrap();
    text.lines().for_each(|x| println!("{}", x))
}

fn fs_iterator_filter() {
    let text = std::fs::read_to_string("text").unwrap();
    text.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line))
}

fn main() {
    fs_iterator_filter();
}
