use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print_answer() {
    let file = File::open("assets/day1_puzzle1_input.txt").unwrap();
    let f = BufReader::new(file);

    let mut max_calories: Vec<i32> = vec![0, 2, 4];
    let mut calories_count: i32 = 0;

    for line_result in f.lines() {
        let line: String = line_result.unwrap();
        if line.len() > 0 {
            calories_count += line.parse::<i32>().unwrap();
        } else {
            let insert_index = max_calories.iter().position(|&x| x <= calories_count);
            match insert_index {
                Some(idx) => {
                    max_calories.insert(idx, calories_count);
                    max_calories = max_calories[0..3].to_vec();
                }
                _ => (),
            }
            calories_count = 0;
        }
    }

    println!(
        "Max Calories: {:?}, {}",
        max_calories.clone(),
        max_calories
            .into_iter()
            .reduce(|acc, curr| acc + curr)
            .unwrap()
    );
}
