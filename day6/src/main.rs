use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::hash::Hash;

fn unique_count<T: Hash + Eq>(v: &VecDeque<T>) -> usize {
    let mut uniques = HashSet::new();
    v.iter().for_each(|e| {
        uniques.insert(e);
    });
    uniques.len()
}

fn get_first_marker_bound(input: &str, marker_char_count: usize) -> Option<String> {
    let mut buf = VecDeque::new();

    for (idx, char) in input.chars().enumerate() {
        if buf.len() == marker_char_count {
            buf.pop_front();
        }
        buf.push_back(char);
        if unique_count(&buf) == marker_char_count {
            return Some((idx + 1).to_string());
        }
    }
    None
}

pub fn get_part1_answer(input: &str) -> Option<String> {
    get_first_marker_bound(input, 4)
}
pub fn get_part2_answer(input: &str) -> Option<String> {
    get_first_marker_bound(input, 14)
}

fn main() {
    let input = String::from_utf8(fs::read("day6/input").unwrap()).unwrap();
    println!(
        "Day6: {}, {}",
        get_part1_answer(&input).unwrap(),
        get_part2_answer(&input).unwrap()
    )
}
