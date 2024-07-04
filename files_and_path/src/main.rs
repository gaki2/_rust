use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("lone222");
    let file = File::open(path);

    match file {
        Ok(v) => {
            println!("ok");
        }
        Err(_) => {
            println!("error");
        }
    }
}
