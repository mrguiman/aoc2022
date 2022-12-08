#![allow(unused)]

#[derive(Debug, Clone)]
struct File {
    size: u32,
    name: String,
}
impl File {
    fn new(size: u32, name: &str) -> Self {
        File {
            size: size,
            name: String::from(name),
        }
    }
}

#[derive(Debug, Clone)]

struct Directory {
    name: String,
    size: u32,
    children_directories: Vec<Directory>,
    children_files: Vec<File>,
}
impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: String::from(name),
            children_directories: vec![],
            children_files: vec![],
            size: 0,
        }
    }
}

fn fold_chain_once(crawl_chain: &mut Vec<Directory>) {
    let folder = crawl_chain.pop().unwrap();
    let chain_len = crawl_chain.len();
    let parent_dir = crawl_chain.get_mut(chain_len - 1).unwrap();
    parent_dir.size += folder.size;
    parent_dir.children_directories.push(folder);
}

fn build_tree(input: &str) -> Directory {
    let mut lines_iter = input.split('\n');
    let mut crawl_chain: Vec<Directory> = vec![];

    'outer: loop {
        match lines_iter.next() {
            None => loop {
                fold_chain_once(&mut crawl_chain);
                if crawl_chain.len() == 1 {
                    break 'outer;
                }
            },
            Some(line) => match line.replace("$ ", "").split_once(' ') {
                Some(("cd", "..")) => fold_chain_once(&mut crawl_chain),
                Some(("cd", path)) => {
                    let new_dir = Directory::new(path);
                    crawl_chain.push(new_dir.clone());
                }
                Some(("dir" | "ls", _)) => (),
                Some((maybe_size, file_name)) => match maybe_size.parse::<u32>() {
                    Ok(size) => {
                        let chain_len = crawl_chain.len();
                        let parent_dir = crawl_chain.get_mut(chain_len - 1).unwrap();
                        parent_dir.children_files.push(File::new(size, file_name));
                        parent_dir.size += size;
                    }
                    _ => (),
                },
                None => (),
            },
        };
    }

    crawl_chain.pop().unwrap()
}

fn sum_sizes(dir: Directory) -> u32 {
    let mut total_size = 0;
    if dir.size < 100000 {
        total_size += dir.size;
    }

    for dir in dir.children_directories {
        total_size += sum_sizes(dir);
    }

    total_size
}

fn get_filtered_dirs(dir: Directory, min_size: u32) -> Vec<u32> {
    let mut qualifying_dir_sizes: Vec<u32> = vec![];

    if dir.size > min_size {
        qualifying_dir_sizes.push(dir.size);
    }

    for dir in dir.children_directories {
        qualifying_dir_sizes.extend(get_filtered_dirs(dir, min_size));
    }

    qualifying_dir_sizes
}

pub fn get_part1_answer(input: &str) -> String {
    let root = build_tree(input);
    sum_sizes(root).to_string()
}
pub fn get_part2_answer(input: &str) -> String {
    let root = build_tree(input);
    let space_to_free_up = 30000000 - (70000000 - root.size);
    let mut qualifying_sizes = get_filtered_dirs(root, space_to_free_up);

    qualifying_sizes.sort();
    qualifying_sizes.reverse();
    qualifying_sizes.pop().unwrap().to_string()
}
