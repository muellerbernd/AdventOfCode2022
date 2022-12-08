use std::collections::HashMap;
use std::fs::read_to_string;

fn transpose<T>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    for inner in &mut v {
        inner.reverse();
    }
    (0..len)
        .map(|_| {
            v.iter_mut()
                .map(|inner| inner.pop().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn get_visible_trees(
    forest: &Vec<Vec<u32>>,
    visible_trees: &mut Vec<Vec<bool>>,
    check_dim1: Vec<usize>,
    check_dim2: Vec<usize>,
) {
    for y in check_dim1 {
        let mut curr_max: i32 = -1;
        for x in &check_dim2 {
            let tree: i32 = forest[y][*x] as i32;
            if tree == 9 {
                visible_trees[y][*x] = true;
                break;
            }
            if tree > curr_max {
                curr_max = tree;
                visible_trees[y][*x] = true;
            }
        }
    }
}
fn main() {
    let file_path = "../inputs/aoc_08.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let mut forest: Vec<Vec<u32>> = raw_input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut visible_trees: Vec<Vec<bool>> = raw_input
        .lines()
        .map(|l| l.chars().map(|_| false).collect())
        .collect();

    get_visible_trees(
        &forest,
        &mut visible_trees,
        (0..forest.len()).collect(),
        (0..forest[0].len()).collect(),
    );
    get_visible_trees(
        &forest,
        &mut visible_trees,
        (0..forest.len()).collect(),
        (0..forest[0].len()).rev().collect(),
    );
    visible_trees = transpose(visible_trees);
    forest = transpose(forest);
    get_visible_trees(
        &forest,
        &mut visible_trees,
        (0..forest.len()).collect(),
        (0..forest[0].len()).collect(),
    );
    get_visible_trees(
        &forest,
        &mut visible_trees,
        (0..forest.len()).collect(),
        (0..forest[0].len()).rev().collect(),
    );
    let task1_res: usize = visible_trees
        .iter()
        .map(|r| r.iter().filter(|c| **c).count())
        .sum();
    println!("task1_res {:?}", task1_res);
}
