use std::io::{self, Read};

fn main() {
    for byte in io::stdin().bytes() {
        let c = byte.unwrap();
        println!("{c}");
    }
}
