use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        let mut result = Vec::new();

        // up
        if self.y > 0 {
            result.push(Self {
                x: self.x,
                y: self.y - 1,
            });
        }
        // down
        if self.y < rows - 1 {
            result.push(Self {
                x: self.x,
                y: self.y + 1,
            });
        }
        // left
        if self.x > 0 {
            result.push(Self {
                x: self.x - 1,
                y: self.y,
            });
        }
        // right
        if self.x < cols - 1 {
            result.push(Self {
                x: self.x + 1,
                y: self.y,
            });
        }

        result
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Node {
    cost: u32,
    coord: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn print_2d<T>(v: &Vec<Vec<T>>)
where
    T: std::fmt::Debug,
{
    for l in v {
        println!("{:?}", l);
    }
}

fn parse(input: String) -> (Coord, Coord, Vec<Vec<u8>>, usize, usize) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut map = vec![vec![0; cols]; rows];
    let mut start = Coord { x: 0, y: 0 };
    let mut goal = Coord { x: 0, y: 0 };

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let letter = match c {
                'S' => {
                    start.x = col;
                    start.y = row;
                    'a'
                }
                'E' => {
                    goal.x = col;
                    goal.y = row;
                    'z'
                }
                'a'..='z' => c,
                _ => panic!("Invalid input"),
            };

            let val = letter as u8 - b'a';
            map[row][col] = val;
        }
    }

    (start, goal, map, rows, cols)
}

fn part_1(start: Coord, goal: Coord, map: &Vec<Vec<u8>>, rows: usize, cols: usize) -> u32 {
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    pq.push(Node {
        cost: 0,
        coord: start.clone(),
    });
    visited.insert(start);

    while let Some(Node { coord, cost }) = pq.pop() {
        if coord == goal {
            return cost;
        }

        let curr_height = map[coord.y][coord.x];
        let neighbours = coord.neighbours(rows, cols);
        let candidates: Vec<_> = neighbours
            .iter()
            .filter(|coord| {
                let height = map[coord.y][coord.x];
                height <= curr_height || height == curr_height + 1
            })
            .collect();

        for candidate in candidates {
            if visited.insert(*candidate) {
                pq.push(Node {
                    cost: cost + 1,
                    coord: *candidate,
                })
            }
        }
    }

    panic!("No path found");
}
fn part_2(start: Coord, goal: Coord, map: &Vec<Vec<u8>>, rows: usize, cols: usize) -> u32 {
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    pq.push(Node {
        cost: 0,
        coord: goal.clone(),
    });
    visited.insert(start);

    while let Some(Node { coord, cost }) = pq.pop() {
        let curr_height = map[coord.y][coord.x];
        if curr_height == 0 {
            return cost;
        }

        let neighbours = coord.neighbours(rows, cols);
        let candidates: Vec<_> = neighbours
            .iter()
            .filter(|coord| {
                let height = map[coord.y][coord.x];
                height >= curr_height || height == curr_height - 1
            })
            .collect();

        for candidate in candidates {
            if visited.insert(*candidate) {
                pq.push(Node {
                    cost: cost + 1,
                    coord: *candidate,
                })
            }
        }
    }

    u32::MAX
}

fn main() {
    let file_path = "../inputs/aoc_12.txt";
    // let file_path = "test_input.txt";

    let raw_input: String =
        read_to_string(file_path).expect("Should have been able to read the file");
    let (start, goal, map, rows, cols) = parse(raw_input);
    println!("start {:?}, goal {:?}", start, goal);
    print_2d(&map);
    let path_cost = part_1(start, goal, &map, rows, cols);
    println!("task01 {:?}", path_cost);
    let path_cost = part_2(start, goal, &map, rows, cols);
    println!("task02 {:?}", path_cost);
}
