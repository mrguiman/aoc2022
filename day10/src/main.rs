use std::fs;
use std::str;

fn map_cycles(input: &str) -> Vec<(u32, i32)> {
    let mut values: Vec<(u32, i32)> = vec![(0, 1)];

    input.split('\n').for_each(|instruction| match instruction {
        "noop" => {
            let last_cycle = values[values.len() - 1];
            values.push((last_cycle.0 + 1, last_cycle.1))
        }
        add_instr => {
            let last_cycle = values[values.len() - 1];
            let (_, value) = add_instr.split_once(' ').unwrap();
            values.push((last_cycle.0 + 1, last_cycle.1));
            values.push((
                last_cycle.0 + 2,
                last_cycle.1 + value.parse::<i32>().unwrap(),
            ));
        }
    });

    values
}

pub fn get_part1_answer(input: &str) -> String {
    let strength_cycles = [20, 60, 100, 140, 180, 220];
    map_cycles(input)
        .iter()
        .filter(|(cycle_idx, _)| strength_cycles.contains(&(cycle_idx)))
        .map(|(cycle, strength)| *cycle as i32 * strength)
        .sum::<i32>()
        .to_string()
}

pub fn get_part2_answer(input: &str) -> String {
    let cycle_values = map_cycles(input);
    let mut screen = String::new();
    let width = 40;
    let height = 6;

    for (cycle, sprite_pos) in cycle_values {
        let drawn_pixel = cycle - (&width * (cycle / &width));
        if (sprite_pos - 1..=sprite_pos + 1).contains(&(drawn_pixel as i32)) {
            screen += "#";
        } else {
            screen += ".";
        }
    }

    screen
        .as_bytes()
        .chunks(width as usize)
        .map(str::from_utf8)
        .map(|f| format!("{}", f.unwrap()))
        .take(height)
        .fold(String::new(), |acc, str| acc + &str + "\n")
}

fn main() {
    let input = String::from_utf8(fs::read("day10/input").unwrap()).unwrap();
    println!(
        "Day10: {}, {}",
        get_part1_answer(&input),
        get_part2_answer(&input)
    )
}
