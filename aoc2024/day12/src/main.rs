use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Region {
    perimeter: u64,
    sides: u64,
    min_row: u64,
    max_row: u64,
    min_col: u64,
    max_col: u64,
    entries: BTreeSet<(u64, u64)>
}

impl Region {
    fn calculate_perimeter(&mut self) {
        for (row, col) in &self.entries {
            // perimeter calculations
            if *row == 0 {
                self.perimeter += 1;
            } else if !self.entries.contains(&(row - 1, *col)) {
                self.perimeter += 1;
            }
            if *col == 0 {
                self.perimeter += 1;
            } else if !self.entries.contains(&(*row, col - 1)) {
                self.perimeter += 1;
            }
            if !self.entries.contains(&(row + 1, *col)) {
                self.perimeter += 1;
            }
            if !self.entries.contains(&(*row, col + 1)) {
                self.perimeter += 1;
            }
        }
    }

    fn compute_sides(&mut self) {
        for row in self.min_row..=self.max_row {
            self.find_horiz_sides_at(row);
        }
        for col in self.min_col..=self.max_col{
            self.find_vert_sides_at(col);
        }
    }

    /// Determine if the given row in the region touches
    /// any sides from above / below. If so, count them.
    fn find_horiz_sides_at(&mut self, row: u64) {
        let mut sides = 0;
        let  mut last_below = None;
        let  mut last_above= None;

        for (r, c) in self.entries.iter().filter(|(r, _)| *r == row) {
            // check if line above is a side
            if *r == 0 || !self.entries.contains(&(r - 1, *c))  {
                // check if this is a new side on same line
                if *c == 0  || last_above != Some(c - 1) {
                    sides += 1;
                }
                last_above = Some(*c);
            }
            // check if line below is a side
            if !self.entries.contains(&(r + 1, *c))  {
                // check if this is a new side on same line
                if *c == 0 || last_below != Some(c - 1) {
                    sides += 1;
                }
                last_below = Some(*c);
            }
        }
        self.sides += sides;
    }

    /// Determine if the given col in the region touches
    /// any sides from the left / right. If so, count them.
    fn find_vert_sides_at(&mut self, col: u64) {
        let mut sides = 0;
        let  mut last_left = None;
        let  mut last_right = None;

        for (r, c) in self.entries.iter().filter(|(_, c)| *c == col) {
            // check if line to the left is a side
            if *c == 0 || !self.entries.contains(&(*r, c - 1))  {
                // check if this is a new side on same line
                if *r == 0  || last_left != Some(r - 1) {
                    sides += 1;
                }
                last_left = Some(*r);
            }
            // check if line to the right is a side
            if !self.entries.contains(&(*r, c + 1))  {
                // check if this is a new side on same line
                if *r == 0 || last_right != Some(r - 1) {
                    sides += 1;
                }
                last_right = Some(*r);
            }
        }
        self.sides += sides;
    }
}

#[derive(Default)]
struct Grid {
    size: u64,
    entries: HashMap<(u64, u64), char>,
}

impl Grid {
    fn neighbors(&self, pos: (u64, u64)) -> [Option<(u64, u64)>; 4] {
        let mut nghbrs = [None; 4];
        if pos.0 > 0 {
            nghbrs[0] = Some((pos.0 - 1, pos.1));
        }
        if pos.1 > 0 {
            nghbrs[1] = Some((pos.0, pos.1- 1));
        }
        if pos.0 < self.size - 1 {
            nghbrs[2] = Some((pos.0 + 1, pos.1));
        }
        if pos.1 < self.size - 1 {
            nghbrs[3] = Some((pos.0, pos.1 + 1));
        }
        nghbrs
    }

    fn create_region(&mut self, start: (u64, u64)) -> Region {
        let letter = self.entries.get(&start).cloned().unwrap();
        let mut region = Region {
            perimeter: 0,
            sides: 0,
            min_row: start.0,
            max_row: start.0,
            min_col: start.1,
            max_col: start.1,
            entries: BTreeSet::from([start]),
        };
        let mut queue = VecDeque::from([start]);
        self.entries.remove(&start);
        while let Some(next) = queue.pop_front() {
            for n in self.neighbors(next).into_iter().filter_map(|x| x) {
                if self.entries.get(&n) != Some(&letter) {
                    continue;
                } else {
                    self.entries.remove(&n);
                    if !region.entries.contains(&n) {
                        region.min_row = min(region.min_row, n.0);
                        region.max_row = max(region.max_row, n.0);
                        region.min_col = min(region.min_col, n.1);
                        region.max_col = max(region.max_col, n.1);
                        region.entries.insert(n);
                        queue.push_back(n);
                    }
                }
            }
        }
        region
    }

    fn create_regions(&mut self) -> Vec<Region> {
        let mut regions = vec![];
        loop {
            if self.entries.is_empty() {
                break;
            }
            let any = *self.entries.keys().next().unwrap();
            regions.push(self.create_region(any))
        }
        regions
    }
}

fn parse_file(filename: &str) -> Grid {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut grid = Grid::default();
    let mut row = 0u64;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }

        for (col, c) in  line.trim().chars().enumerate() {
            grid.entries.insert((row , col as u64), c);
        }
        line.clear();
        row += 1;
    }
    grid.size = row;
    grid
}

fn part_1(filename: &str) {
    let regions = parse_file(filename).create_regions();
    let sum: u64 = regions.into_iter()
        .map(|mut r| {
            r.calculate_perimeter();
            r.perimeter * (r.entries.len() as u64)
        })
        .sum();
    println!("Part 1: {}", sum);
}

fn part_2(filename: &str) {
    let regions = parse_file(filename).create_regions();
    let sum: u64 = regions.into_iter()
        .map(|mut r| {
            r.compute_sides();
            r.sides * (r.entries.len() as u64)
        })
        .sum();
    println!("Part 2: {}", sum);
}

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}
