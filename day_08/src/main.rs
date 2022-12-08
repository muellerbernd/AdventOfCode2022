use std::collections::HashMap;
use std::fs::read_to_string;
fn main() {
    // let file_path = "../inputs/aoc_08.txt";
    let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    println!("{raw_input}");
}
