use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Iter;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Square {
    X, M, A, S,
}

#[derive(Copy, Clone)]
enum Direction {
    Northwest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl Direction {
    fn iter() -> impl Iterator<Item=Self> {
        [
            Self::Northwest,
            Self::North,
            Self::NorthEast,
            Self::East,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::West
        ].into_iter()
    }
}

impl Square {
    fn comes_before(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::X, Self::M) | (Self::M, Self::A) | (Self::A, Self::S) => true,
            _ => false
        }
    }
}

impl TryFrom<char> for Square {
    type Error = String;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'X' => Ok(Self::X),
            'M' => Ok(Self::M),
            'A' => Ok(Self::A),
            'S' => Ok(Self::S),
            e => Err(format!("Unrecognized character: {e}")),
        }
    }
}

#[derive(Default, Debug)]
struct Grid {
    max_row: usize,
    max_col: usize,
    entries: HashMap<(usize, usize), Square>,
}

impl Grid {
    fn get_neighbor(
        &self,
        pos: (usize, usize),
        dir: Direction
    ) -> Option<((usize, usize), Square)> {
        match dir {
            Direction::Northwest => if pos.1 > 0 && pos.0 > 0 {
                self.entries.get(&(pos.0 - 1, pos.1 - 1))
                    .map(|x| ((pos.0 - 1, pos.1 - 1), *x))
            } else {
                None
            }
            Direction::North => if pos.0 > 0 {
                self.entries.get(&(pos.0 - 1, pos.1))
                    .map(|x| ((pos.0 - 1, pos.1), *x))
            } else {
                None
            }
            Direction::NorthEast => if pos.0 > 0 {
                self.entries.get(&(pos.0 - 1, pos.1 + 1))
                    .map(|x| ((pos.0 - 1, pos.1 + 1), *x))
            } else {
                None
            }
            Direction::East => self.entries.get(&(pos.0, pos.1 + 1))
                .map(|x| ((pos.0, pos.1 + 1), *x)),
            Direction::SouthEast => self.entries.get(&(pos.0 + 1, pos.1 + 1))
                .map(|x| ((pos.0 + 1, pos.1 + 1), *x)),
            Direction::South => self.entries.get(&(pos.0 + 1, pos.1))
                .map(|x| ((pos.0 + 1, pos.1), *x)),
            Direction::SouthWest => if pos.1 > 0 {
                self.entries.get(&(pos.0 + 1, pos.1 - 1))
                    .map(|x| ((pos.0 + 1, pos.1 - 1), *x))
            } else {
                None
            }
            Direction::West => if pos.1 > 0 {
                self.entries.get(&(pos.0, pos.1 - 1))
                    .map(|x| ((pos.0, pos.1 - 1), *x))
            } else {
                None
            }
        }
    }
}

fn search_word(root: (usize, usize), grid: &Grid) -> u64 {
    let mut count = 0;
    for dir in Direction::iter() {
        let mut stack = Some((root, Square::X));
        while let Some((next_pos, next_sq)) = stack.take() {
            let Some((nghbr_pos, nghbr_sq)) = grid.get_neighbor(next_pos, dir) else {
                continue
            };
            if next_sq.comes_before(&nghbr_sq) {
                if nghbr_sq == Square::S {
                    count += 1;
                    continue
                }
                stack = Some((nghbr_pos, nghbr_sq));
            }
        }
    }
    count
}

fn find_x(root: (usize, usize), grid: &Grid) -> bool {
    let corners = [
        grid.get_neighbor(root, Direction::Northwest).map(|x| x.1),
        grid.get_neighbor(root, Direction::NorthEast).map(|x| x.1),
        grid.get_neighbor(root, Direction::SouthEast).map(|x| x.1),
        grid.get_neighbor(root, Direction::SouthWest).map(|x| x.1),
    ];

    corners == [Some(Square::M), Some(Square::M), Some(Square::S), Some(Square::S)] ||
        corners == [Some(Square::M), Some(Square::S), Some(Square::S), Some(Square::M)] ||
        corners == [Some(Square::S), Some(Square::S), Some(Square::M), Some(Square::M)] ||
        corners == [Some(Square::S), Some(Square::M), Some(Square::M), Some(Square::S)]

}

fn parse_file(filename: &str, root_char: char) -> (Vec<(usize, usize)>, Grid) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut roots = vec![];
    let mut grid = Grid::default();

    let mut row = 0usize;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }

        for (col, c) in  line.trim().chars().enumerate() {
            let square = Square::try_from(c).unwrap();
            if c == root_char {
                roots.push((row, col));
            }
            grid.entries.insert((row, col), square);
            if row == 0 {
                grid.max_col = std::cmp::max(grid.max_col, col);
            }
        }
        grid.max_row = row;
        line.clear();
        row += 1;
    }
    (roots, grid)
}

fn part_1(filename: &str) {
    let (roots, grid) = parse_file(filename, 'X');
    let mut count = 0;
    for root in roots {
        count += search_word(root, &grid);
    }
    println!("Part 1: {count}");
}

fn part_2(filename: &str) {
    let (roots, grid) = parse_file(filename, 'A');
    let mut count = 0;
    for root in roots {
        if find_x(root, &grid) {
            count += 1;
        }
    }
    println!("Part 2: {count}");
}

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}
