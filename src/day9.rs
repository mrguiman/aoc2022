use std::collections::HashSet;

fn apply_move(pos: (i32, i32), move_str: &str) -> (i32, i32) {
    match move_str {
        "U" => (pos.0, pos.1 + 1),
        "R" => (pos.0 + 1, pos.1),
        "D" => (pos.0, pos.1 - 1),
        "L" => (pos.0 - 1, pos.1),
        _ => pos,
    }
}

fn adjust_following_knot(head_knot: &(i32, i32), following_knot: &mut (i32, i32)) {
    let x_delta = head_knot.0 - following_knot.0;
    let y_delta = head_knot.1 - following_knot.1;
    let mut x_move = 0;
    let mut y_move = 0;

    match (x_delta.abs(), y_delta.abs()) {
        (0, 2) => y_move = 1 * y_delta.signum(),
        (2, 0) => x_move = 1 * x_delta.signum(),
        (2, 2) | (2, 1) | (1, 2) => {
            x_move = 1 * x_delta.signum();
            y_move = 1 * y_delta.signum();
        }
        _ => (),
    }

    *following_knot = (following_knot.0 + x_move, following_knot.1 + y_move);
}

pub fn get_part1_answer(input: &str) -> String {
    let mut current_tail_pos: (i32, i32) = (0, 0);
    let mut current_head_pos: (i32, i32) = (0, 0);
    let mut tail_visit_positions: HashSet<(i32, i32)> = HashSet::from([current_tail_pos.clone()]);

    input
        .split('\n')
        .map(|move_str| move_str.split_once(' ').unwrap())
        .map(|(dir, times)| (dir, times.parse::<i32>().unwrap()))
        .for_each(|(dir, times)| {
            for _ in 1..=times {
                current_head_pos = apply_move(current_head_pos, dir);
                adjust_following_knot(&current_head_pos, &mut current_tail_pos);
                tail_visit_positions.insert(current_tail_pos.clone());
            }
        });

    tail_visit_positions.len().to_string()
}
pub fn get_part2_answer(input: &str) -> String {
    let mut knots: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let mut tail_visit_positions: HashSet<(i32, i32)> =
        HashSet::from([knots[knots.len() - 1].clone()]);

    input
        .split('\n')
        .map(|move_str| move_str.split_once(' ').unwrap())
        .map(|(dir, times)| (dir, times.parse::<i32>().unwrap()))
        .for_each(|(dir, times)| {
            for _ in 1..=times {
                knots[0] = apply_move(knots[0], dir);
                for i in 0..knots.len() - 1 {
                    adjust_following_knot(&knots[i].clone(), &mut knots[i + 1]);
                }
                tail_visit_positions.insert(knots[knots.len() - 1].clone());
            }
        });

    tail_visit_positions.len().to_string()
}
