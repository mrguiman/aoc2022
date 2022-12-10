#![allow(unused)]
use std::cmp;
use std::str;

fn is_tree_visible_in_row(row: &str, tree: u32, tree_index: usize) -> bool {
    let mut left_visible = true;
    let mut right_visible = true;

    let chars = row
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    for c in &chars[0..tree_index] {
        if c >= &tree {
            left_visible = false;
        }
    }

    for c in &chars[tree_index + 1..chars.len()] {
        if c >= &tree {
            right_visible = false;
        }
    }

    left_visible || right_visible
}

fn get_col_characters(lines: &Vec<&str>, col_idx: &usize) -> String {
    lines
        .iter()
        .enumerate()
        .map(|(_, line)| {
            line.chars()
                .enumerate()
                .find(|(i, _)| i == col_idx)
                .unwrap()
        })
        .fold(String::new(), |mut acc, (_, c)| {
            acc.push(c);
            acc
        })
}

fn calculate_scenic_score_for_row(row: &str, tree: u32, tree_index: usize) -> u32 {
    let mut left_score: u32 = 0;
    let mut right_score: u32 = 0;

    let chars = row
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    if tree_index > 0 {
        let left_chars = &mut chars.clone()[0..tree_index];
        left_chars.reverse();
        for c in left_chars {
            left_score += 1;
            if *c >= tree {
                break;
            }
        }
    }

    if tree_index < chars.len() - 1 {
        for c in &chars[tree_index + 1..chars.len()] {
            right_score += 1;
            if c >= &tree {
                break;
            }
        }
    }

    left_score * right_score
}

fn is_tree_visible(tree: u32, tree_pos: (usize, usize), lines: &Vec<&str>) -> bool {
    let max_x_pos = lines[0].len() - 1;
    if tree_pos.0 == 0
        || tree_pos.1 == 0
        || tree_pos.0 == max_x_pos
        || tree_pos.1 == lines.len() - 1
    {
        true
    } else {
        if is_tree_visible_in_row(lines[tree_pos.1], tree, tree_pos.0) {
            true
        } else {
            let char_col = get_col_characters(lines, &tree_pos.0);
            is_tree_visible_in_row(&char_col, tree, tree_pos.1)
        }
    }
}

fn get_tree_scenic_score(tree: u32, tree_pos: (usize, usize), lines: &Vec<&str>) -> u32 {
    let char_col = get_col_characters(lines, &tree_pos.0);
    let char_line = lines[tree_pos.1];
    calculate_scenic_score_for_row(&char_col, tree, tree_pos.1)
        * calculate_scenic_score_for_row(char_line, tree, tree_pos.0)
}

pub fn get_part1_answer(input: &str) -> String {
    let mut visible_trees: u32 = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    lines.iter().enumerate().for_each(|(line_idx, line)| {
        line.chars().enumerate().for_each(|(char_idx, char)| {
            if is_tree_visible(char.to_digit(10).unwrap(), (char_idx, line_idx), &lines) {
                visible_trees += 1;
            }
        });
    });

    visible_trees.to_string()
}

pub fn get_part2_answer(input: &str) -> String {
    let mut highest_scenic_score: u32 = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    lines.iter().enumerate().for_each(|(line_idx, line)| {
        line.chars().enumerate().for_each(|(char_idx, char)| {
            highest_scenic_score = cmp::max(
                highest_scenic_score,
                get_tree_scenic_score(char.to_digit(10).unwrap(), (char_idx, line_idx), &lines),
            );
        });
    });
    highest_scenic_score.to_string()
}
