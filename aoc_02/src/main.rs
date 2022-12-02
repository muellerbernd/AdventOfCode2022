use std::collections::HashMap;
use std::fmt::format;
use std::fs::read_to_string;

fn main() {
    let mut remap = HashMap::new();
    remap.insert(String::from("X"), String::from("A"));
    remap.insert(String::from("Y"), String::from("B"));
    remap.insert(String::from("Z"), String::from("C"));
    let mut out_r = HashMap::new();
    out_r.insert(String::from("AA"), 3);
    out_r.insert(String::from("AB"), 6);
    out_r.insert(String::from("AC"), 0);
    out_r.insert(String::from("BA"), 0);
    out_r.insert(String::from("BB"), 3);
    out_r.insert(String::from("BC"), 6);
    out_r.insert(String::from("CA"), 6);
    out_r.insert(String::from("CB"), 0);
    out_r.insert(String::from("CC"), 3);
    // let file_path = "../inputs/aoc_02.txt";
    let file_path = "test_input.txt";
    println!("In file {}", file_path);

    let contents: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    println!("contents {}", contents);
    let strategy_guide = contents.lines();
    let outcomes: Vec<i32> = Vec::new();
    for val in strategy_guide {
        let mut iter = val.split_whitespace();
        let opponent = iter.next().unwrap();
        let response = remap.get(iter.next().unwrap()).unwrap();
        let k: String = format!("{}{}", opponent, response);
        println!("k:{}", k);
        println!(
            "opponent {:?} response {:?} outcome {}",
            opponent,
            response,
            out_r.get(&k).unwrap()
        );
    }
}
