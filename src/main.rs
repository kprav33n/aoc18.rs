use aoc18;
use std::env;
use std::io::{self, Read};
use std::result::Result;

fn read_stdin_and_report_result<T: std::fmt::Display>(f: fn(&str) -> T) {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => println!("{}", f(&buffer)),
        Err(e) => println!("failed to read from STDIN: {}", e),
    }
}

fn read_stdin_and_return_result<T>(f: fn(&str) -> T) -> Result<T, String> {
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => Ok(f(&buffer)),
        Err(e) => Err(format!("Failed to read from STDIN: {}", e)),
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
        "day05a" => read_stdin_and_report_result(aoc18::day05::num_remaining_units),
        "day05b" => read_stdin_and_report_result(aoc18::day05::smallest_num_remaining_units),
        "day06a" => read_stdin_and_report_result(aoc18::day06::largest_area),
        "day06b" => read_stdin_and_report_result(|x| aoc18::day06::safe_area(x, 10000)),
        "day07a" => read_stdin_and_report_result(aoc18::day07::steps),
        "day07b" => read_stdin_and_report_result(|x| aoc18::day07::completion_time(x, 5, 60)),
        "day08a" => read_stdin_and_report_result(aoc18::day08::meta_sum),
        "day08b" => read_stdin_and_report_result(aoc18::day08::root_value),
        "day09a" => read_stdin_and_report_result(aoc18::day09::winning_score),
        "day09b" => read_stdin_and_report_result(aoc18::day09::winning_score2),
        "day10a" => match read_stdin_and_return_result(aoc18::day10::message) {
            Ok((s, _)) => println!("{}", s),
            Err(e) => println!("error: {}", e),
        },
        "day10b" => match read_stdin_and_return_result(aoc18::day10::message) {
            Ok((_, n)) => println!("{}", n),
            Err(e) => println!("error: {}", e),
        },
        "day11a" => read_stdin_and_report_result(aoc18::day11::largest_powered_cell),
        "day11b" => match read_stdin_and_return_result(aoc18::day11::largest_powered_cell2) {
            Ok((p, s)) => println!("{},{},{}", p.x, p.y, s),
            Err(e) => println!("error: {}", e),
        },
        "day12a" => read_stdin_and_report_result(|x| aoc18::day12::sum_pots_after(x, 20)),
        "day12b" => {
            read_stdin_and_report_result(|x| aoc18::day12::sum_pots_after(x, 50_000_000_000))
        }
        "day13a" => read_stdin_and_report_result(aoc18::day13::first_crash),
        "day13b" => read_stdin_and_report_result(aoc18::day13::last_cart_location),
        "day14a" => read_stdin_and_report_result(aoc18::day14::score_after),
        "day14b" => read_stdin_and_report_result(aoc18::day14::num_recipes_before),
        "day16a" => read_stdin_and_report_result(aoc18::day16::num_samples),
        "day16b" => read_stdin_and_report_result(aoc18::day16::result_r0),
        _ => println!("Unknown command: {}", command),
    }
}
