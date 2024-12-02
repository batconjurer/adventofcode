use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};


#[derive(Debug, Clone)]
struct Grid {
    start: (i64, i64),
    plots: HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn neighbors(&self, (row, col): (i64, i64)) -> [Option<(i64, i64)>; 4] {
        let inner_row = if row < 0 {
            let maybe_row = (row % self.rows as i64) + self.rows as i64;
             if maybe_row == self.rows as i64 {
                 0
             } else {
                 maybe_row
             }
        } else {
            row % self.rows as i64
        } as usize;
        let inner_col = if col < 0 {
            let maybe_col = (col % self.cols as i64) + self.cols as i64;
            if maybe_col == self.cols as i64 {
                0
            } else {
                maybe_col
            }
        } else {
            col % self.cols as i64
        } as usize;

        let mut neighbors: [Option<(i64, i64)>; 4] = [None; 4];
        if inner_row > 0 {
            neighbors[0] = self.plots
                .contains(&(inner_row - 1, inner_col))
                .then_some((row - 1, col));
        } else {
            neighbors[0] = self.plots
                .contains(&(self.rows - 1, inner_col))
                .then_some((row - 1, col));
        }
        if inner_col > 0 {
            neighbors[1] = self.plots
                .contains(&(inner_row, inner_col - 1))
                .then_some((row, col - 1));
        } else {
            neighbors[1] = self.plots
                .contains(&(inner_row, self.cols - 1))
                .then_some((row, col - 1));
        }
        if inner_row + 1 == self.rows {
            neighbors[2] = self.plots
                .contains(&(0, inner_col))
                .then_some((row + 1, col));
        } else {
            neighbors[2] = self.plots
                .contains(&(inner_row + 1, inner_col))
                .then_some((row + 1, col));
        }
        if inner_col + 1 == self.cols {
            neighbors[3] = self.plots
                .contains(&(inner_row, 0))
                .then_some((row, col + 1));
        } else {
            neighbors[3] = self.plots
                .contains(&(inner_row, inner_col + 1))
                .then_some((row, col + 1));
        }

        neighbors
    }
}

fn parse(filename: &str) -> Grid{
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut grid = Grid {
        start: (0, 0),
        plots: Default::default(),
        rows: 0,
        cols: 0,
    };
    let mut odd_rocks = 0;
    let mut even_rocks = 0;
    let mut row = 0usize;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let mut cols = 0;
        for (col, c) in line.trim().chars().enumerate() {
            match c {
                '.' => _ = grid.plots.insert((row, col)),
                'S' => {
                    grid.plots.insert((row, col));
                    grid.start = (row as i64, col as i64);
                }
                '#' => {
                    if (row + col) % 2 == 0 {
                        even_rocks += 1;
                    } else {
                        odd_rocks += 1;
                    }
                },
                _ => unreachable!(),
            }
            cols = col;
        }
        row += 1;
        grid.cols = cols + 1;
        line.clear();
    }
    grid.rows = row;
    println!("Odd rocks {}, even rocks {}", odd_rocks, even_rocks);
    grid
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pos: (i64, i64),
    dist: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that then we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.dist.cmp(&self.dist)
            .then_with(|| self.pos.0.cmp(&other.pos.0)
                .then_with(|| self.pos.1.cmp(&other.pos.1))
            )
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_one(filename: &str) {
    let grid = parse(filename);
    let reachable = step_counter(&grid,  131*4 + 65, grid.start);
    println!("Part one: {}", reachable);
}

#[allow(dead_code)]
fn write_dists_to_file(distances: &HashMap<(i64, i64), usize>, grid: &Grid) {
    let min_row = distances
        .iter()
        .map(|((row, _), _)| *row)
        .min()
        .unwrap();
    let max_row = distances
        .iter()
        .map(|((row, _), _)| *row)
        .max()
        .unwrap();
    let min_col = distances
        .iter()
        .map(|((_, col), _)| *col)
        .min()
        .unwrap();
    let max_col = distances
        .iter()
        .map(|((_, col), _)| *col)
        .max()
        .unwrap();
    let mut file = File::create("dists.txt").unwrap();
    for row in min_row..=max_row {
        let mut line = String::new();
        for col in min_col..=max_col {
            let inner_row = if row < 0 {
                (row % grid.rows as i64) + grid.rows as i64
            } else {
                row % grid.rows as i64
            } as usize;
            let inner_col = if col < 0 {
                (col % grid.cols as i64) + grid.cols as i64
            } else {
                col % grid.cols as i64
            } as usize;
            if let Some(dist) = distances.get(&(row, col)) {
                if *dist >= 100 {
                    line.push_str(&format!(" {}", dist));
                } else if *dist >= 10 {
                    line.push_str(&format!("  {}", dist));
                } else {
                    line.push_str(&format!("   {}", dist));
                }
            } else if grid.plots.contains(&(inner_row, inner_col)) {
                line.push_str("   .");
            } else {
                line.push_str("   #");
            }
        }
        writeln!(file, "{}", line).unwrap();
        line.clear();
    }
}
fn step_counter(grid: &Grid, max_steps: usize, start_point: (i64, i64)) -> usize {
    let mut queue = BinaryHeap::new();
    queue.push(State{ pos: start_point, dist: 0usize});
    let mut distances = HashMap::from([(start_point, 0usize)]);
    while let Some(State{pos: next, ..}) = queue.pop() {
        for neighbor in grid.neighbors(next)
            .into_iter()
            .filter_map(|x| x)
        {

            let new_dist = distances.get(&next).unwrap() + 1;
            let old_dist = distances.get(&neighbor).cloned().unwrap_or(usize::MAX);
            if new_dist < old_dist {
                distances.insert(neighbor, new_dist);
                if new_dist <= max_steps {
                    queue.push(State { pos: neighbor, dist: new_dist });
                }
            }
        }
    }

    distances.iter()
        .filter_map(|((r, h), dist)| {
            (*dist <= max_steps && (r + h)% 2 == (grid.start.0 + grid.start.1 + (max_steps as i64 % 2)) % 2).then_some((*r, *h))
        }).count()

}

fn part_two(scale: u64) {
    //scale = 202300;
    // 65 steps from:
    // center: 7461
    // top left: 7433
    // top right: 7433
    // bottom left: 7429
    // bottom right: 7429
    let side = 2 * scale + 1;
    let square = side * side;
    let centers = square / 2 + 1;
    let off_centers = square - centers;
    let steps = centers * 7461 + off_centers * 7433 + 2*scale*scale;
    println!("Steps: {}", steps  );
   // println!("Border: {}", steps - interior);

}

fn main() {
    part_one("input.txt");
    part_two(202300);
}

