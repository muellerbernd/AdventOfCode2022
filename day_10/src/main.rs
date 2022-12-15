use std::fs::read_to_string;
// use std::collections::Hash Map;

fn next_cycle(cycle: &mut i32, register_val: &i32, acc: &mut i32) {
    if (*cycle - 20) % 40 == 0 {
        *acc += *cycle * register_val;
    }
    *cycle += 1;
}

fn task1(lines: &Vec<&str>) {
    let mut register_val: i32 = 1;
    let mut cycle: i32 = 1;
    let mut acc: i32 = 0;
    for line in lines {
        next_cycle(&mut cycle, &register_val, &mut acc);
        if line.chars().nth(0).unwrap() == 'a' {
            next_cycle(&mut cycle, &register_val, &mut acc);
            register_val += line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        }
        if cycle > 220 {
            break;
        }
    }
    println!("task1 {}", acc);
}

fn next_cycle_task2(cycle: &mut i32, register_val: &i32, crt: &mut Vec<char>) {
    if *cycle >= *register_val && *cycle < *register_val + 3 {
        crt.push('#');
    } else {
        crt.push(' ');
    }
    *cycle += 1;
    *cycle %= 40;
}

fn task2(lines: &Vec<&str>) {
    let mut register_val: i32 = 0;
    let mut cycle: i32 = 0;
    let mut crt: Vec<char> = Vec::new();
    for line in lines {
        next_cycle_task2(&mut cycle, &register_val, &mut crt);
        if line.chars().nth(0).unwrap() == 'a' {
            next_cycle_task2(&mut cycle, &register_val, &mut crt);
            register_val += line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            // register_val %= 40;
        }
    }
    println!("{:?} {}", crt.len(), cycle);
    (0..6).for_each(|i| {
        println!("{:?}", crt[i * 40..(i + 1) * 40].iter().collect::<String>());
    });
}

fn main() {
    let file_path = "../inputs/aoc_10.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{}", raw_input);
    task1(&raw_input.lines().collect());
    task2(&raw_input.lines().collect());
}
