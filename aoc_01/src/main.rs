use ndarray::prelude::*;
use ndarray_stats::*;
use std::fs;

fn main() {
    let file_path = "../inputs/aoc_01.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let elf_calories = contents.split("\n\n").filter(|&x| !x.is_empty());

    let vec: Vec<&str> = elf_calories.collect();
    let mut sums: Vec<i32> = Vec::new();
    for elf in vec {
        let numbers: Vec<i32> = elf
            .split("\n")
            .filter(|&x| !x.is_empty())
            .map(|s| s.parse().expect("parse error"))
            .collect();
        let array = Array::from(numbers);
        sums.push(array.sum());
    }
    let array = Array::from(sums.clone());
    let max = array.max().unwrap();
    println!("part 1: {:?}", max);
    // part 2
    sums.sort_by(|a, b| b.cmp(a));
    let slice = Array::from(sums.clone());
    let part2_sum = slice.slice(s![0..3]).sum();
    println!("part 2: {:?}", part2_sum);
}
