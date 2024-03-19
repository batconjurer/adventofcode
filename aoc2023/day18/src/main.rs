use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Dir {
    R, D, L, U
}

/// Indicates if a vertex in a simple rectilinear
/// polygon is concave/convex. An edge case occurs
/// if two moves travel in the same direction, represented
/// by `Internal`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Conv {
    Convex,
    Concave,
    Internal,
}

#[derive(Debug, Clone)]
struct Move {
    dir: Dir,
    amount: i64,
    hex: String,
}

/// The data of a continuator rectangle to be removed.
#[derive(Debug, Clone)]
struct Continuator {
    ix: usize,
    edge: [(i64, i64, Conv); 2],
    length: i64,
    prev_ix: usize,
    next_ix: usize,
    prev: (i64, i64, Conv),
    next: (i64, i64, Conv),
    prev_length: i64,
    next_length: i64,
}


struct Tunnel {
    boundary: Vec<(i64, i64, Conv)>,
}

fn parse_hex(hex: &str) -> Move {
    let mut hex = hex.replace('#', "");
    let dir = hex.pop().unwrap();
    let dir = match dir {
        '0' => Dir::R,
        '1' => Dir::D,
        '2' => Dir::L,
        '3' => Dir::U,
        _ => unreachable!(),
    };
    let amount = i64::from_str_radix(&hex, 16).unwrap();
    Move {
        dir,
        amount,
        hex: "".to_string(),
    }
}

impl Tunnel {
    fn find_boundary(moves: &[Move]) -> Self {
        let last_dir = moves.last().unwrap().dir;
        // The boundary is traversed in a clockwise manner. This allows us to easily
        // classify vertices as concave/convex
        let convexity = |first: Dir, second: Dir| match (first, second) {
            (Dir::U, Dir::R) | (Dir::D, Dir::L) | (Dir::R, Dir::D) | (Dir::L, Dir::U) => Conv::Convex,
            (Dir::U, Dir::U) | (Dir::D, Dir::D) | (Dir::R, Dir::R) | (Dir::L, Dir::L) => Conv::Internal,
            _ => Conv::Concave
        };
        let mut boundary = vec![];
        let mut last = (0i64, 0, last_dir);

        for m in moves {
            let next = match m.dir {
                Dir::R => (last.0, last.1 + m.amount),
                Dir::D => (last.0 + m.amount, last.1),
                Dir::L => (last.0, last.1 - m.amount),
                Dir::U => (last.0 - m.amount, last.1),
            };
            let convex = convexity(last.2, m.dir);
            boundary.push((last.0, last.1, convex));
            last = (next.0, next.1, m.dir);
        }
        Self {
            boundary,
        }
    }

