use std::collections::HashSet;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    index: usize,
}

impl Node {
    fn new(index: usize, row_length: usize) -> Self {
        let x = index % row_length;
        let y = index / row_length;
        Node {
            x: x,
            y: y,
            index: y * (row_length) + x,
        }
    }
}

fn get_target_nodes(input: &str) -> (Node, Node) {
    let row_length = input.find('\n').unwrap() + 1;
    input
        .chars()
        .enumerate()
        .filter(|(_, char)| *char == 'S' || *char == 'E')
        .fold(
            (Node::new(0, row_length), Node::new(0, row_length)),
            |acc, (i, char)| match char {
                'S' => (Node::new(i, row_length), acc.1.clone()),
                'E' => (acc.0.clone(), Node::new(i, row_length)),
                _ => acc,
            },
        )
}

fn translate_char(char: Option<char>) -> Option<char> {
    match char {
        Some('S') => Some('a'),
        Some('E') => Some('z'),
        x => x,
    }
}

fn get_accessible_neighboors(input: &str, node: &Node) -> Vec<Node> {
    let mut neighboors: Vec<Node> = vec![];
    let last_visible_char_index = input.find('\n').unwrap() - 1;
    let row_length = last_visible_char_index + 2;

    if node.x > 0 {
        neighboors.push(Node::new(node.index - 1, row_length));
    }
    if node.x < last_visible_char_index {
        neighboors.push(Node::new(node.index + 1, row_length));
    }
    if node.y > 0 {
        neighboors.push(Node::new(node.index - row_length, row_length))
    }
    if node.y < input.len() / row_length {
        neighboors.push(Node::new(node.index + row_length, row_length));
    }

    neighboors
        .into_iter()
        .filter(|target_node| {
            let diff = (translate_char(input.chars().nth(target_node.index)).unwrap() as u8) as i32
                - (translate_char(input.chars().nth(node.index)).unwrap() as u8) as i32;
            diff <= 1
        })
        .collect::<Vec<Node>>()
}

pub fn get_part1_answer(input: &str) -> String {
    let (start, end) = get_target_nodes(input); // could probably do without crawling the entire input to find start and end beforehand
    let mut nodes_to_explore: Vec<(Node, u32)> = Vec::from([(start.clone(), 0)]);
    let mut explored_nodes: HashSet<Node> = HashSet::new();

    while let Some((node, distance)) = nodes_to_explore.pop() {
        if node == end {
            return distance.to_string();
        }
        if explored_nodes.contains(&node) {
            continue;
        }
        explored_nodes.insert(node.clone());

        let accessible_neighboors = get_accessible_neighboors(input, &node);
        nodes_to_explore.append(
            &mut accessible_neighboors
                .into_iter()
                .filter(|node| !explored_nodes.contains(node))
                .map(|node| (node, distance + 1))
                .collect::<Vec<(Node, u32)>>(),
        );

        nodes_to_explore.sort_by(|(_, a), (_, b)| b.cmp(a));
    }

    String::from("Couldn't find the end, sorry")
}
pub fn get_part2_answer(_input: &str) -> String {
    String::new()
}