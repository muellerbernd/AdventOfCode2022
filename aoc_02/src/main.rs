use std::collections::HashMap;
use std::fmt::format;
use std::fs::read_to_string;

fn main() {
    let mut shape_score = HashMap::new();
    shape_score.insert(String::from("A"), 1);
    shape_score.insert(String::from("B"), 2);
    shape_score.insert(String::from("C"), 3);
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
    let file_path = "../inputs/aoc_02.txt";
    // let file_path = "test_input.txt";
    println!("In file {}", file_path);

    let contents: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let strategy_guide = contents.lines();
    let mut scores: Vec<i32> = Vec::new();
    for val in strategy_guide.clone() {
        let mut iter = val.split_whitespace();
        let opponent = iter.next().unwrap();
        let response = remap.get(iter.next().unwrap()).unwrap();
        let k: String = format!("{}{}", opponent, response);
        let round_score = out_r.get(&k).unwrap() + shape_score.get(response).unwrap();
        scores.push(round_score);
    }
    println!("task1: {:?}", scores.iter().sum::<i32>());
    // task2

    let mut strategy = HashMap::new();
    strategy.insert(String::from("X"), 0);
    strategy.insert(String::from("Y"), 3);
    strategy.insert(String::from("Z"), 6);
    let mut task2_scores: Vec<i32> = Vec::new();
    for val in strategy_guide.clone() {
        let mut iter = val.split_whitespace();
        let opponent = iter.next().unwrap();
        let tip = iter.next().unwrap();
        let matching_strat: Vec<_> = out_r
            .iter()
            .filter_map(|(k, v)| {
                if v == strategy.get(tip).unwrap() {
                    Some(k)
                } else {
                    None
                }
            })
            .collect();
        let strat_to_play: Vec<String> = matching_strat
            .iter()
            .filter(|x| x.chars().nth(0).unwrap().to_string() == opponent).
            map(|x| x.to_string())
            .collect();
        let k: String = strat_to_play.first().unwrap().to_owned();
        let response: String = k.chars().nth(1).unwrap().to_string();
        let round_score = out_r.get(&k).unwrap() + shape_score.get(&response).unwrap();
        task2_scores.push(round_score);
    }
    println!("task2: {:?}", task2_scores.iter().sum::<i32>());
}
