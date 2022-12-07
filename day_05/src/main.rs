use std::fs::read_to_string;
fn transpose<T>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    for inner in &mut v {
        inner.reverse();
    }
    (0..len)
        .map(|_| {
            v.iter_mut()
                .map(|inner| inner.pop().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn do_move(stacks: &mut Vec<Vec<char>>, mv: Vec<usize>) {
    assert!(mv.len() == 3);
    let count: usize = mv.get(0).unwrap().to_owned();
    let from: usize = mv.get(1).unwrap().to_owned() - 1;
    let to: usize = mv.get(2).unwrap().to_owned() - 1;
    for _ in 0..count {
        match stacks[from].pop() {
            Some(v) => stacks[to].push(v),
            None => break,
        }
    }
}

fn do_move_task2(stacks: &mut Vec<Vec<char>>, mv: Vec<usize>) {
    assert!(mv.len() == 3);
    let mut temp_stack: Vec<char> = Vec::new();
    let count: usize = mv.get(0).unwrap().to_owned();
    let from: usize = mv.get(1).unwrap().to_owned() - 1;
    let to: usize = mv.get(2).unwrap().to_owned() - 1;
    for _ in 0..count {
        match stacks[from].pop() {
            Some(v) => temp_stack.push(v),
            None => break,
        }
    }
    for _ in 0..count {
        match temp_stack.pop() {
            Some(v) => stacks[to].push(v),
            None => break,
        }
    }
}
fn main() {
    let file_path = "../inputs/aoc_05.txt";
    // let file_path = "test_input.txt";

    let raw: String = read_to_string(file_path).expect("Should have been able to read the file");
    let (stacks_raw, rearrangements_raw) = raw.split_once("\n\n").unwrap();
    let stacks: Vec<Vec<char>> = stacks_raw
        .lines()
        .into_iter()
        .rev()
        .skip(1)
        .map(|x| {
            x.chars()
                .skip(1)
                .step_by(4)
                .collect()
        })
        .collect();
    let mut stacks_t: Vec<Vec<char>> = transpose(stacks)
        .into_iter()
        .map(|stack| stack.into_iter().filter(|c| c != &' ').collect())
        .collect();
    let mut stacks_task2_t: Vec<Vec<char>> = stacks_t.clone();
    // move x items from A to B
    let rearrangements: Vec<Vec<usize>> = rearrangements_raw
        .lines()
        .into_iter()
        .map(|line| {
            line.split(" ")
                .into_iter()
                .filter(|c| c.parse::<usize>().is_ok())
                .map(|c| c.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    for mv in rearrangements.clone() {
        do_move(&mut stacks_t, mv);
    }
    println!("task1 {:?}", stacks_t.iter().map(|stack| stack.last().unwrap_or(&' ').to_string()).collect::<Vec<String>>().join(""));

    for mv in rearrangements {
        do_move_task2(&mut stacks_task2_t, mv);
    }
    println!("task2 {:?}", stacks_task2_t.iter().map(|stack| stack.last().unwrap_or(&' ').to_string()).collect::<Vec<String>>().join(""));
}
