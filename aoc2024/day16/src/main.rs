use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Obstacle {
    Wall,
    Free,
    End,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Dir {
    North, South, East, West
}

impl Default for Dir {
    fn default() -> Self {
        Self::East
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Forward,
    TurnCW,
    TurnCCW,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
struct Reindeer {
    pos: (u64, u64),
    dir: Dir,
}

impl Reindeer {
    fn step(&mut self, step: Move, maze: &Maze) -> bool {
        match step {
            Move::Forward => match self.dir {
                Dir::North => if self.pos.0 > 0 {
                    match maze.grid.get(&(self.pos.0 - 1 , self.pos.1)) {
                        Some(Obstacle::Free) | Some(Obstacle::End) => {
                            self.pos.0 -= 1;
                        }
                        _ => return false
                    }
                } else {
                    return false;
                }
                Dir::South => match maze.grid.get(&(self.pos.0 + 1, self.pos.1)) {
                    Some(Obstacle::Free) | Some(Obstacle::End) => {
                        self.pos.0 += 1;
                    }
                    _ => return false,
                }
                Dir::East => match maze.grid.get(&(self.pos.0, self.pos.1 + 1)) {
                    Some(Obstacle::Free) | Some(Obstacle::End) => {
                        self.pos.1 += 1;
                    }
                    _ => return false
                }
                Dir::West => if self.pos.1 > 0 {
                    match maze.grid.get(&(self.pos.0 , self.pos.1 - 1)) {
                        Some(Obstacle::Free) | Some(Obstacle::End) => {
                            self.pos.1 -= 1;
                        }
                        _ => return false
                    }
                } else {
                    return false;
                }
            }
            Move::TurnCW => match self.dir {
                Dir::North => self.dir = Dir::East,
                Dir::South => self.dir = Dir::West,
                Dir::East => self.dir = Dir::South,
                Dir::West => self.dir = Dir::North,
            }
            Move::TurnCCW => match self.dir {
                Dir::North => self.dir = Dir::West,
                Dir::South => self.dir = Dir::East,
                Dir::East => self.dir = Dir::North,
                Dir::West => self.dir = Dir::South,
            }
        }
        true
    }
}

#[derive(Debug, Default)]
struct Maze {
    start: (u64, u64),
    grid: HashMap<(u64, u64), Obstacle>
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Partial {
    reindeer: Reindeer,
    moves: Vec<Move>,
    score: u64,
}

impl Partial {
    fn is_full(&self, maze: &Maze) -> bool {
        maze.grid.get(&self.reindeer.pos) == Some(&Obstacle::End)
    }

    fn north_neighbor(&self, maze: &Maze) -> Option<Self> {
        if self.reindeer.pos.0 > 0 {
            match maze.grid.get(&(self.reindeer.pos.0 - 1 , self.reindeer.pos.1)) {
                Some(Obstacle::Free) | Some(Obstacle::End) => {
                    let (ms, score) = match self.reindeer.dir {
                        Dir::North => {
                            let mut ms = self.moves.clone();
                            ms.push(Move::Forward);
                            (ms, self.score + 1)
                        }
                        Dir::South => {
                            unreachable!()
                        }
                        Dir::East => {
                            let mut ms = self.moves.clone();
                            ms.push(Move::TurnCCW);
                            ms.push(Move::Forward);
                            (ms, self.score + 1001)
                        }
                        Dir::West => {
                            let mut ms = self.moves.clone();
                            ms.push(Move::TurnCW);
                            ms.push(Move::Forward);
                            (ms, self.score + 1001)
                        }
                    };
                    Some(Partial {
                        reindeer: Reindeer {
                            pos: (self.reindeer.pos.0 - 1, self.reindeer.pos.1),
                            dir: Dir::North,
                        },
                        moves: ms,
                        score,
                    })
                }
                _ => None
            }
        } else {
            None
        }
    }

    fn west_neighbor(&self, maze: &Maze) -> Option<Self> {
        if self.reindeer.pos.1 > 0 {
            match maze.grid.get(&(self.reindeer.pos.0 , self.reindeer.pos.1 - 1)) {
                Some(Obstacle::Free) | Some(Obstacle::End) => {
                    let (ms, score) = match self.reindeer.dir {
                        Dir::North => {
                            let mut ms = self.moves.clone();
                            ms.push(Move::TurnCCW);
                            ms.push(Move::Forward);
                            (ms, self.score + 1001)
                        }
                        Dir::East => {
                            unreachable!()
                        }
                        Dir::South => {
                            let mut ms = self.moves.clone();
                            ms.push(Move::TurnCW);
                            ms.push(Move::Forward);
                            (ms, self.score + 1001)
                        }
                        Dir::West => {
                            let mut ms = self.moves.clone();
                            ms.push(Move::Forward);
                            (ms, self.score + 1)
                        }
                    };
                    Some(Partial {
                        reindeer: Reindeer {
                            pos: (self.reindeer.pos.0, self.reindeer.pos.1 - 1),
                            dir: Dir::West,
                        },
                        moves: ms,
                        score,
                    })
                }
                _ => None
            }
        } else {
            None
        }
    }

    fn east_neighbor(&self, maze: &Maze) -> Option<Self> {
        match maze.grid.get(&(self.reindeer.pos.0 , self.reindeer.pos.1 + 1)) {
            Some(Obstacle::Free) | Some(Obstacle::End) => {
                let (ms, score) = match self.reindeer.dir {
                    Dir::North => {
                        let mut ms = self.moves.clone();
                        ms.push(Move::TurnCW);
                        ms.push(Move::Forward);
                        (ms, self.score + 1001)
                    }
                    Dir::West => {
                        unreachable!()
                    }
                    Dir::South => {
                        let mut ms = self.moves.clone();
                        ms.push(Move::TurnCCW);
                        ms.push(Move::Forward);
                        (ms, self.score + 1001)
                    }
                    Dir::East => {
                        let mut ms = self.moves.clone();
                        ms.push(Move::Forward);
                        (ms, self.score + 1)
                    }
                };
                Some(Partial {
                    reindeer: Reindeer {
                        pos: (self.reindeer.pos.0, self.reindeer.pos.1 + 1),
                        dir: Dir::East,
                    },
                    moves: ms,
                    score,
                })
            }
            _ => None
        }
    }

    fn south_neighbor(&self, maze: &Maze) -> Option<Self> {
        match maze.grid.get(&(self.reindeer.pos.0 + 1, self.reindeer.pos.1)) {
            Some(Obstacle::Free) | Some(Obstacle::End) => {
                let (ms, score) = match self.reindeer.dir {
                    Dir::East => {
                        let mut ms = self.moves.clone();
                        ms.push(Move::TurnCW);
                        ms.push(Move::Forward);
                        (ms, self.score + 1001)
                    }
                    Dir::North => {
                        unreachable!()
                    }
                    Dir::West => {
                        let mut ms = self.moves.clone();
                        ms.push(Move::TurnCCW);
                        ms.push(Move::Forward);
                        (ms, self.score + 1001)
                    }
                    Dir::South => {
                        let mut ms = self.moves.clone();
                        ms.push(Move::Forward);
                        (ms, self.score + 1)
                    }
                };
                Some(Partial {
                    reindeer: Reindeer {
                        pos: (self.reindeer.pos.0 + 1, self.reindeer.pos.1),
                        dir: Dir::South,
                    },
                    moves: ms,
                    score,
                })
            }
            _ => None
        }
    }

    fn neighbors(&self, maze: &Maze) -> impl Iterator<Item=Partial> {
        let mut ns = [const {None}; 3];
        match self.reindeer.dir {
            Dir::North => {
                ns[0] = self.west_neighbor(maze);
                ns[1] = self.east_neighbor(maze);
                ns[2] = self.north_neighbor(maze);
            }
            Dir::South => {
                ns[0] = self.west_neighbor(maze);
                ns[1] = self.east_neighbor(maze);
                ns[2] = self.south_neighbor(maze);
            }
            Dir::East => {
                ns[0] = self.south_neighbor(maze);
                ns[1] = self.north_neighbor(maze);
                ns[2] = self.east_neighbor(maze);
            }
            Dir::West => {
                ns[0] = self.south_neighbor(maze);
                ns[1] = self.north_neighbor(maze);
                ns[2] = self.west_neighbor(maze);
            }
        }
        ns.into_iter().filter_map(|x| x)
    }

    fn path(&self, maze: &Maze) -> HashMap<(u64, u64), Dir> {
        let mut reindeer = Reindeer {
            pos: maze.start,
            dir: Default::default(),
        };
        let mut moves = self.moves.iter().map(|m| {
            assert!(reindeer.step(*m, maze));
            (reindeer.pos, reindeer.dir)
        }).collect::<HashMap<_, _>>();
        if !moves.contains_key(&maze.start) {
            moves.insert(maze.start, Dir::East);
        }
        moves
    }

}

impl PartialOrd for Partial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl Ord for Partial {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

fn search(reindeer: Reindeer, maze: &Maze) -> Vec<Partial> {
    let mut stack = BinaryHeap::new();
    let mut dists = HashMap::new();
    let partial = Partial {
        reindeer,
        moves: vec![],
        score: 0,
    };
    dists.insert(partial.reindeer.pos, 0);
    stack.push(partial);

    let mut solutions = vec![];
    while let Some(next) = stack.pop() {
        if next.is_full(maze) {
            solutions.push(next);
            continue;
        }

        if next.score > dists.get(&next.reindeer.pos).map(|x| x + 1000).unwrap_or(u64::MAX) {
            continue
        }

        for n in next.neighbors(maze) {
            let dist = *dists.get(&n.reindeer.pos).unwrap_or(&u64::MAX);
            if n.score <= dist {
                dists.insert(n.reindeer.pos, n.score);
            }
            if n.score <= dist.saturating_add(1000) {
                stack.push(n);
            }
        }
    }
    solutions
}

fn parse_file(filename: &str) -> (Reindeer, Maze) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut row = 0u64;
    let mut reindeer = Reindeer::default();
    let mut maze = Maze::default();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }

        if line.trim().is_empty() {
            line.clear();
            continue;
        }

        for (col, c) in line.trim().chars().enumerate() {
            match c {
                '#' => {
                    maze.grid.insert((row, col as u64), Obstacle::Wall);
                }
                '.' => {
                    maze.grid.insert((row, col as u64), Obstacle::Free);
                },
                'E' => {
                    maze.grid.insert((row, col as u64), Obstacle::End);
                },
                'S' => {
                    reindeer.pos = (row, col as u64);
                    maze.grid.insert((row, col as u64), Obstacle::Free);
                }
                other => panic!("Unknown char {other}")
            }
        }
        row += 1;
        line.clear();
    }
    maze.start = reindeer.pos;
    (reindeer, maze)
}

fn part_1(filename: &str) {
    let (reindeer, maze) = parse_file(filename);
    let score = search(reindeer, &maze).first().unwrap().score;
    println!("Part 1: {score}");
}

fn part_2(filename: &str) {
    let (reindeer, maze) = parse_file(filename);
    let mut visited = HashSet::new();
    let solns = search(reindeer, &maze);
    let best = solns.iter().map(|s| s.score).min().unwrap();
    for soln in solns.into_iter().filter(|s| s.score == best) {
        for (pos, _) in soln.path(&maze) {
            visited.insert(pos);
        }
    }
    println!("Part 2: {}", visited.len());
}

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}
