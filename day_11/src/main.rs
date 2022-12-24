use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct MonkeyNotes {
    name: String,
    items: Vec<u64>,
    operation: String,
    test_divisor: u64,
    test_true: usize,
    test_false: usize,
}

fn parse_input(raw_input: String) -> Vec<MonkeyNotes> {
    let monkey_batch = raw_input.split("\n\n");
    let mut monkeys: Vec<MonkeyNotes> = Vec::new();
    for m in monkey_batch {
        let monkey: Vec<&str> = m.lines().collect();
        let monkey_name: String = monkey[0].to_string().to_lowercase();
        let monkey_name: String = monkey_name[0..monkey_name.len() - 1].to_string();
        let starting_items: Vec<&str> = monkey[1].split(":").collect();
        let starting_items: Vec<u64> = starting_items[1]
            .split(",")
            .map(|n| n.replace(" ", "").parse::<u64>().unwrap())
            .collect();
        let operation = monkey[2].split("=").last().unwrap()[1..].to_string();
        let test_divisor: u64 = monkey[3]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let test_true: Vec<&str> = monkey[4].split_whitespace().collect();
        let test_true: usize = test_true[test_true.len() - 1].parse().unwrap();
        let test_false: Vec<&str> = monkey[5].split_whitespace().collect();
        let test_false: usize = test_false[test_false.len() - 1].parse().unwrap();
        monkeys.push(MonkeyNotes {
            name: monkey_name,
            items: starting_items,
            operation,
            test_divisor,
            test_true,
            test_false,
        });
    }
    // println!("{:?}", monkey_map);
    return monkeys;
}

fn task01(mut monkeys: Vec<MonkeyNotes>, modulo: u64) {
    let mut monkey_activity: Vec<u64> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let e = monkeys[i].clone();
            monkey_activity[i] += e.items.len() as u64;
            let mut op: String =
                e.operation.split_whitespace().collect::<Vec<&str>>()[1].to_string();
            let mut worry_level_delta: u64 = 0;
            if let Ok(result) =
                e.operation.split_whitespace().collect::<Vec<&str>>()[2].parse::<u64>()
            {
                worry_level_delta = result;
            } else {
                op = "sqr".to_string();
            };
            for worry_level in e.items.iter() {
                let mut new_worry_level = match op.as_str() {
                    "sqr" => worry_level * worry_level,
                    "*" => worry_level * worry_level_delta,
                    "+" => worry_level + worry_level_delta,
                    // "-" => worry_level - worry_level_delta,
                    // "/" => worry_level / worry_level_delta,
                    _ => 0,
                };
                assert!(new_worry_level != 0);
                new_worry_level /= 3;
                if new_worry_level % e.test_divisor == 0 {
                    monkeys[e.test_true].items.push(new_worry_level % modulo);
                } else {
                    monkeys[e.test_false].items.push(new_worry_level % modulo);
                }
            }
            monkeys[i].items.clear();
        }
    }
    monkey_activity.sort_by(|a, b| b.cmp(a));
    println!("{:?}", monkey_activity);
    let monkey_business = monkey_activity[0] * monkey_activity[1];
    println!("task01 {:?}", monkey_business);
}

fn task02(mut monkeys: Vec<MonkeyNotes>, modulo: u64) {
    let mut monkey_activity: Vec<u64> = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let e = monkeys[i].clone();
            monkey_activity[i] += e.items.len() as u64;
            let mut op: String =
                e.operation.split_whitespace().collect::<Vec<&str>>()[1].to_string();
            let mut worry_level_delta: u64 = 0;
            if let Ok(result) =
                e.operation.split_whitespace().collect::<Vec<&str>>()[2].parse::<u64>()
            {
                worry_level_delta = result;
            } else {
                op = "sqr".to_string();
            };
            for worry_level in e.items.iter() {
                let new_worry_level = match op.as_str() {
                    "sqr" => worry_level * worry_level,
                    "*" => worry_level * worry_level_delta,
                    "+" => worry_level + worry_level_delta,
                    // "-" => worry_level - worry_level_delta,
                    // "/" => worry_level / worry_level_delta,
                    _ => 0,
                };
                assert!(new_worry_level != 0);
                // new_worry_level /= 3;
                if new_worry_level % e.test_divisor == 0 {
                    monkeys[e.test_true].items.push(new_worry_level % modulo);
                } else {
                    monkeys[e.test_false].items.push(new_worry_level % modulo);
                }
            }
            monkeys[i].items.clear();
        }
    }
    monkey_activity.sort_by(|a, b| b.cmp(a));
    println!("{:?}", monkey_activity);
    let monkey_business = monkey_activity[0] * monkey_activity[1];
    println!("task02 {:?}", monkey_business);
}

fn main() {
    let file_path = "../inputs/aoc_11.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let monkeys: Vec<MonkeyNotes> = parse_input(raw_input);
    println!("{:?}", monkeys);
    let modulo: u64 = monkeys
        .iter()
        .fold(1, |acc, monkey| monkey.test_divisor * acc);
    println!("modulo {:?}", modulo);
    task01(monkeys.clone(), modulo);
    task02(monkeys.clone(), modulo);
}
