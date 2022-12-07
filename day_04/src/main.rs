use std::collections::HashSet;
use std::fs::read_to_string;
fn main() {
    let file_path = "../inputs/aoc_04.txt";
    // let file_path = "test_input.txt";

    let contents: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    // println!("contents {}", contents);
    let mut count = 0;
    let mut task2_count = 0;
    for line in contents.lines() {
        let pairs: Vec<String> = line.split(",").map(|x| x.to_string()).collect();
        let pair1: String = pairs.get(0).unwrap().to_string();
        let pair2: String = pairs.get(1).unwrap().to_string();
        let pair1_r: Vec<i32> = pair1.split("-").map(|x| x.parse().unwrap()).collect();
        let pair2_r: Vec<i32> = pair2.split("-").map(|x| x.parse().unwrap()).collect();
        let p1_set: HashSet<String> = (pair1_r.get(0).unwrap().to_owned()
            ..=pair1_r.get(1).unwrap().to_owned())
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let p2_set: HashSet<String> = (pair2_r.get(0).unwrap().to_owned()
            ..=pair2_r.get(1).unwrap().to_owned())
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let diff1: HashSet<_> = p1_set.difference(&p2_set).collect();
        let diff2: HashSet<_> = p2_set.difference(&p1_set).collect();
        if diff1.is_empty() || diff2.is_empty() {
            count += 1;
        }
        // task2
        let intersect: HashSet<_> = p1_set.intersection(&p2_set).collect();
        if !intersect.is_empty() {
            task2_count += 1;
        }
    }
    println!("task 1: {}", count);
    println!("task 2: {}", task2_count);
}
