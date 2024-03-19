use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Board {
    inner: Vec<Vec<u64>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn at(&self, (row, col): (usize, usize)) -> Option<u64> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(self.inner[row][col])
        }
    }
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Dir {
    Up, Down, Left, Right
}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Partial {
    head: (usize, usize),
    dir: Dir,
    visited: BTreeSet<(usize, usize)>,
    //path: Vec<((usize, usize), u64)>,
    heat_loss: u64,
    length: i8,
}

impl Partial {
    fn neighbors(&self, board: &Board) -> Vec<Self> {
        let north_neighbor = if self.head.0 != 0  {
            let next_head = (self.head.0 - 1, self.head.1);
            let heat_loss = board.at(next_head).unwrap();
            let mut next = self.clone();
            next.head = next_head;
            next.dir = Dir::Up;
            next.heat_loss += heat_loss;
            if self.dir == next.dir {
                next.length += 1;
            } else {
                next.length = 0;
            }
            if next.length > 3 || self.visited.contains(&next.head) {
                None
            } else {
                next.visited.insert(next.head);
                // next.path.push((next.head, next.heat_loss));
                Some(next)
            }
        } else {
            None
        };
        let west_neighbor = if self.head.1 != 0 {
            let next_head = (self.head.0, self.head.1 - 1);
            let heat_loss = board.at(next_head).unwrap();
            let mut next = self.clone();
            next.head = next_head;
            next.dir = Dir::Left;
            next.heat_loss += heat_loss;
            if self.dir == next.dir {
                next.length += 1;
            } else {
                next.length = 0;
            }
            if next.length > 3  || self.visited.contains(&next.head){
                None
            } else {
                next.visited.insert(next.head);
                // next.path.push((next.head, next.heat_loss));
                Some(next)
            }
        }  else {
            None
        };
        let south_neighbor = if let Some(heat_loss) = board.at((self.head.0 + 1, self.head.1)) {
            let mut next = self.clone();
            next.head = (self.head.0 + 1, self.head.1);
            next.dir = Dir::Down;
            next.heat_loss += heat_loss;
            if self.dir == next.dir {
                next.length += 1;
            } else {
                next.length = 0;
            }
            if next.length > 3  || self.visited.contains(&next.head) {
                None
            } else {
                next.visited.insert(next.head);
                // next.path.push((next.head, next.heat_loss));
                Some(next)
            }
        } else {
            None
        };
        let east_neighbor = if let Some(heat_loss) = board.at((self.head.0, self.head.1 + 1)) {
            let mut next = self.clone();
            next.head = (self.head.0, self.head.1 + 1);
            next.dir = Dir::Right;
            next.heat_loss += heat_loss;
            if self.dir == next.dir {
                next.length += 1;
            } else {
                next.length = 0;
            }
            if next.length > 3 || self.visited.contains(&next.head) {
                None
            } else {
                next.visited.insert(next.head);
                // next.path.push((next.head, next.heat_loss));
                Some(next)
            }
        } else {
            None
        };
        match self.dir {
            Dir::Up => {
                let mut neighbors: Vec<_> = vec![north_neighbor, east_neighbor, west_neighbor]
                    .into_iter()
                    .filter_map(|n| n)
                    .collect();
                neighbors.sort_by_key(|n| n.heat_loss);
                neighbors
            }
            Dir::Down => {
                let mut neighbors: Vec<_> = vec![south_neighbor, east_neighbor, west_neighbor]
                    .into_iter()
                    .filter_map(|n| n)
                    .collect();
                neighbors.sort_by_key(|n| n.heat_loss);
                neighbors
            }
            Dir::Left => {
                let mut neighbors: Vec<_> = vec![north_neighbor, south_neighbor, west_neighbor]
                    .into_iter()
                    .filter_map(|n| n)
                    .collect();
                neighbors.sort_by_key(|n| n.heat_loss);
                neighbors
            }
            Dir::Right => {
                let mut neighbors: Vec<_> = vec![north_neighbor, east_neighbor, south_neighbor]
                    .into_iter()
                    .filter_map(|n| n)
                    .collect();
                neighbors.sort_by_key(|n| n.heat_loss);
                neighbors
            }
        }
    }
}

fn branch_and_bound(board: &Board) -> u64 {
    let mut stack = vec![
        Partial {
            head: (0, 0),
            dir: Dir::Down,
            visited: BTreeSet::from([(0, 0)]),
            //path: vec![((0, 0), 0)],
            heat_loss: 0,
            length: -1,
        },
    ];
    let dest = (board.rows - 1, board.cols -1);
    let mut min_heat = 122;
    while let Some(partial) = stack.pop() {
        for n in partial.neighbors(&board) {
            if n.head == dest {
                if n.heat_loss < min_heat {
                    println!("new minimum: {}", n.heat_loss);
                    min_heat = n.heat_loss;
                }
            } else if n.heat_loss < min_heat {
                stack.push(n);
            }
        }
    }
    min_heat
}

fn parse(filename: &str) -> Board {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut board = vec![];
    let mut cols = 0;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let row = line.trim()
            .chars()
            .map(|c| u64::from_str(&c.to_string()).unwrap() )
            .collect::<Vec<_>>();
        cols = row.len();
        board.push(row);
        line.clear()
    }
    Board {
        rows: board.len(),
        cols,
        inner: board,
    }
}

fn part_one(filename: &str) {
    let board = parse(filename);
    let min_heat = branch_and_bound(&board);
    println!("Part one: {}", min_heat);
}

fn main() {
    part_one("test.txt");
}
