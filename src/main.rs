use std::{fs, string::FromUtf8Error};

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    );

    let day5_input = parse_full_input("assets/day5_puzzle1_input.txt").unwrap();
    println!(
        "Day5: {:?}, {:?}",
        day5::get_part1_answer(&day5_input),
        day5::get_part2_answer(&day5_input)
    );

    let day6_input = parse_full_input("assets/day6_puzzle1_input.txt").unwrap();
    println!(
        "Day6: {:?}, {:?}",
        day6::get_part1_answer(&day6_input),
        day6::get_part2_answer(&day6_input)
    );

    let day7_input = parse_full_input("assets/day7_puzzle_input.txt").unwrap();
    println!(
        "Day7: {:?}, {:?}",
        day7::get_part1_answer(&day7_input),
        day7::get_part2_answer(&day7_input)
    );

    // let day8_input = parse_full_input("assets/day8_puzzle_input.txt").unwrap();
    // println!(
    //     "Day8: {:?}, {:?}",
    //     day8::get_part1_answer(&day8_input),
    //     day8::get_part2_answer(&day8_input)
    // );

    let day9_input = parse_full_input("assets/day9_puzzle_input.txt").unwrap();
    println!(
        "Day9: {:?}, {:?}",
        day9::get_part1_answer(&day9_input),
        day9::get_part2_answer(&day9_input)
    );

    let day10_input = parse_full_input("assets/day10_puzzle_input.txt").unwrap();
    println!("Day10: {:?}", day10::get_part1_answer(&day10_input));
    println!("{}", day10::get_part2_answer(&day10_input))
}
