mod parse_cube;
mod part2;

pub use part2::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone)]
enum WrapsTo {
    North((u64, u64)),
    South((u64, u64)),
    East((u64, u64)),
    West((u64, u64)),
}

#[derive(Debug, Clone)]
enum Tile {
    Open,
    Corner([WrapsTo; 2], bool),
    Edge(WrapsTo, bool),
    Closed,
}

impl Tile {
    fn is_open(&self) -> bool {
        match self {
            Tile::Open => true,
            Tile::Corner(_, open) => *open,
            Tile::Edge(_, open) => *open,
            Tile::Closed => false,
        }
    }
}

#[derive(Debug, Clone)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Move(u64),
    TurnLeft,
    TurnRight,
}


type Board = HashMap<(u64, u64), Tile>;

#[derive(Debug, Clone)]
struct You {
    position: (u64, u64),
    heading: Heading,
}

impl You {
    fn password(&self) -> u64 {
        let heading_summand = match self.heading {
            Heading::North => 3u64,
            Heading::South => 1,
            Heading::East => 0,
            Heading::West => 2,
        };
        1000 * (self.position.0 + 1)+ 4 * (self.position.1 + 1) + heading_summand
    }

    fn perform(&mut self, inst: Instruction, board: &Board) {
        match inst {
            Instruction::TurnRight => {
                self.heading = match self.heading {
                    Heading::North => Heading::East,
                    Heading::East => Heading::South,
                    Heading::South => Heading::West,
                    Heading::West => Heading::North,
                };
            }
            Instruction::TurnLeft => {
                self.heading = match self.heading {
                    Heading::North => Heading::West,
                    Heading::West => Heading::South,
                    Heading::South => Heading::East,
                    Heading::East => Heading::North,
                }
            }
            Instruction::Move(amt) => self.walk(amt, board)
        }
    }

    fn walk(&mut self, amount: u64, board: &Board) {
        for _ in 0..amount {
            let pos_candidate = match self.heading {
                Heading::North => match board[&self.position] {
                    Tile::Edge(WrapsTo::North(pos), _) |
                    Tile::Corner([WrapsTo::North(pos), _], _) => pos,
                    _ => (self.position.0 - 1, self.position.1),
                }
                Heading::South => match board[&self.position] {
                    Tile::Edge(WrapsTo::South(pos), _) |
                    Tile::Corner([WrapsTo::South(pos), _], _) => pos,
                    _ => (self.position.0 + 1, self.position.1),
                }
                Heading::East => match board[&self.position] {
                    Tile::Edge(WrapsTo::East(pos), _) |
                    Tile::Corner([_, WrapsTo::East(pos)], _) => pos,
                    _ => (self.position.0, self.position.1 + 1),
                }
                Heading::West => match board[&self.position] {
                    Tile::Edge(WrapsTo::West(pos), _) |
                    Tile::Corner([_, WrapsTo::West(pos)], _) => pos,
                    _ => (self.position.0, self.position.1 - 1),
                }
            };
            if board[&pos_candidate].is_open() {
                self.position = pos_candidate;
            } else {
                return ;
            }
        }
    }
}

fn parse_input(file_prefix: &str) -> (Vec<Instruction>, Board) {
    // parse the directions
    let mut file = File::open(format!("{}_dirs.txt", file_prefix)).unwrap();
    let mut directions = String::new();
    _ = file.read_to_string(&mut directions).unwrap();
    let directions = parse_directions(directions);

    // parse the board
    let mut board = HashMap::<(u64, u64), Tile>::new();
    let file = File::open(format!("{}.txt", file_prefix)).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut row = 0;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        board.extend(line.chars().enumerate().filter_map(|(col, c)| match c {
            '.' => Some(((row, col as u64), Tile::Open)),
            '#' => Some(((row, col as u64), Tile::Closed)),
            _ => None,
        }));
        row += 1;
        line.clear();
    }
    (directions, construct_edges(board))
}

