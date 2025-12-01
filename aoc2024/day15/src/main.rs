use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Obstacle {
    None,
    Wall,
    Box,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DWObstacle {
    None,
    Wall,
    LeftBox,
    RightBox,
}

impl TryFrom<char> for Obstacle {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Obstacle::Wall),
            '.' => Ok(Obstacle::None),
            '@' => Ok(Obstacle::None),
            'O' => Ok(Obstacle::Box),
            other => Err(format!("Could not parse {other} as obsacle.")),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn next_pos(&self, pos: (u64, u64)) -> Option<(u64, u64)> {
        match self {
            Dir::Up => {
                if pos.0 > 0 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
            Dir::Down => Some((pos.0 + 1, pos.1)),
            Dir::Left => {
                if pos.1 > 0 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
            Dir::Right => Some((pos.0, pos.1 + 1)),
        }
    }

    fn prev_pos(&self, pos: (u64, u64)) -> Option<(u64, u64)> {
        match self {
            Dir::Down => {
                if pos.0 > 0 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
            Dir::Up => Some((pos.0 + 1, pos.1)),
            Dir::Right => {
                if pos.1 > 0 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
            Dir::Left => Some((pos.0, pos.1 + 1)),
        }
    }
}

impl TryFrom<char> for Dir {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Dir::Up),
            '>' => Ok(Dir::Right),
            '<' => Ok(Dir::Left),
            'v' => Ok(Dir::Down),
            other => Err(format!("Could not parse {other} as direction.")),
        }
    }
}

#[derive(Debug, Default)]
struct Grid {
    pos: (u64, u64),
    entries: HashMap<(u64, u64), Obstacle>,
    max_row: u64,
    max_col: u64,
}

impl Grid {
    fn step(&mut self, dir: Dir) {
        // position to attempt to move to
        let next = dir.next_pos(self.pos).unwrap();
        // lower bounds wall check
        if next.0 * next.1 == 0 || next.0 == self.max_row || next.1 == self.max_col {
            return;
        }

        // check if the next position is free or a box
        match self.entries.get(&next) {
            Some(Obstacle::None) => {
                // if position is free, we can end here
                self.pos = next;
                return;
            }
            Some(Obstacle::Wall) => return,
            _ => {}
        }
        // check if we can push the box
        let Some(mut next_check) = dir.next_pos(next) else {
            return;
        };
        let maybe_free = loop {
            if let Some(obs) = self.entries.get_mut(&next_check) {
                if *obs == Obstacle::None {
                    *obs = Obstacle::Box;
                    break true;
                } else if *obs == Obstacle::Wall {
                    return;
                }
            } else {
                return;
            }
            if let Some(n) = dir.next_pos(next_check) {
                next_check = n;
            } else {
                return;
            }
        };
        if maybe_free {
            self.entries.insert(next, Obstacle::None);
            self.pos = next;
        }
    }

    fn gps(&self) -> u64 {
        self.entries
            .iter()
            .filter_map(|(pos, obs)| {
                if *obs == Obstacle::Box {
                    Some(pos.0 * 100 + pos.1)
                } else {
                    None
                }
            })
            .sum()
    }

    fn expand(self) -> DoubleGrid {
        DoubleGrid {
            pos: (self.pos.0, 2 * self.pos.1),
            entries: self
                .entries
                .into_iter()
                .flat_map(|(k, v)| match v {
                    Obstacle::None => [
                        ((k.0, 2 * k.1), DWObstacle::None),
                        ((k.0, 2 * k.1 + 1), DWObstacle::None),
                    ],
                    Obstacle::Box => [
                        ((k.0, 2 * k.1), DWObstacle::LeftBox),
                        ((k.0, 2 * k.1 + 1), DWObstacle::RightBox),
                    ],
                    Obstacle::Wall => [
                        ((k.0, 2 * k.1), DWObstacle::Wall),
                        ((k.0, 2 * k.1 + 1), DWObstacle::Wall),
                    ],
                })
                .collect(),
            max_row: self.max_row,
            max_col: 2 * self.max_col + 1,
        }
    }
}

struct DoubleGrid {
    pos: (u64, u64),
    entries: HashMap<(u64, u64), DWObstacle>,
    max_row: u64,
    max_col: u64,
}

impl DoubleGrid {
    fn step(&mut self, dir: Dir) {
        // position to attempt to move to
        let next = dir.next_pos(self.pos).unwrap();
        // lower bounds wall check
        if next.0 * next.1 == 0 || next.0 == self.max_row || next.1 == self.max_col {
            return;
        }

        // check if the next position is free or a box
        match self.entries.get(&next) {
            Some(DWObstacle::None) => {
                // if position is free, we can end here
                self.pos = next;
                return;
            }
            Some(DWObstacle::Wall) => return,
            _ => {}
        }

        let mut stack = vec![self.pos];
        let mut to_push = HashSet::new();
        while let Some(next) = stack.pop() {
            to_push.insert(next);
            let Some(next_check) = dir.next_pos(next) else {
                return;
            };
            if to_push.contains(&next_check) {
                continue;
            }
            match self.entries.get(&next_check) {
                None => return,
                Some(DWObstacle::Wall) => return,
                Some(DWObstacle::None) => continue,
                Some(DWObstacle::LeftBox) => {
                    stack.push(next_check);
                    stack.push(Dir::Right.next_pos(next_check).unwrap());
                }
                Some(DWObstacle::RightBox) => {
                    stack.push(next_check);
                    stack.push(Dir::Left.next_pos(next_check).unwrap());
                }
            }
        }
        let entries = self.entries.clone();
        for push_pos in &to_push {
            // move pushed object
            let obs = *entries.get(&push_pos).unwrap();
            self.entries.insert(dir.next_pos(*push_pos).unwrap(), obs);
            // fill in space left behind
            let prev_pos = dir.prev_pos(*push_pos).unwrap();
            if !to_push.contains(&prev_pos) {
                self.entries.insert(*push_pos, DWObstacle::None);
            }
        }
        self.pos = dir.next_pos(self.pos).unwrap();
    }

    fn gps(&self) -> u64 {
        self.entries
            .iter()
            .filter_map(|(pos, obs)| {
                if *obs == DWObstacle::LeftBox {
                    Some(pos.0 * 100 + pos.1)
                } else {
                    None
                }
            })
            .sum()
    }
}

fn parse_file(filename: &str) -> (Grid, Vec<Dir>) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut grid = Grid::default();
    let mut instructions = vec![];
    let mut row = 0u64;

    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }

        if line.trim().is_empty() {
            line.clear();
            continue;
        }

        for (col, c) in line.trim().chars().enumerate() {
            if c == '@' {
                grid.pos = (row, col as u64);
            }
            if let Ok(obs) = Obstacle::try_from(c) {
                grid.entries.insert((row, col as u64), obs);
                grid.max_col = col as u64;
            } else {
                instructions.push(Dir::try_from(c).unwrap());
            }
        }
        row += 1;
        line.clear();
    }
    grid.max_row = row - 2;
    (grid, instructions)
}

fn part_1(filename: &str) {
    let (mut grid, instructions) = parse_file(filename);

    for inst in instructions {
        grid.step(inst);
    }
    println!("Part 2: {}", grid.gps());
}

fn part_2(filename: &str) {
    let (grid, instructions) = parse_file(filename);
    let mut grid = grid.expand();

    for inst in instructions {
        grid.step(inst);
    }
    println!("Part 2: {}", grid.gps());
}

fn main() {
    part_1("input.txt");
    part_2("input.txt")
}
