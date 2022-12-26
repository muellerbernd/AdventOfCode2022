use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Write;
// Create a GraphMap with directed edges, and add one edge to it
use petgraph::algo::dijkstra;
use petgraph::dot::{Config, Dot};
use petgraph::graphmap::UnGraphMap;

fn print_2d(v: &Vec<Vec<i32>>) {
    for l in v {
        println!("{:?}", l);
    }
}

fn setup_graph(height_map: &Vec<Vec<i32>>) -> UnGraphMap<(usize, usize), i32> {
    // let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut g = UnGraphMap::new();
    let y_len = height_map.len();
    let x_len = height_map[0].len();
    // let mut c: i32;
    // let mut p: i32;
    let mut c: (usize, usize);
    let mut p: (usize, usize);
    for y_index in 0..y_len {
        for x_index in 0..x_len {
            let curr_val = height_map[y_index][x_index];
            let val_l = if x_index >= 1 {
                Some(height_map[y_index][x_index - 1])
            } else {
                None
            };
            let val_r = if x_index < x_len - 1 {
                Some(height_map[y_index][x_index + 1])
            } else {
                None
            };
            let val_u = if y_index >= 1 {
                Some(height_map[y_index - 1][x_index])
            } else {
                None
            };
            let val_d = if y_index < y_len - 1 {
                Some(height_map[y_index + 1][x_index])
            } else {
                None
            };
            // p = format!("{}{}", x_index.clone(), y_index.clone())
            //     .parse()
            //     .unwrap();
            p = (x_index, y_index);
            if val_r.is_some() && (val_r.unwrap() - curr_val).abs() <= 1 {
                // graph
                //     .entry((x_index, y_index))
                //     .and_modify(|v| v.push((x_index + 1, y_index)))
                //     .or_insert(vec![(x_index + 1, y_index)]);
                c = (x_index + 1, y_index);
                g.add_edge(p, c, -1);
            }
            if val_l.is_some() && (val_l.unwrap() - curr_val).abs() <= 1 {
                // graph
                //     .entry((x_index, y_index))
                //     .and_modify(|v| v.push((x_index - 1, y_index)))
                //     .or_insert(vec![(x_index - 1, y_index)]);
                // c = format!("{}{}", x_index - 1, y_index).parse().unwrap();
                c = (x_index - 1, y_index);
                g.add_edge(p, c, -1);
            }
            if val_u.is_some() && (val_u.unwrap() - curr_val).abs() <= 1 {
                // graph
                //     .entry((x_index, y_index))
                //     .and_modify(|v| v.push((x_index, y_index - 1)))
                //     .or_insert(vec![(x_index, y_index - 1)]);
                // c = format!("{}{}", x_index, y_index - 1).parse().unwrap();
                c = (x_index, y_index - 1);
                g.add_edge(p, c, -1);
            }
            if val_d.is_some() && (val_d.unwrap() - curr_val).abs() <= 1 {
                // graph
                //     .entry((x_index, y_index))
                //     .and_modify(|v| v.push((x_index, y_index + 1)))
                //     .or_insert(vec![(x_index, y_index + 1)]);
                // c = format!("{}{}", x_index, y_index + 1).parse().unwrap();
                c = (x_index, y_index + 1);
                g.add_edge(p, c, -1);
            }
        }
    }
    // println!("{:?}", graph);
    // println!("{:?}", graph.get(&(4, 2)));
    // println!("{:?}", graph.get(&(1, 1)));
    g
}

fn main() {
    let file_path = "../inputs/aoc_12.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{}", raw_input);
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
    println!("start: {:?}, goal: {:?}", start_pos, goal_pos);
    let mod_input_lines: Vec<String> = raw_input
        .lines()
        .map(|l| l.replace("S", "a").replace("E", "z"))
        .collect();
    // println!("{:?}", mod_input_lines);
    let height_map: Vec<Vec<i32>> = mod_input_lines
        .iter()
        .map(|l| l.chars().map(|c| c as i32 - 'a' as i32).collect())
        .collect();
    // print_2d(&height_map);
    // get_graph(&height_map);
    // let g: UnGraphMap<i32, i32> = setup_graph(&height_map);
    let g: UnGraphMap<(usize, usize), i32> = setup_graph(&height_map);
    println!("{:?}", g.neighbors(start_pos));
    println!("{:?}", g);
    let tree = Dot::with_config(&g, &[Config::EdgeNoLabel]);
    println!("{:?}", tree);
    let temp_file = "graph.gv";
    // Open a file in write-only (ignoring errors).
    // This creates the file if it does not exist (and empty the file if it exists).
    let mut file = File::create(temp_file).unwrap();
    // Write a &str in the file (ignoring the result).
    writeln!(&mut file, "{:?}", tree).unwrap();

    // Find the shortest path from stast to goal using `1` as the cost for every edge.
    let node_map: HashMap<(usize, usize), i32> =
        dijkstra(&g, goal_pos.into(), Some(start_pos.into()), |_| 1);
        // dijkstra(&g, start_pos.into(), Some(goal_pos.into()), |_| 1);
    // println!("{:?}", node_map.keys().filter(|x| x.0 == 1 as usize).collect::<Vec<&(usize,usize)>>());
    println!("{:?}", node_map);
    println!("{:?}", node_map.get(&start_pos).unwrap());
}
