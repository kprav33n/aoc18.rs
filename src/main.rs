extern crate aoc18;

use std::env;
use std::io::{self, Read};

fn read_stdin_and_report_result<T: std::fmt::Display>(f: fn(&str) -> T) {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => println!("{}", f(&buffer)),
        Err(e) => println!("Failed to read from STDIN: {}", e),
    }
}

fn main() {
    let command = &env::args().nth(1).unwrap() as &str;
    match command {
        "day01a" => read_stdin_and_report_result(aoc18::day01::resulting_frequency),
        "day01b" => read_stdin_and_report_result(aoc18::day01::resulting_frequency_twice),
        "day02a" => read_stdin_and_report_result(aoc18::day02::checksum),
        "day02b" => read_stdin_and_report_result(aoc18::day02::common_letters),
        "day03a" => read_stdin_and_report_result(aoc18::day03::overlapping_area),
        "day03b" => read_stdin_and_report_result(aoc18::day03::intact_claim),
        "day04a" => read_stdin_and_report_result(aoc18::day04::slacker_id_min),
        "day04b" => read_stdin_and_report_result(aoc18::day04::slacker_id_min2),
        _ => println!("Unknown command: {}", command),
    }
}
