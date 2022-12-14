use std::collections::{HashMap, VecDeque};

struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation_type: Operations,
    operation_value: Option<u64>,
    test_divisor: u64,
    true_case_monkey: usize,
    false_case_monkey: usize,
}

#[derive(Clone)]
enum Operations {
    Square,
    Add,
    MultiplyBy,
}
fn parse_operation(operation_line: &str) -> Option<(Operations, Option<u64>)> {
    let members = operation_line
        .split_once('=')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .collect::<Vec<&str>>();
    match (members[1].trim(), members[2].trim()) {
        ("+", value) => Some((Operations::Add, Some(value.parse::<u64>().unwrap()))),
        ("*", "old") => Some((Operations::Square, None)),
        ("*", value) => Some((Operations::MultiplyBy, Some(value.parse::<u64>().unwrap()))),
        _ => None,
    }
}
fn parse_monkey_items(items_str: &str) -> VecDeque<u64> {
    items_str
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(",")
        .map(|item_str| item_str.trim().parse::<u64>().unwrap())
        .collect::<VecDeque<u64>>()
}

fn parse_number_at_end_of_line(line: &str) -> u64 {
    line.split(" ").last().unwrap().parse::<u64>().unwrap()
}

fn parse_monkey_inventory(monkey_inv: &str) -> Monkey {
    let monkey_inventory_strs = monkey_inv.split('\n').collect::<Vec<&str>>();
    let parsed_operation = parse_operation(monkey_inventory_strs[2]);

    Monkey {
        id: monkey_inventory_strs[0]
            .replace(":", "")
            .split_once(' ')
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap(),
        items: parse_monkey_items(monkey_inventory_strs[1]),
        operation_type: parsed_operation.as_ref().unwrap().0.clone(),
        operation_value: parsed_operation.as_ref().unwrap().1.clone(),
        test_divisor: parse_number_at_end_of_line(monkey_inventory_strs[3]),
        true_case_monkey: parse_number_at_end_of_line(monkey_inventory_strs[4]) as usize,
        false_case_monkey: parse_number_at_end_of_line(monkey_inventory_strs[5]) as usize,
    }
}

fn inspect_item(item: u64, op_type: &Operations, op_value: &Option<u64>) -> u64 {
    match op_type {
        Operations::Add => item + op_value.unwrap(),
        Operations::MultiplyBy => item * op_value.unwrap(),
        Operations::Square => item * item,
    }
}

pub fn get_part1_answer(input: &str) -> String {
    let mut monkeys: HashMap<usize, Monkey> = input
        .split("\n\n")
        .map(parse_monkey_inventory)
        .map(|monkey| (monkey.id.clone(), monkey))
        .collect::<HashMap<usize, Monkey>>();

    let monkeys_count = monkeys.len();
    let mut monkey_inspection_counts: HashMap<usize, u64> = HashMap::new();

    monkeys.iter().for_each(|(id, _)| {
        monkey_inspection_counts.insert(id.clone(), 0);
    });

    for _ in 0..20 {
        for monkey_idx in 0..monkeys_count {
            let mut monkey_inv = monkeys.remove(&monkey_idx).unwrap();
            let starting_items_count = monkey_inv.items.len();

            for _ in 0..starting_items_count {
                let new_worry_level = inspect_item(
                    monkey_inv.items.pop_front().unwrap(),
                    &monkey_inv.operation_type,
                    &monkey_inv.operation_value,
                ) / 3;
                *monkey_inspection_counts.get_mut(&monkey_idx).unwrap() += 1;

                let target_monkey = if new_worry_level % monkey_inv.test_divisor == 0 {
                    monkeys.get_mut(&monkey_inv.true_case_monkey)
                } else {
                    monkeys.get_mut(&monkey_inv.false_case_monkey)
                };

                target_monkey.unwrap().items.push_back(new_worry_level);
            }

            monkeys.insert(monkey_idx, monkey_inv);
        }
    }

    let mut busiest_monkeys = monkey_inspection_counts
        .values()
        .map(|v| v.clone())
        .collect::<Vec<u64>>();
    busiest_monkeys.sort();

    String::from(
        busiest_monkeys
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, curr| acc * curr)
            .to_string(),
    )
}
pub fn get_part2_answer(input: &str) -> String {
    let mut monkeys: HashMap<usize, Monkey> = input
        .split("\n\n")
        .map(parse_monkey_inventory)
        .map(|monkey| (monkey.id.clone(), monkey))
        .collect::<HashMap<usize, Monkey>>();

    let monkeys_count = monkeys.len();
    let mut monkey_inspection_counts: HashMap<usize, u64> = HashMap::new();
    let common_dom = monkeys
        .values()
        .map(|m| m.test_divisor)
        .reduce(|acc, curr| acc * curr)
        .unwrap();

    monkeys.iter().for_each(|(id, _)| {
        monkey_inspection_counts.insert(id.clone(), 0);
    });

    for _ in 0..10000 {
        for monkey_idx in 0..monkeys_count {
            let mut monkey_inv = monkeys.remove(&monkey_idx).unwrap();
            let starting_items_count = monkey_inv.items.len();

            for _ in 0..starting_items_count {
                let new_worry_level = inspect_item(
                    monkey_inv.items.pop_front().unwrap(),
                    &monkey_inv.operation_type,
                    &monkey_inv.operation_value,
                ) % common_dom;
                *monkey_inspection_counts.get_mut(&monkey_idx).unwrap() += 1;

                let target_monkey = if new_worry_level % monkey_inv.test_divisor == 0 {
                    monkeys.get_mut(&monkey_inv.true_case_monkey)
                } else {
                    monkeys.get_mut(&monkey_inv.false_case_monkey)
                };

                target_monkey.unwrap().items.push_back(new_worry_level);
            }

            monkeys.insert(monkey_idx, monkey_inv);
        }
    }

    let mut busiest_monkeys = monkey_inspection_counts
        .values()
        .map(|v| v.clone())
        .collect::<Vec<u64>>();
    busiest_monkeys.sort();

    String::from(
        busiest_monkeys
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, curr| acc * curr)
            .to_string(),
    )
}
