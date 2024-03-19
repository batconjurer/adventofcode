use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
enum Line {
    Vertical(usize),
    Horizontal(usize),
}

#[derive(Debug, PartialEq, Eq)]
struct Pattern {
    elements: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Pattern {
    fn is_symmetric(&self, line: Line) -> bool {
        match line {
            Line::Vertical(line) => {
                let cols_to_check = std::cmp::min(line + 1, self.cols - 1 - line);
                for row in 0..self.rows {
                    let row = &self.elements[row];
                    for col in 1..=cols_to_check {
                        let upper = col + line;
                        let lower = line + 1 - col;
                        if row[upper] != row[lower] {
                            return false;
                        }
                    }
                }
                true
            }
            Line::Horizontal(line) => {
                let rows_to_check = std::cmp::min(line + 1, self.rows - 1 - line);
                for row in 1..=rows_to_check {
                    for col in 0..self.cols {
                        let upper = row + line;
                        let lower = line + 1 - row;
                        if self.elements[upper][col] != self.elements[lower][col] {
                            return false;
                        }
                    }
                }
                true
            }
        }
    }

    fn find_symmetry(&self) -> Option<Line> {
        for row in 0..self.rows - 1 {
            if self.is_symmetric(Line::Horizontal(row)) {
                return Some(Line::Horizontal(row));
            }
        }
        for col in 0..self.cols - 1 {
            if self.is_symmetric(Line::Vertical(col)) {
                return Some(Line::Vertical(col));
            }
        }
        None
    }

    fn fix_smudge(&self) -> Option<Line> {
        let old_symmetry = self.find_symmetry().expect("Couldn't find a symmetry");
        for row in 0..self.rows {
            for col in 0..self.cols {
                let mut new_pattern = self.elements.clone();
                match new_pattern[row][col] {
                    '.' => new_pattern[row][col] = '#',
                    '#' => new_pattern[row][col] = '.',
                    _ => unreachable!(),
                }
                let new_pattern = Pattern{ elements: new_pattern, rows: self.rows, cols: self.cols};
                let new_symmetry = {
                    for row in 0..self.rows - 1 {
                        if new_pattern.is_symmetric(Line::Horizontal(row)) {
                            if old_symmetry != Line::Horizontal(row) {
                                return Some(Line::Horizontal(row));
                            }
                        }
                    }
                    for col in 0..self.cols - 1 {
                        if new_pattern.is_symmetric(Line::Vertical(col)) {
                            if old_symmetry != Line::Vertical(col) {
                                return Some(Line::Vertical(col));
                            }
                        }
                    }
                    None
                };
                if new_symmetry.is_some() {
                    return new_symmetry;
                }
            }
        }
        None
    }
}

fn parse(filename: &str) -> Vec<Pattern> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut pattern = vec![];
    let mut patterns = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            patterns.push(Pattern {
                elements: pattern.clone(),
                rows: pattern.len(),
                cols: pattern[0].len(),
            });
            pattern.clear();
            break;
        }
        if line.trim().is_empty() {
            patterns.push(Pattern {
                elements: pattern.clone(),
                rows: pattern.len(),
                cols: pattern[0].len(),
            });
            pattern.clear();
        } else {
            pattern.push(line.trim().chars().collect::<Vec<_>>());
        }
        line.clear();
    }
    patterns
}

fn part_one(filename: &str) -> usize {
    let patterns = parse(filename);
    let mut total = 0;
    for pattern in patterns {
        total += match pattern.find_symmetry().expect("Couldn't find a symmetry") {
            Line::Vertical(line) => line + 1,
            Line::Horizontal(line) => 100 * line + 100,
        };
    }
    total
}

fn part_two(filename: &str) -> usize {
    let patterns = parse(filename);
    let mut total = 0;
    for pattern in patterns {
        total += match pattern.fix_smudge().expect("Couldn't find a smudge") {
            Line::Vertical(line) => line + 1,
            Line::Horizontal(line) => 100 * line + 100,
        };
    }
    total
}

fn main() {
    println!("Part one: {}", part_one("input.txt"));
    println!("Part two: {}", part_two("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smudge() {
        let pattern = Pattern {
            elements: vec![
                vec!['.', '.', '.', '#', '.', '#', '.', '#', '#'],
                vec!['.', '#', '#', '#', '#', '#', '.', '#', '#'],
                vec!['.', '#', '.', '#', '#', '.', '#', '.', '.'],
                vec!['.', '#', '.', '#', '#', '.', '#', '.', '.'],
                vec!['.', '#', '#', '#', '#', '#', '.', '#', '#'],
                vec!['.', '.', '.', '#', '.', '#', '.', '#', '#'],
                vec!['#', '#', '#', '.', '.', '#', '#', '.', '.'],
                vec!['#', '#', '#', '#', '.', '#', '#', '#', '#'],
                vec!['#', '.', '.', '#', '.', '#', '.', '#', '.'],
                vec!['#', '#', '.', '.', '.', '#', '.', '.', '.'],
                vec!['.', '.', '.', '#', '#', '#', '.', '.', '.'],
                vec!['#', '#', '#', '#', '#', '.', '.', '.', '.'],
                vec!['#', '.', '.', '#', '#', '.', '.', '#', '#']
            ],
            rows: 13,
            cols: 9
        };
        let symmetry = pattern.fix_smudge().expect("Test failed");
        assert_eq!(symmetry, Line::Vertical(7));
    }

    #[test]
    fn test_smudge_2() {
        let pattern = Pattern {
            elements: vec![
                vec!['#', '.', '#', '#', '.', '.', '.', '#', '#', '#', '#'],
                vec!['#', '#', '.', '.', '#', '#', '#', '.', '.', '.', '.'],
                vec!['#', '#', '#', '#', '#', '.', '.', '.', '#', '#', '#'],
                vec!['#', '#', '#', '.', '.', '#', '.', '#', '#', '.', '.'],
                vec!['.', '#', '#', '.', '.', '.', '.', '#', '#', '.', '.'],
                vec!['.', '#', '#', '.', '.', '.', '.', '.', '#', '.', '.'],
                vec!['#', '#', '#', '.', '.', '#', '.', '#', '#', '.', '.']
            ],
            rows: 7,
            cols: 11
        };
        let symmetry = pattern.find_symmetry().expect("Test failed");
        assert_eq!(symmetry, Line::Vertical(9));
        let new_symmetry = pattern.fix_smudge().expect("Test failed");
        assert_eq!(new_symmetry, Line::Horizontal(4));

    }
}