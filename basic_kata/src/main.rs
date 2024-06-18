mod temp;

fn parse() {
    let apple = "1";
    let int = apple.parse::<u32>().unwrap();
    println!("{}", int);
}

fn main() {
    let a: u32 = 0b1001011001111111;
    println!("{}", a as u8);
}