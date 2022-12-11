use std::collections::HashMap;
use std::fs::read_to_string;

fn parse(lines: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut dirs: HashMap<String, Vec<String>> = HashMap::new();
    let mut dir_stack: Vec<String> = Vec::new();
    for l in lines {
        if l.matches("$ cd").count() > 0 {
            let c: String = l
                .split(" ")
                .nth(2)
                .map(|c| c.to_string().to_owned())
                .unwrap();
            match c.as_str() {
                ".." => {
                    dir_stack.pop();
                }
                _ => {
                    dir_stack.push(c);
                }
            }
            println!("{} {:?}", l, dir_stack);
            let curr_dir: String = dir_stack.join("-");
            if !dirs.contains_key(&curr_dir) {
                dirs.insert(curr_dir, Vec::new());
            }
        }
        if l.matches("$").count() == 0 {
            let curr_dir: String = dir_stack.join("-");
            println!("{:?}", curr_dir);
            if dirs.contains_key(&curr_dir) {
                if !dirs.get(&curr_dir).unwrap().contains(&l) {
                    dirs.get_mut(&curr_dir).unwrap().push(l);
                }
            } else {
                // dirs.insert(curr_dir, vec![l]);
                unreachable!();
            }
        }
    }
    dirs
}

fn resolve_all_dirs(
    dir_map: &HashMap<String, Vec<String>>,
    dir_sizes: &mut HashMap<String, usize>,
    curr_dir: &String,
) {
    let mut curr_size: usize = 0;
    for v in dir_map.get(curr_dir).unwrap() {
        if v.matches("dir").count() > 0 {
            let child: String = format!("{}-{}", curr_dir, v.split_whitespace().nth(1).unwrap());
            resolve_all_dirs(dir_map, dir_sizes, &child);
            curr_size += dir_sizes.get(&child).unwrap();
        } else {
            let file_size = v
                .split_whitespace()
                .nth(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            curr_size += file_size;
        }
    }
    dir_sizes.insert(curr_dir.to_string(), curr_size);
}

fn main() {
    // let file_path = "../inputs/aoc_07.txt";
    let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<String> = raw_input
        .lines()
        .map(|l| l.to_string().to_owned())
        .collect();
    let dirs: HashMap<String, Vec<String>> = parse(lines);
    println!("{:?}", dirs);

    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    resolve_all_dirs(&dirs, &mut dir_sizes, &"/".to_string());
    // println!("{:?}", dir_sizes);
    let task1: usize = dir_sizes
        .iter()
        .filter_map(|(_, dir_size)| {
            if dir_size <= &&100000 {
                Some(dir_size)
            } else {
                None
            }
        })
        .sum();

    println!("task1 {:?}", task1);
    println!("{}", dir_sizes.get(&"/".to_string()).unwrap());
    let task2: usize = *dir_sizes
        .iter()
        .filter_map(|(_, dir_size)| {
            if 70000000 - dir_sizes.get(&"/".to_string()).unwrap() + dir_size >= 30000000 {
                Some(dir_size)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    println!("task2 {:?}", task2);
}
