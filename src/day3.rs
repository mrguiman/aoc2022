use std::str;

static CHARS: [char; 53] = [
    ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn find_duplicate(items: &str, rucksacks: Vec<&str>) -> Option<char> {
    items.chars().find(|character| {
        rucksacks
            .iter()
            .filter(|sack| sack.contains([*character]))
            .collect::<Vec<&&str>>()
            .len()
            == rucksacks.len()
    })
}
fn to_target_item_value(items: &str, rucksacks: Vec<&str>) -> i32 {
    match find_duplicate(items, rucksacks) {
        Some(character) => CHARS.iter().position(|x| *x == character).unwrap_or(0) as i32,
        _ => 0,
    }
}

fn split_in_half(value: &str) -> (&str, &str) {
    let splits = value
        .as_bytes()
        .chunks(value.len() / 2)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    (splits[0], splits[1])
}
pub fn get_part1_answer(input: &String) -> i32 {
    input
        .split('\n')
        .map(split_in_half)
        .map(|item_groups| to_target_item_value(item_groups.0, vec![item_groups.1]))
        .sum::<i32>()
}

pub fn get_part2_answer(input: &String) -> i32 {
    let rucksacks = input.split('\n').collect::<Vec<&str>>();

    group_by(rucksacks, 3)
        .iter()
        .map(|rucksacks| to_target_item_value(rucksacks[0], vec![rucksacks[1], rucksacks[2]]))
        .sum()
}

fn group_by<T: Clone>(vector: Vec<T>, count: usize) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = vec![];
    for i in (0..vector.len()).step_by(count) {
        result.push(vec![
            vector[i].clone(),
            vector[i + 1].clone(),
            vector[i + 2].clone(),
        ])
    }
    result
}
