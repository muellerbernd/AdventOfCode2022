use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn add_dim_down(v: &mut Vec<Vec<char>>) {
    let new_dim: Vec<char> = v[0].iter().map(|_| '.').collect();
    v.push(new_dim);
}

fn add_dim_up(v: &mut Vec<Vec<char>>) {
    let new_dim: Vec<char> = v[0].iter().map(|_| '.').collect();
    v.insert(0, new_dim);
}

fn add_dim_left(v: &mut Vec<Vec<char>>) {
    for r in v {
        r.insert(0, '.');
    }
}
fn add_dim_right(v: &mut Vec<Vec<char>>) {
    for r in v {
        r.push('.');
    }
}

fn print_2d(v: &Vec<Vec<char>>) {
    for l in v {
        println!("{:?}", l);
    }
}

fn get_2d_index(
    pos: &Position,
    v: &Vec<Vec<char>>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> (usize, usize) {
    ((i32::abs(min_x) + pos.x) as usize, (max_y - pos.y) as usize)
}

fn calc_tail_move(head: &Position, tail: &Position) -> Position {
    let mut new_tail_pos = Position {
        x: tail.x,
        y: tail.y,
    };
    let diff_x = head.x - tail.x;
    let diff_y = head.y - tail.y;
    let (delta_x, delta_y) = if (i32::abs(diff_y) == 2) && (i32::abs(diff_x) == 2) {
        (diff_x / 2, diff_y / 2)
    } else if i32::abs(diff_x) > 1 {
        (diff_x / 2, diff_y)
    } else if i32::abs(diff_y) > 1 {
        (diff_x, diff_y / 2)
    } else {
        (0, 0)
    };
    new_tail_pos.x += delta_x;
    new_tail_pos.y += delta_y;
    new_tail_pos
}

fn main() {
    let file_path = "../inputs/aoc_09.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let mut grid: Vec<Vec<char>> = vec![vec!['s']];
    let mut curr_head_pos = Position { x: 0, y: 0 };
    let mut curr_tail_pos = Position { x: 0, y: 0 };
    let mut min_x = 0;
    let mut max_x = 0;
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
                    curr_head_pos.x += 1;
                    if curr_head_pos.x > max_x {
                        max_x = curr_head_pos.x;
                        add_dim_right(&mut grid);
                    }
                }
                "L" => {
                    curr_head_pos.x -= 1;
                    if curr_head_pos.x < min_x {
                        min_x = curr_head_pos.x;
                        add_dim_left(&mut grid);
                    }
                }
                "U" => {
                    curr_head_pos.y += 1;
                    if curr_head_pos.y > max_y {
                        max_y = curr_head_pos.y;
                        add_dim_up(&mut grid);
                    }
                }
                "D" => {
                    curr_head_pos.y -= 1;
                    if curr_head_pos.y < min_y {
                        min_y = curr_head_pos.y;
                        add_dim_down(&mut grid);
                    }
                }
                _ => println!("unknown direction"),
            }
            // println!("{:?}", curr_head_pos);
            curr_tail_pos = calc_tail_move(&curr_head_pos, &curr_tail_pos);
            let (h_x, h_y) = get_2d_index(&curr_tail_pos, &grid, min_x, max_x, min_y, max_y);
            grid[h_y][h_x] = 'T';
            // let (h_x, h_y) = get_2d_index(&curr_head_pos, &grid, min_x, max_x, min_y, max_y);
            // grid[h_y][h_x] = 'H';
            // print_2d(&grid);
        }
    }
    // print_2d(&grid);
    let tail_positions: usize = grid
        .iter()
        .map(|l| l.iter().filter(|c| c == &&'T').count())
        .sum();
    println!("task1: {}", tail_positions);
}
