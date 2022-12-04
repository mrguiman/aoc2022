use std::{fs, string::FromUtf8Error};

mod day1;
mod day2;
mod day3;
mod day4;

fn parse_full_input(path: &str) -> Result<String, FromUtf8Error> {
    String::from_utf8(fs::read(path).unwrap())
}
fn main() {
    println!("Hello, AOC2022 !");
    println!("Day1: {:?}", day1::get_answer());

    let day2_input = parse_full_input("assets/day2_puzzle1_input.txt").unwrap();
    println!(
        "Day2: {:?}, {:?}",
        day2::get_part1_answer(&day2_input),
        day2::get_part2_answer(&day2_input)
    );
    let day3_input = parse_full_input("assets/day3_puzzle1_input.txt").unwrap();
    println!(
        "Day3: {:?}, {:?}",
        day3::get_part1_answer(&day3_input),
        day3::get_part2_answer(&day3_input)
    );
    let day4_input = parse_full_input("assets/day4_puzzle1_input.txt").unwrap();
    println!(
        "Day4: {:?}, {:?}",
        day4::get_part1_answer(&day4_input),
        day4::get_part2_answer(&day4_input)
    )
}
