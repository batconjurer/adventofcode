use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn parse_input(filename: &str) -> Engine {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut line_number = 0;
    let mut next_word: Option<Word> = None;
    let mut engine = Engine::default();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if let Some(word) = next_word.as_mut() {
                    word.length += 1;
                    word.val.push(c);
                } else {
                    next_word = Some(Word{head: (x, line_number), length: 1, val: c.into()});
                }
            } else {
                if let Some(word) = next_word.take() {
                    engine.words.push(word);
                }
                if c != '.' && c != '\n' {
                    engine.symbols.insert((x, line_number, c));
                }
            }
        }

        line.clear();
        line_number += 1;
    }
    engine
}

#[derive(Debug)]
struct Word {
    head: (usize, usize),
    length: usize,
    val: String,
}

impl Word {
    fn adjacent(&self, (x, y): (usize, usize)) -> Option<u64> {
        let (x, y) = (x as i64, y as i64);
        let (head_x, head_y) = (self.head.0 as i64, self.head.1 as i64);
        if head_x - 1 <= x && x <= head_x + self.length as i64 && (y - head_y).abs() <= 1 {
            Some(u64::from_str(&self.val).unwrap())
        } else {
            None
        }
    }
}

#[derive(Default, Debug)]
struct Engine {
    words: Vec<Word>,
    symbols: HashSet<(usize, usize, char)>,
}

impl Engine {
    fn part_one(&self) -> u64 {
        let mut total = 0;
        for word in &self.words {
            for (x, y, _) in &self.symbols {
                if let Some(val) = word.adjacent((*x, *y)) {
                    total += val;
                    break;
                }
            }
        }
        total
    }

    fn part_two(&self) -> u64 {
        let mut total = 0;
        for (x, y, symbol) in  &self.symbols {
            if *symbol == '*' {
                let mut adjacent = 0;
                let mut gear_ratio: u64 = 1;
                for word in &self.words {
                    if let Some(val) = word.adjacent((*x, *y)) {
                        if adjacent > 2 {
                            break;
                        }
                        adjacent += 1;
                        gear_ratio *= val;
                    }
                }
                if adjacent == 2 {
                    total += gear_ratio;
                }
            }
        }
        total
    }
}

fn main() {
    let engine = parse_input("input.txt");
    println!("Part one: {}", engine.part_one());
    println!("Part two: {}", engine.part_two());
}