    /// Find and delete a continuator rectangle, returning the number of
    /// points contained in it.
    ///
    /// First, find the smallest knob in the polygon, it must belong to a side
    /// of a continuator rectangle.
    ///
    /// Look at the two adjacent edges and determine if the edge can be pushed
    /// inward. If so, return the volume of points removed by this shrinking.
    ///
    /// The last case is when only a rectangle is left. Then return the area.
    fn next_continuator(&mut self) -> u64 {
        // find the smallest knob
        let Continuator { ix, edge,
            length,
            prev_ix,
            next_ix,
            prev,
            next,
            prev_length,
            next_length } = self.boundary.windows(2).enumerate()
            .filter_map(|(ix, edge)| if edge[0].2 == Conv::Convex && edge[1].2 == Conv::Convex {
                // get the prev and next nodes
                let (prev_ix, prev) = if ix == 0 {
                    (self.boundary.len() - 1, self.boundary[self.boundary.len() - 1])
                } else {
                    (ix - 1, self.boundary[ix - 1])
                };
                let (next_ix, next) = if ix == self.boundary.len() - 2 {
                    (0, self.boundary[0])
                } else {
                    (ix + 2, self.boundary[ix + 2])
                };
                // get previous and next edge lengths
                let prev_length = (edge[0].0 - prev.0).abs() + (edge[0].1 - prev.1).abs();
                let next_length = (edge[1].0 - next.0).abs() + (edge[1].1 - next.1).abs();
                let length = (edge[0].0 - edge[1].0).abs() + (edge[0].1 - edge[1].1).abs() + 1;
                if next.2 == Conv::Concave {
                    if prev.2 == Conv::Convex && next_length >= prev_length {
                        None
                    } else {
                        Some(Continuator{
                            ix,
                            edge: [edge[0], edge[1]],
                            length,
                            prev_ix,
                            next_ix,
                            prev,
                            next,
                            prev_length,
                            next_length,
                        })
                    }
                } else if prev.2 == Conv::Concave {
                    if next.2 == Conv::Convex && prev_length >= next_length {
                        None
                    } else {
                        Some(Continuator{
                            ix,
                            edge: [edge[0], edge[1]],
                            length,
                            prev_ix,
                            next_ix,
                            prev,
                            next,
                            prev_length,
                            next_length,
                        })
                    }
                } else if self.boundary.len() != 4 {
                    None
                } else {
                    Some(Continuator{
                        ix,
                        edge: [edge[0], edge[1]],
                        length,
                        prev_ix,
                        next_ix,
                        prev,
                        next,
                        prev_length,
                        next_length,
                    })
                }
            } else {
                None
            })
            .min_by_key(|c| c.length)
            .expect("An unexpected bug in removing a continuator rectangle has been reached");
        let is_horizontal = edge[0].0 == edge[1].0;
        // remove the continuator rectangle
        if prev_length == next_length {
            for index in [prev_ix, ix, ix + 1, next_ix]
                .into_iter()
                .sorted()
                .rev()
            {
                self.boundary.remove(index);
            }
        } else if prev_length < next_length {
            if is_horizontal {
                self.boundary[ix + 1].0 = prev.0;
            } else {
                self.boundary[ix + 1].1 = prev.1;
            }
            if ix > prev_ix {
                self.boundary.remove(ix);
                self.boundary.remove(prev_ix);
            } else {
                self.boundary.remove(prev_ix);
                self.boundary.remove(ix);
            }
        } else {
            if is_horizontal {
                self.boundary[ix].0 = next.0;
            } else {
                self.boundary[ix].1 = next.1;
            }
            if ix + 1 > next_ix {
                self.boundary.remove(ix + 1);
                self.boundary.remove(next_ix);
            } else {
                self.boundary.remove(next_ix);
                self.boundary.remove(ix + 1);
            }
        }
        if self.boundary.is_empty() {
            return ((prev_length + 1) * length) as u64;
        } else {
           return (min(prev_length, next_length) * length) as u64;
        }
    }
}


fn parse(filename: &str) -> Vec<Move> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut moves = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let mut chars = line.trim().split(" ");
        let dir = match chars.next().unwrap() {
            "R" => Dir::R,
            "D" => Dir::D,
            "L" => Dir::L,
            "U" => Dir::U,
            _ => unreachable!()
        };
        let amount = i64::from_str(chars.next().unwrap()).unwrap();
        let hex = chars.next().unwrap().replace('(', "").replace(')', "");
        moves.push(Move{
            dir,
            amount,
            hex
        });
        line.clear();
    }
    moves
}

fn part_one(filename: &str) {
    let moves = parse(filename);
    let mut tunnel = Tunnel::find_boundary(&moves);
    let mut volume = 0;

    while !tunnel.boundary.is_empty() {
        volume += tunnel.next_continuator();
    }
    println!("Part one: {}", volume);
}

fn part_two(filename: &str) {
    let moves = parse(filename)
        .into_iter()
        .map(|m| parse_hex(&m.hex))
        .collect::<Vec<_>>();
    let mut tunnel = Tunnel::find_boundary(&moves);
    let mut volume = 0;
    while !tunnel.boundary.is_empty() {
        volume += tunnel.next_continuator();
    }
    println!("Part two: {}", volume);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
}
