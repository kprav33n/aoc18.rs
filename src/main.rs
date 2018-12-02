extern crate aoc18;

use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => println!("{}", aoc18::day01::resulting_frequency(&buffer)),
        Err(e) => println!("Failed to read from STDIN: {}", e),
    }
}
