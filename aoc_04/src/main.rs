use std::fs::read_to_string;

fn main() {
    let file_path = "../inputs/aoc_04.txt";
    // let file_path = "test_input.txt";

    let contents: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    // println!("contents {}", contents);
    let mut count = 0;
    for line in contents.lines() {
        let pairs: Vec<String> = line.split(",").map(|x| x.to_string()).collect();
        let pair1: String = pairs.get(0).unwrap().to_string();
        let pair2: String = pairs.get(1).unwrap().to_string();
        let pair1_r: Vec<i32> = pair1.split("-").map(|x| x.parse().unwrap()).collect();
        let pair2_r: Vec<i32> = pair2.split("-").map(|x| x.parse().unwrap()).collect();
        let p1: String = (pair1_r.get(0).unwrap().to_owned()..=pair1_r.get(1).unwrap().to_owned())
            .into_iter()
            .map(|x| "~".to_owned() + &x.to_string() + &"+".to_owned())
            .collect::<Vec<String>>()
            .join(" ");
        let p2: String = (pair2_r.get(0).unwrap().to_owned()..=pair2_r.get(1).unwrap().to_owned())
            .into_iter()
            .map(|x| "~".to_owned() + &x.to_string() + &"+".to_owned())
            .collect::<Vec<String>>()
            .join(" ");
        if p2.contains(&p1) {
            count = count + 1;
        } else if p1.contains(&p2) {
            count = count + 1;
        }
    }
    println!("task 1: {}", count);
}
