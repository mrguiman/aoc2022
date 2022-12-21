use std::collections::HashSet;
use std::fs;

fn parse_section_id(str: &str) -> i32 {
    str::parse::<i32>(str).unwrap()
}

fn to_ranges(line: &str) -> Vec<(i32, i32)> {
    line.split(',')
        .map(|sections| {
            let section_bounds = sections.split_once('-').unwrap();
            (
                parse_section_id(section_bounds.0),
                parse_section_id(section_bounds.1),
            )
        })
        .collect::<Vec<(i32, i32)>>()
}
fn to_hashsets(line: &str) -> Vec<HashSet<i32>> {
    line.split(',')
        .map(|sections| {
            let bounds = sections.split_once('-').unwrap();
            HashSet::from_iter(parse_section_id(bounds.0)..=parse_section_id(bounds.1))
        })
        .collect::<Vec<HashSet<i32>>>()
}
pub fn get_part1_answer(input: &String) -> usize {
    input
        .split('\n')
        .map(to_ranges)
        .map(|ranges| {
            let mut sorted_ranges = ranges.clone();
            sorted_ranges.sort_by(|(a, b), (x, y)| if a != x { a.cmp(x) } else { y.cmp(b) });
            sorted_ranges
        })
        .filter(|ranges| ranges[1].1 <= ranges[0].1)
        .count()
}

pub fn get_part2_answer(input: &String) -> usize {
    input
        .split('\n')
        .map(to_hashsets)
        .filter(|sets| {
            sets[0].union(&sets[1]).collect::<HashSet<&i32>>().len()
                != sets[0].len() + sets[1].len()
        })
        .count()
}

fn main() {
    let input = String::from_utf8(fs::read("day4/input").unwrap()).unwrap();
    println!(
        "Day4: {}, {}",
        get_part1_answer(&input),
        get_part2_answer(&input)
    )
}
