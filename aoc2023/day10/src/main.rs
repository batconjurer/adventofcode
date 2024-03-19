use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PipeType {
    Horizontal,
    Vertical,
    J,
    L,
    F,
    Seven,
    None,
    Start,
}

impl PipeType {
    /// Given a pipe and a position in the infinite positive quadrant,
    /// find the (up to) two neighboring positions
    fn endpoints(&self, pos: (usize, usize)) -> [Option<(usize, usize)>; 2] {
        match self {
            PipeType::Horizontal => {
                if pos.1 == 0 {
                    [None, Some((pos.0, pos.1 + 1))]
                } else {
                    [Some((pos.0, pos.1 - 1)), Some((pos.0, pos.1 + 1))]
                }
            }
            PipeType::Start | PipeType::Vertical => {
                if pos.0 == 0 {
                    [None, Some((pos.0 + 1, pos.1))]
                } else {
                    [Some((pos.0 + 1, pos.1)), Some((pos.0 - 1, pos.1))]
                }
            }
            PipeType::J => [
                if pos.0 == 0 {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                },
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                },
            ],
            PipeType::L => [
                if pos.0 == 0 {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                },
                Some((pos.0, pos.1 + 1)),
            ],
            PipeType::F => [Some((pos.0, pos.1 + 1)), Some((pos.0 + 1, pos.1))],
            PipeType::Seven => [
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                },
                Some((pos.0 + 1, pos.1)),
            ],
            PipeType::None => [None, None],
        }
    }
}

impl From<char> for PipeType {
    fn from(value: char) -> Self {
        match value {
            '|' => PipeType::Vertical,
            '-' => PipeType::Horizontal,
            'J' => PipeType::J,
            'L' => PipeType::L,
            'F' => PipeType::F,
            '7' => PipeType::Seven,
            '.' => PipeType::None,
            'S' => PipeType::Start,
            other => panic!("Unexpected character {}", other),
        }
    }
}

#[derive(Debug)]
struct Pipe {
    pipe_type: PipeType,
    pos: (usize, usize),
}

impl Pipe {

    /// Get the (up to two) neighbors of this pipe
    fn endpoints(&self) -> [Option<(usize, usize)>; 2] {
        self.pipe_type.endpoints(self.pos)
    }

    /// Check if this pipe and another pipe contain each other
    /// as neighbors
    fn adjacent(&self, other: &Self) -> bool {
        let mut a_sees_b = false;
        let mut b_sees_a = false;
        for neighbor in  self.endpoints() {
            if let Some(neighbor) = neighbor {
                if neighbor == other.pos {
                    a_sees_b = true;
                    break;
                }
            }
        }

        for neighbor in other.endpoints() {
            if let Some(neighbor) = neighbor {
                if neighbor == self.pos {
                    b_sees_a = true;
                    break;
                }
            }
        }
        a_sees_b && b_sees_a
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Pipe>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    /// Adds the upper bounds check of this grid to the neighbors of the
    /// pipe given by `pos`
    fn endpoints(&self, pos: (usize, usize)) -> [Option<(usize, usize)>; 2] {
        let pipe = &self.grid[pos.0][pos.1];
        let mut ends = pipe.endpoints();
        ends[0] = if let Some(e) = ends[0] {
            if e.0 < self.rows && e.1 < self.cols {
                Some(e)
            } else {
                None
            }
        } else {
            None
        };
        ends[1] = if let Some(e) = ends[1] {
            if e.0 < self.rows && e.1 < self.cols {
                Some(e)
            } else {
                None
            }
        } else {
            None
        };
        ends
    }

    /// Gets the (up to two) pipes connecting to the pipe at `pos`
    fn neighbors(&self, pos: (usize, usize)) -> [Option<(usize, usize)>; 2] {
        let pipe = &self.grid[pos.0][pos.1];
        let mut ends = self.endpoints(pos);
        ends[0] = if let Some(e) = ends[0] {
          if pipe.adjacent(&self.grid[e.0][e.1]) {
              Some(e)
          } else {
              None
          }
        } else {
            None
        };
        ends[1] = if let Some(e) = ends[1] {
          if pipe.adjacent(&self.grid[e.0][e.1]) {
              Some(e)
          } else {
              None
          }
        } else {
            None
        };
        ends
    }
}

#[derive(Debug)]
struct Visitor {
    start: (usize, usize),
    previous: (usize, usize),
    current: (usize, usize),
    path: HashSet<(usize, usize)>,
}

impl Visitor {

    fn new(start: (usize, usize)) -> Self {
        Self {
            start,
            previous: start,
            current: start,
            path: HashSet::from([start]),
        }
    }

    fn step(&mut self, grid: &Grid) {
        let next = grid.neighbors(self.current);
        if let Some(next) = next[0] {
            if next != self.previous {
                self.previous = self.current;
                self.current = next;
                self.path.insert(self.current);
                return
            }
        }
        if let Some(next) = next[1] {
            if next != self.previous {
                self.previous = self.current;
                self.current = next;
                self.path.insert(self.current);
                return
            }
        }
        panic!("Could not advance to the next step in the loop. This is a bug!")

    }

    fn find_loop(&mut self, grid: &Grid) -> usize {
        self.step(grid);
        while self.current != self.start {
            self.step(grid);
        }
        self.path.len() / 2
    }

    fn in_loop(&self, mut pos: (usize, usize), grid: &Grid) -> bool {
        let mut in_loop = false;
        if pos.0 == 0 || pos.1 == 0 {
            return false;
        }
        pos = (pos.0 - 1, pos.1 - 1);
        loop {
            if self.path.contains(&pos) {
                match &grid.grid[pos.0][pos.1].pipe_type {
                    PipeType::Horizontal | PipeType::Vertical | PipeType::F | PipeType::J | PipeType::Start => {
                        in_loop = !in_loop;
                    }
                    _ => {}
                }
            }
            if pos.0 == 0 || pos.1 == 0 {
                break;
            }
            pos = (pos.0 - 1, pos.1 - 1);
        }
        in_loop
    }

}

fn parse_file(filename: &str) -> (Grid, Visitor) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut grid = vec![];
    let mut row = 0usize;
    let mut start = (0usize, 0usize);
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let grid_line: Vec<_> = line.trim().chars().enumerate().map(|(col, c)|{
            if c == 'S' {
                start = (row, col);
            }
            Pipe{
                pipe_type: PipeType::from(c),
                pos: (row, col),
            }
        } ).collect();
        grid.push(grid_line);
        row += 1;
        line.clear()
    }
    let cols = grid[0].len();
    (Grid {
        grid,
        rows: row,
        cols,
    }, Visitor::new(start))
}

fn part_one(filename: &str) {
    let (grid, mut visitor) = parse_file(filename);
    let steps = visitor.find_loop(&grid);
    println!("Part one: {}", steps);
}

fn part_two(filename: &str) {
    let (grid, mut visitor) = parse_file(filename);
    visitor.find_loop(&grid);
    let mut total = 0u64;
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if !visitor.path.contains(&(row, col)) {
                if visitor.in_loop((row, col), &grid) {
                    total += 1;
                }
            }
        }
    }

    println!("Part two: {}", total);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt")
}
