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

fn get_scenic_scores(
    forest: &Vec<Vec<u32>>,
    scenic_scores: &mut Vec<Vec<i32>>,
    check_dim1: Vec<usize>,
    check_dim2: Vec<usize>,
) {
    for y in check_dim1 {
        let mut max_view_dists: Vec<i32> = vec![0; 10];
        for x in &check_dim2 {
            let tree: usize = forest[y][*x] as usize;
            scenic_scores[y][*x] *= max_view_dists[tree];
            max_view_dists = max_view_dists.iter().map(|x| x + 1).collect();
            max_view_dists = max_view_dists
                .iter()
                .enumerate()
                .map(|(i, v)| if i <= tree { 1 } else { *v })
                .collect();
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
    println!("{:?}", forest);
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

    let mut forest_task2: Vec<Vec<u32>> = raw_input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut scenic_scores: Vec<Vec<i32>> = raw_input
        .lines()
        .map(|l| l.chars().map(|_| 1).collect())
        .collect();
    get_scenic_scores(
        &forest_task2,
        &mut scenic_scores,
        (0..forest_task2.len()).collect(),
        (0..forest_task2[0].len()).collect(),
    );
    get_scenic_scores(
        &forest_task2,
        &mut scenic_scores,
        (0..forest_task2.len()).collect(),
        (0..forest_task2[0].len()).rev().collect(),
    );
    forest_task2 = transpose(forest_task2);
    scenic_scores = transpose(scenic_scores);
    get_scenic_scores(
        &forest_task2,
        &mut scenic_scores,
        (0..forest_task2.len()).collect(),
        (0..forest_task2[0].len()).collect(),
    );
    get_scenic_scores(
        &forest_task2,
        &mut scenic_scores,
        (0..forest_task2.len()).collect(),
        (0..forest_task2[0].len()).rev().collect(),
    );
    println!("scenic_scores {:?}", scenic_scores);
    let task2_res = scenic_scores
        .iter()
        .map(|r| r.iter().max().unwrap())
        .max().unwrap();
    println!("task2_res {:?}", task2_res);
}
