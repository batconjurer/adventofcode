use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
struct Grid {
    size: u64,
    trail_heads: Vec<(u64, u64)>,
    entries: HashMap<(u64, u64), u64>,
}

impl Grid {
    fn neighbors(&self, pos: (u64, u64)) -> [Option<(u64, u64)>; 4] {
        let mut neighbors = [None; 4];
        let curr_level = self.entries[&pos];
        if curr_level == 9 {
            return neighbors;
        }
        if pos.0 > 0 && self.entries[&(pos.0 - 1, pos.1)] == curr_level + 1 {
            neighbors[0] = Some((pos.0 - 1, pos.1));
        }
        if pos.1 > 0  && self.entries[&(pos.0, pos.1 - 1)] == curr_level + 1 {
            neighbors[1] = Some((pos.0, pos.1 - 1));
        }
        if pos.0 < self.size - 1 && self.entries[&(pos.0 + 1, pos.1)] == curr_level + 1 {
            neighbors[2] = Some((pos.0 + 1, pos.1));
        }
        if pos.1 < self.size - 1 && self.entries[&(pos.0, pos.1 + 1)] == curr_level + 1 {
             neighbors[3] = Some((pos.0, pos.1 + 1));
        }
        neighbors
    }
}

fn search_trailhead(pos: (u64, u64), grid: &Grid) -> usize {
    let mut queue = VecDeque::from([pos]);

    let mut ends = HashSet::<(u64, u64)>::new();
    let mut visited = HashSet::<(u64, u64)>::new();

    while let Some(next) = queue.pop_front() {
        for n in grid.neighbors(next).into_iter().filter_map(|x| x) {
            if visited.contains(&n) {
                continue;
            }
            if grid.entries[&n] == 9 {
                ends.insert(n);
            } else {
                queue.push_back(n);
                visited.insert(n);
            }
        }
    }

    ends.len()
}

fn search_trails(pos: (u64, u64), grid: &Grid) -> usize {
    let mut queue = Vec::from([pos]);
    let mut ends = 0usize;

    while let Some(next) = queue.pop() {
        for n in grid.neighbors(next).into_iter().filter_map(|x| x) {

            if grid.entries[&n] == 9 {
                ends += 1;
            } else {
                queue.push(n);
            }
        }
    }

    ends
}

fn parse(filename: &str) -> Grid {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut grid = Grid::default();
    let mut row = 0u64;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }

        for (col, c) in line.trim().chars().enumerate() {
            let level = c.to_digit(10).unwrap() as u64;
            grid.entries.insert((row, col as u64), level);
            if level == 0 {
                grid.trail_heads.push((row, col as u64));
            }
        }
        row += 1;
        line.clear();
    }
    grid.size = row;
    grid
}

fn part_1(filename: &str) {
    let mut grid = parse(filename);
    let trailheads = std::mem::take(&mut grid.trail_heads);
    let total = trailheads
        .into_iter()
        .fold(0usize, |mut acc, head| {
            acc += search_trailhead(head, &grid);
            acc
        });
    println!("Part 1: {total}");
}

fn part_2(filename: &str) {
    let mut grid = parse(filename);
    let trailheads = std::mem::take(&mut grid.trail_heads);
    let total = trailheads
        .into_iter()
        .fold(0usize, |mut acc, head| {
            acc += search_trails(head, &grid);
            acc
        });
    println!("Part 2: {total}");
}


fn main() {
    part_1("input.txt");
    part_2("input.txt");
}
