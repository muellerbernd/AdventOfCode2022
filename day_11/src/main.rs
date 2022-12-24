use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct MonkeyNotes {
    starting_items: Vec<i16>,
    operation: String,
    test_divisor: i16,
    test_true: String,
    test_false: String,
}

fn parse_input(raw_input: String) -> HashMap<String, MonkeyNotes> {
    let monkey_batch = raw_input.split("\n\n");
    let mut monkey_map: HashMap<String, MonkeyNotes> = HashMap::new();
    for m in monkey_batch {
        let monkey: Vec<&str> = m.lines().collect();
        let monkey_name: String = monkey[0].to_string().to_lowercase();
        let monkey_name: String = monkey_name[0..monkey_name.len() - 1].to_string();
        let starting_items: Vec<&str> = monkey[1].split(":").collect();
        let starting_items: Vec<i16> = starting_items[1]
            .split(",")
            .map(|n| n.replace(" ", "").parse::<i16>().unwrap())
            .collect();
        let operation = monkey[2].split("=").last().unwrap()[1..].to_string();
        let test_divisor: i16 = monkey[3]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i16>()
            .unwrap();
        let test_true: Vec<&str> = monkey[4].split_whitespace().collect();
        let test_true: String = test_true[test_true.len() - 2..test_true.len()]
            .join(" ")
            .to_string();
        let test_false: Vec<&str> = monkey[5].split_whitespace().collect();
        let test_false: String = test_false[test_false.len() - 2..test_false.len()]
            .join(" ")
            .to_string();
        let monkey_note = MonkeyNotes {
            starting_items,
            operation,
            test_divisor,
            test_true,
            test_false,
        };
        monkey_map.insert(monkey_name, monkey_note);
    }
    return monkey_map;
}

fn main() {
    let file_path = "../inputs/aoc_11.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let monkey_map: HashMap<String, MonkeyNotes> = parse_input(raw_input);
    for (k, v) in monkey_map {
        println!("{} {:?}", k, v);
    }
}
