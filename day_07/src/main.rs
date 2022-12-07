use std::collections::HashMap;
use std::fs::read_to_string;
#[derive(Debug, Clone)]
struct Node {
    value: String,
    children: HashMap<String, Node>,
}

impl Node {
    fn new(value: String) -> Self {
        Node {
            value: value,
            children: HashMap::new(),
        }
    }
}

fn resolve_all_dirs(
    dir_map: &HashMap<String, Vec<String>>,
    mut temp_dirs: Vec<String>,
) -> Vec<String> {
    for file in temp_dirs.clone() {
        if file.matches("dir").count() > 0 {
            temp_dirs.remove(temp_dirs.iter().position(|t| t == &file).unwrap());
            let mut d: Vec<String> = dir_map
                .get(file.split(" ").nth(1).unwrap())
                .unwrap()
                .to_vec();
            // println!("{:?}", d);
            temp_dirs.append(&mut d);
        } else {
            if temp_dirs.iter().find(|f| f == &&file).is_none() {
                temp_dirs.push(file);
            }
        }
    }
    let subdir_cntr = temp_dirs
        .iter()
        .filter(|file| file.matches("dir").count() > 0)
        .count();
    if subdir_cntr > 0 {
        println!("resolve_all_dirs {:?}", subdir_cntr);
        return resolve_all_dirs(dir_map, temp_dirs);
    }
    temp_dirs
}

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
            // println!("{} {:?}", l, dir_stack);
        }
        if l.matches("$").count() == 0 {
            let curr_dir: String = dir_stack.last().unwrap().to_string();
            if dirs.contains_key(&curr_dir) {
                dirs.get_mut(&curr_dir).unwrap().push(l);
            } else {
                dirs.insert(curr_dir, vec![l]);
            }
        }
    }
    dirs
}

fn main() {
    let file_path = "../inputs/aoc_07.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<String> = raw_input
        .lines()
        .map(|l| l.to_string().to_owned())
        .collect();
    let dirs: HashMap<String, Vec<String>> = parse(lines);
    println!("{:?}", dirs);

    // let mut root = Node::new("/".to_string());
    // for (k, v) in dirs.clone() {
    //     let mut parent = Node::new(k);
    //     for c in v {
    //         let mut child = Node::new(c);
    //     }
    // }
    // let mut dirs_rec: HashMap<String, Vec<String>> = HashMap::new();
    // for (k, v) in dirs.clone() {
    //     println!("{}", k);
    //     let res_dirs = resolve_all_dirs(&dirs, v.clone());
    //     dirs_rec.insert(k, res_dirs);
    // }
    // println!("{:?}", dirs_rec);
    // let mut dir_sizes: Vec<i32> = Vec::new();
    // for (k, v) in dirs_rec {
    //     let dir_size: i32 = v
    //         .iter()
    //         .map(|line| line.split(" ").nth(0).unwrap().parse::<i32>().unwrap())
    //         .sum();
    //     println!("{} {:?}", k, dir_size);
    //     dir_sizes.push(dir_size);
    // }
    // let task1: i32 = dir_sizes
    //     .iter()
    //     .filter(|dir_size| dir_size <= &&100000)
    //     .sum();
    //
    // println!("task1 {:?}", task1);
}
