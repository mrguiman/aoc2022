use std::str;

fn get_relevant_characters(line: &str) -> Vec<String> {
    line.as_bytes()
        .chunks(4)
        .map(str::from_utf8)
        .map(|chunk| match chunk {
            Ok(result) => result.trim().replace("[", "").replace("]", ""),
            _ => String::from(""),
        })
        .collect::<Vec<String>>()
}

fn parse_instructions(input: &str) -> Vec<usize> {
    input
        .replace("move ", "")
        .replace(" from ", ",")
        .replace(" to ", ",")
        .split(",")
        .fold(Vec::<usize>::new(), |mut acc, str| {
            let n: usize = str.parse().unwrap();
            acc.push(n);
            acc
        })
}

fn get_initial_stacks(stacks_parts: &str) -> Vec<Vec<String>> {
    let mut lines = stacks_parts.split('\n').collect::<Vec<&str>>();
    let mut stacks = get_relevant_characters(lines.pop().unwrap())
        .iter()
        .map(|_| Vec::<String>::new())
        .collect::<Vec<Vec<String>>>();

    lines.iter().rev().for_each(|line| {
        get_relevant_characters(*line)
            .iter()
            .enumerate()
            .for_each(|(idx, character)| {
                if character.len() > 0 {
                    stacks[idx].push(character.clone())
                };
            })
    });
    stacks
}

fn part_1_get_crates_from_stack(
    stack_to_move_from: &mut Vec<String>,
    crates_to_move: usize,
) -> Vec<String> {
    stack_to_move_from
        .drain(stack_to_move_from.len() - crates_to_move..=stack_to_move_from.len() - 1)
        .rev()
        .collect::<Vec<String>>()
}

fn part_2_get_crates_from_stack(
    stack_to_move_from: &mut Vec<String>,
    crates_to_move: usize,
) -> Vec<String> {
    stack_to_move_from
        .drain(stack_to_move_from.len() - crates_to_move..=stack_to_move_from.len() - 1)
        .collect::<Vec<String>>()
}

fn get_answer(
    input: &String,
    take_crates: impl Fn(&mut Vec<String>, usize) -> Vec<String>,
) -> String {
    let (stacks_parts, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = get_initial_stacks(stacks_parts);
    let parsed_instructions = instructions
        .split('\n')
        .map(parse_instructions)
        .collect::<Vec<Vec<usize>>>();

    // Apply instructions to the stacks
    parsed_instructions.iter().for_each(|instruction| {
        let crates_to_move = instruction[0];
        let from_index = instruction.get(1).unwrap();
        let to_index = *instruction.get(2).unwrap();
        let stack_to_move_from = &mut stacks[*from_index - 1];
        let crates_to_move = take_crates(stack_to_move_from, crates_to_move);

        stacks
            .get_mut(to_index - 1)
            .unwrap()
            .extend_from_slice(&crates_to_move);
    });

    // Print last element of stacks together in a single string
    stacks.iter().fold(String::new(), |acc, vec| {
        String::from(format!("{}", acc.clone() + vec.last().unwrap()))
    })
}

pub fn get_part1_answer(input: &String) -> String {
    get_answer(input, part_1_get_crates_from_stack)
}
pub fn get_part2_answer(input: &String) -> String {
    get_answer(input, part_2_get_crates_from_stack)
}
