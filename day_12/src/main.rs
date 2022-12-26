use std::collections::HashMap;
use std::fs::read_to_string;

fn get_graph(height_map: &Vec<Vec<i32>>) {
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    for y_index in 0..height_map.len() {
        for x_index in 0..height_map[0].len() - 1 {
            let val_l = height_map[y_index][x_index];
            let val_r = height_map[y_index][x_index + 1];
            if val_l < val_r {
                graph
                    .entry((x_index + 1, y_index))
                    .and_modify(|v| v.push((x_index, y_index)))
                    .or_insert(vec![(x_index, y_index)]);
                if val_r - val_l <= 1 {
                    graph
                        .entry((x_index, y_index))
                        .and_modify(|v| v.push((x_index + 1, y_index)))
                        .or_insert(vec![(x_index + 1, y_index)]);
                }
            } else if val_l == val_r {
                graph
                    .entry((x_index + 1, y_index))
                    .and_modify(|v| v.push((x_index, y_index)))
                    .or_insert(vec![(x_index, y_index)]);
                graph
                    .entry((x_index, y_index))
                    .and_modify(|v| v.push((x_index + 1, y_index)))
                    .or_insert(vec![(x_index + 1, y_index)]);
            } else {
                graph
                    .entry((x_index, y_index))
                    .and_modify(|v| v.push((x_index + 1, y_index)))
                    .or_insert(vec![(x_index + 1, y_index)]);
                if val_l - val_r <= 1 {
                    graph
                        .entry((x_index + 1, y_index))
                        .and_modify(|v| v.push((x_index, y_index)))
                        .or_insert(vec![(x_index, y_index)]);
                }
            }
        }
    }
    for x_index in 0..height_map[0].len() {
        for y_index in 0..height_map.len() - 1 {
            let val_u = height_map[y_index][x_index];
            let val_d = height_map[y_index + 1][x_index];
            if val_u < val_d {
                graph
                    .entry((x_index, y_index))
                    .and_modify(|v| v.push((x_index, y_index + 1)))
                    .or_insert(vec![(x_index, y_index + 1)]);
                if val_u - val_d <= 1 {
                    graph
                        .entry((x_index, y_index + 1))
                        .and_modify(|v| v.push((x_index, y_index)))
                        .or_insert(vec![(x_index, y_index)]);
                }
            } else if val_u == val_d {
                graph
                    .entry((x_index, y_index))
                    .and_modify(|v| v.push((x_index, y_index + 1)))
                    .or_insert(vec![(x_index, y_index + 1)]);
                graph
                    .entry((x_index, y_index + 1))
                    .and_modify(|v| v.push((x_index, y_index)))
                    .or_insert(vec![(x_index, y_index)]);
            } else {
                graph
                    .entry((x_index, y_index))
                    .and_modify(|v| v.push((x_index + 1, y_index)))
                    .or_insert(vec![(x_index + 1, y_index)]);
                if val_u - val_d <= 1 {
                    graph
                        .entry((x_index, y_index))
                        .and_modify(|v| v.push((x_index, y_index + 1)))
                        .or_insert(vec![(x_index, y_index + 1)]);
                }
            }
        }
    }
    println!("{:?}", graph);
    println!("{:?}", graph.get(&(4,2)));
    println!("{:?}", graph.get(&(1,1)));
}

fn main() {
    // let file_path = "../inputs/aoc_12.txt";
    let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", raw_input);
    let mut start_pos = (0, 0);
    let mut goal_pos = (0, 0);
    // get start and end point from input
    for (i, l) in raw_input.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == 'S' {
                start_pos.0 = j;
                start_pos.1 = i;
            }
            if c == 'E' {
                goal_pos.0 = j;
                goal_pos.1 = i;
            }
        }
    }
    let mod_input_lines: Vec<String> = raw_input
        .lines()
        .map(|l| l.replace("S", "a").replace("E", "z"))
        .collect();
    println!("{:?}", mod_input_lines);
    let height_map: Vec<Vec<i32>> = mod_input_lines
        .iter()
        .map(|l| l.chars().map(|c| c as i32 - 'a' as i32).collect())
        .collect();
    println!("{:?} {:?} {:?}", height_map, start_pos, goal_pos);
    get_graph(&height_map);
}