fn parse_directions(dirs: String) -> Vec<Instruction> {
    let mut moves: Vec<Instruction> = dirs
        .split(|c| c == 'L' || c == 'R')
        .map(|amt| Instruction::Move(u64::from_str_radix(amt.trim(), 10).unwrap()))
        .collect();
    let mut turns: Vec<Instruction> = dirs
        .chars()
        .filter_map(|c| match c {
            'L' => Some(Instruction::TurnLeft),
            'R' => Some(Instruction::TurnRight),
            _ => None,
        })
        .collect();
    let first_char = dirs.chars().next().unwrap();
    if first_char == 'L' || first_char == 'R' {
        let optional_tail = if turns.len() > moves.len() {
            turns.pop()
        } else {
            None
        };
        let mut instructions: Vec<_> = turns
            .into_iter()
            .zip(moves.into_iter())
            .flat_map(|pair| [pair.0, pair.1].into_iter())
            .collect();
        if let Some(tail) = optional_tail {
            instructions.push(tail);
        }
        instructions
    } else {
        let optional_tail = if turns.len() < moves.len() {
            moves.pop()
        } else {
            None
        };
        let mut instructions: Vec<_> = moves
            .into_iter()
            .zip(turns.into_iter())
            .flat_map(|pair| [pair.0, pair.1].into_iter())
            .collect();
        if let Some(tail) = optional_tail {
            instructions.push(tail);
        }
        instructions
    }
}

fn construct_edges(mut board: Board) -> Board {
    let mut updates = HashMap::<(u64, u64), Tile>::new();
    for ((row, col), tile) in &board {
        let open = if let Tile::Closed = tile {
            false
        } else {
            true
        };
        let north = if *row == 0 || !board.contains_key(&(row - 1, *col)) {
            board.keys()
                .filter(|(_, c) | c == col)
                .max_by_key(|(r, _)| r)
                .map(|pos| WrapsTo::North(*pos))
        } else {
            None
        };
        let west = if *col == 0 || !board.contains_key(&(*row, col - 1)) {
            board.keys()
                .filter(|(r, _)| r == row)
                .max_by_key(|(_, c)| c)
                .map(|pos| WrapsTo::West(*pos))
        } else {
            None
        };
        let south = if !board.contains_key(&(row + 1, *col)) {
            board.keys()
                .filter(|(_, c) | c == col)
                .min_by_key(|(r, _)| r)
                .map(|pos| WrapsTo::South(*pos))
        } else {
            None
        };
        let east = if !board.contains_key(&(*row, col + 1)) {
            board.keys()
                .filter(|(r, _)| r == row)
                .min_by_key(|(_, c)| c)
                .map(|pos| WrapsTo::East(*pos))
        } else {
            None
        };

        match (north, south, east, west) {
            (Some(n), None, Some(e), None) => updates.insert((*row, *col), Tile::Corner([n, e], open)),
            (Some(n), None, None, Some(w)) => updates.insert((*row, *col), Tile::Corner([n, w], open)),
            (None, Some(s), Some(e), None) => updates.insert((*row, *col), Tile::Corner([s, e], open)),
            (None, Some(s), None, Some(w)) => updates.insert((*row, *col), Tile::Corner([s, w], open)),
            (Some(n), None, None, None) => updates.insert((*row, *col), Tile::Edge(n, open)),
            (None, Some(s), None, None) => updates.insert((*row, *col), Tile::Edge(s, open)),
            (None, None, Some(e), None) => updates.insert((*row, *col), Tile::Edge(e, open)),
            (None, None, None, Some(w)) => updates.insert((*row, *col), Tile::Edge(w, open)),
            _ => None,
        };
    }
    for (k, v) in updates {
        board.insert(k, v);
    }
    board
}

fn part_one(file_prefix: &str) {
    let (directions, board) = parse_input(file_prefix);
    let start = *board.keys()
        .filter(|(row, _)| *row == 0)
        .min_by_key(|x| x.1)
        .unwrap();
    let mut you = You {
        position: start,
        heading: Heading::East,
    };
    for instruction in directions {
        you.perform(instruction, &board);
    }
    println!("Part one: {}", you.password());

}

fn main() {
    part_one("input");
    part_2();
}
