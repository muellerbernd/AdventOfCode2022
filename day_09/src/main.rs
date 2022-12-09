use std::collections::HashMap;
use std::fs::read_to_string;

fn add_dim_down(v: &mut Vec<Vec<String>>) {
    let new_dim: Vec<String> = v[0].iter().map(|c| ".".to_string()).collect();
    v.push(new_dim);
}

fn add_dim_up(v: &mut Vec<Vec<String>>) {
    let new_dim: Vec<String> = v[0].iter().map(|c| ".".to_string()).collect();
    v.insert(0, new_dim);
}

fn add_dim_left(v: &mut Vec<Vec<String>>) {
    for r in v {
        r.insert(0, ".".to_string());
    }
}
fn add_dim_right(v: &mut Vec<Vec<String>>) {
    for r in v {
        r.push(".".to_string());
    }
}

fn print_2d(v: Vec<Vec<String>>) {
    for l in v {
        println!("{:?}", l);
    }
}

fn main() {
    // let file_path = "../inputs/aoc_09.txt";
    let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let mut grid: Vec<Vec<String>> = vec![vec!["s".to_string()]];
    let mut curr_x = 0;
    let mut min_x = 0;
    let mut max_x = 0;
    let mut curr_y = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for l in raw_input.lines() {
        let dir: String = l.split(" ").collect::<Vec<&str>>()[0].to_string();
        let count: i32 = l.split(" ").collect::<Vec<&str>>()[1]
            .to_string()
            .parse::<i32>()
            .unwrap();
        for _ in 0..count {
            match dir.as_str() {
                "R" => {
                    curr_x += 1;
                    if curr_x > max_x {
                        max_x = curr_x;
                        add_dim_right(&mut grid);
                    }
                }
                "L" => {
                    curr_x -= 1;
                    if curr_x < min_x {
                        min_x = curr_x;
                        add_dim_left(&mut grid);
                    }
                }
                "U" => {
                    curr_y += 1;
                    if curr_y > max_y {
                        max_y = curr_y;
                        add_dim_up(&mut grid);
                    }
                }
                "D" => {
                    curr_y -= 1;
                    if curr_y < min_y {
                        min_y = curr_y;
                        add_dim_down(&mut grid);
                    }
                }
                _ => println!("unknown direction"),
            }
        }
        // println!("{}, {}", dir, count);
    }
    print_2d(grid);
}
