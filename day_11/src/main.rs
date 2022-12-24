use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct MonkeyNotes {
    items: Vec<i16>,
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
            items: starting_items,
            operation,
            test_divisor,
            test_true,
            test_false,
        };
        monkey_map.insert(monkey_name, monkey_note);
    }
    return monkey_map;
}

fn task01(mut monkey_map: HashMap<String, MonkeyNotes>) {
    let mut monkey_activity: HashMap<String, i16> = HashMap::new();
    for m in monkey_map.keys() {
        monkey_activity.insert(m.to_string(), 0);
    }
    println!("{:?}", monkey_activity);
    for r in 0..20 {
        println!("round {}", r + 1);
        for (k, v) in &monkey_map {
            println!("{} {:?}", k, v);
            *monkey_activity.get_mut(k).unwrap() += (v.items.len() > 0) as i16;
            let mut op: String =
                v.operation.split_whitespace().collect::<Vec<&str>>()[1].to_string();
            let mut worry_level_delta: i16 = 0;
            if let Ok(result) =
                v.operation.split_whitespace().collect::<Vec<&str>>()[2].parse::<i16>()
            {
                worry_level_delta = result;
            } else {
                op = "sqr".to_string();
            };
            println!("operation {}", op);
            for (index, worry_level) in v.items.iter().enumerate() {
                let new_worry_level = match op.as_str() {
                    "sqr" => worry_level * worry_level,
                    "*" => worry_level * worry_level_delta,
                    "+" => worry_level + worry_level_delta,
                    "-" => worry_level - worry_level_delta,
                    "/" => worry_level / worry_level_delta,
                    _ => -1,
                };
                assert!(new_worry_level != -1);
                println!("worry_level {}", new_worry_level);
            }
        }
        break;
    }
}

fn main() {
    let file_path = "../inputs/aoc_11.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let monkey_map: HashMap<String, MonkeyNotes> = parse_input(raw_input);
    // After each monkey inspects an item but before it tests your worry level, your relief that the monkey's inspection didn't damage the item causes your worry level to be divided by three and rounded down to the nearest integer.
    task01(monkey_map.clone());
}
