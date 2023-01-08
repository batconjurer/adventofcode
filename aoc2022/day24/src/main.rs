use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Blizzard {
    pos: (u64, u64),
    dir: Direction,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
struct Valley {
    max_row: u64,
    max_col: u64,
    blizzards: Vec<Blizzard>,
    you: (u64, u64),
    mins: u64,
}

impl Valley {
    fn step(&mut self) {
        for blizzard in self.blizzards.iter_mut() {
            let pos = match blizzard.dir {
                Direction::North => if blizzard.pos.0 == 1 {
                    (self.max_row - 1, blizzard.pos.1)
                } else {
                    (blizzard.pos.0 - 1, blizzard.pos.1)
                }
                Direction::South => if blizzard.pos.0 == self.max_row - 1 {
                    (1, blizzard.pos.1)
                } else {
                    (blizzard.pos.0 + 1, blizzard.pos.1)
                }
                Direction::East => if blizzard.pos.1 == self.max_col - 1 {
                    (blizzard.pos.0, 1)
                } else {
                    (blizzard.pos.0, blizzard.pos.1 + 1)
                }
                Direction::West => if blizzard.pos.1 == 1 {
                    (blizzard.pos.0, self.max_col - 1)
                } else {
                    (blizzard.pos.0, blizzard.pos.1 - 1)
                }
            };
            blizzard.pos = pos;
        }
    }

    fn safe(&self, you: (u64, u64)) -> bool {
        !self.blizzards.iter().any(|b| b.pos == you)
    }

    fn neighbors(&self) -> Vec<(u64, u64)> {
        let (row, col) = self.you;
        let mut ngbhrs = vec![];
        if row != 0 && row != self.max_row {
            if row == 1 && col == 1 {
                ngbhrs.push((0, 1));
            }
            if row > 1 {
                ngbhrs.push((row - 1, col));
            }
            if col > 1 {
                ngbhrs.push((row, col - 1))
            }
            if row < self.max_row - 1 {
                ngbhrs.push((row + 1, col));
            }
            if col < self.max_col - 1 {
                ngbhrs.push((row, col + 1));
            }
            if row == self.max_row - 1 && col == self.max_col - 1 {
                ngbhrs.push((self.max_row, self.max_col - 1));
            }
        } else if row == 0 {
            ngbhrs.push((1, 1));
        } else if row == self.max_row {
            ngbhrs.push((self.max_row - 1, self.max_col - 1));
        }
        ngbhrs.push(self.you);
        ngbhrs
    }
}

fn parse_input(filename: &str) -> Valley {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut blizzards = vec![];
    let mut row = 0u64;
    let mut cols = 0u64;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        cols = line.len() as u64;
        for (col, c) in line.chars().enumerate() {
            match c {
                '>' => blizzards.push(Blizzard{pos: (row, col as u64), dir: Direction::East}),
                '^' => blizzards.push(Blizzard{pos: (row, col as u64), dir: Direction::North}),
                '<' => blizzards.push(Blizzard{pos: (row, col as u64), dir: Direction::West}),
                'v' => blizzards.push(Blizzard{pos: (row, col as u64), dir: Direction::South}),
                _ => {}
            }
        }
        row += 1;
        line.clear();
    }
    Valley {
        max_row: row - 1,
        max_col: cols - 1,
        blizzards,
        you: (0, 1),
        mins: 0,
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Vertex {
    pos: (u64, u64),
    time: u64
}

impl<'a> From<&'a Valley> for Vertex {
    fn from(valley: &'a Valley) -> Self {
        Self {
            pos: valley.you,
            time: valley.mins,
        }
    }
}

fn bfs(dest: (u64, u64), valley: Valley) -> Valley {
    let mut stack = VecDeque::from([valley]);
    let mut visited = HashSet::<Vertex>::new();
    while let Some(mut valley) = stack.pop_front() {
        valley.step();
        for neighbor in valley.neighbors() {
            if valley.safe(neighbor) {
                let mut next_valley = valley.clone();
                next_valley.you = neighbor;
                next_valley.mins += 1;
                if neighbor == dest {
                    return next_valley;
                }
                if !visited.contains(&(&next_valley).into()) {
                    stack.push_back(next_valley.clone());
                    visited.insert((&next_valley).into());
                }
            }
        }
    }
    Default::default()
}


fn run(filename: &str) {
    let valley = parse_input(filename);
    let valley = bfs((valley.max_row, valley.max_col - 1), valley);
    println!("Part one: {}", valley.mins);
    let valley = bfs((0, 1), valley);
    let valley = bfs((valley.max_row, valley.max_col - 1), valley);
    println!("Part two: {}", valley.mins);
}

fn main() {
    run("input.txt")
}
