use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::ops::Index;
use std::str::Chars;

use either::Either;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Add,
    Mul
}

#[derive(Default, Clone, Debug)]
pub struct ParsedNum {
    value: u64,
    repr: [char; 4],
    span: [u64; 2],
}

impl Index<usize> for ParsedNum {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        &self.repr[index]
    }
}

/// A cursor that parses numbers and spans as it traverses over a string
/// (intended to be line in a file)
pub struct Cursor<'a> {
    unparsed: Peekable<Chars<'a>>,
    index: u64,
}

impl<'a> Cursor<'a> {

    fn new(line: &'a str) -> Self {
        Self {
            unparsed: line.chars().peekable(),
            index: 0,
        }
    }

    /// given a string of numbers seperated by an unknown number of spaces, split off
    /// the first number and return the parsed number and remaining string, if possible.
    /// We also compute the span of the number (start and end position in the string)
    fn next_number(&mut self) -> Option<ParsedNum> {
        // move past whitespace
        while self.unparsed.next_if_eq(&' ').is_some() {
            self.index += 1;
        }
        // check if there is another number to parse and start span
        let span_start = match self.unparsed.peek() {
            Some(c) if c.is_digit(10) => self.index,
            _ => return None,
        };
        // parse the next number
        let mut num = String::new();
        let mut repr = [' '; 4];
        let mut repr_ix = 0;
        while let Some(c) = self.unparsed.next_if(|c| c.is_digit(10)) {
            num.push(c);
            repr[repr_ix] = c;
            repr_ix += 1;
            self.index += 1;
        }
        let value = u64::from_str_radix(&num, 10)
            .expect(&format!("Failed to parse number {}", num));

        Some(ParsedNum {
            value,
            repr,
            span: [span_start, span_start + (num.len() as u64) - 1],
        })
    }
}

#[derive(Default, Clone, Debug)]
pub struct Parsed {
    first: Vec<ParsedNum>,
    second: Vec<ParsedNum>,
    third: Vec<ParsedNum>,
    fourth: Vec<ParsedNum>,
    ops: Vec<Op>,
}

impl Parsed {
    fn process_blocks(&mut self) {
        for block in 0..self.ops.len() {
            let block_start = *[
                self.first[block].span[0],
                self.second[block].span[0],
                self.third[block].span[0],
                self.fourth[block].span[0],
            ].iter().min().unwrap() as usize;

            let block_end = *[
                self.first[block].span[1],
                self.second[block].span[1],
                self.third[block].span[1],
                self.fourth[block].span[1],
            ].iter().max().unwrap() as usize;
            assert!(block_end - block_start < 4);
            let first_shift = self.first[block].span[0] as usize - block_start;
            let second_shift = self.second[block].span[0] as usize - block_start;
            let third_shift = self.third[block].span[0] as usize - block_start;
            let fourth_shift = self.fourth[block].span[0] as usize - block_start;
            self.first[block].repr.as_mut_slice().rotate_right(first_shift);
            self.second[block].repr.as_mut_slice().rotate_right(second_shift);
            self.third[block].repr.as_mut_slice().rotate_right(third_shift);
            self.fourth[block].repr.as_mut_slice().rotate_right(fourth_shift);
        }
    }

    fn fetch_block(&self, ix: usize, op: Op) -> [u64; 4] {
        let default = match op {
            Op::Add => 0u64,
            Op::Mul => 1,
        };
        let mut block = [default; 4];
        for i in 0..4usize {
            let mut num = String::new();
            num.push(self.first[ix][i]);
            num.push(self.second[ix][i]);
            num.push(self.third[ix][i]);
            num.push(self.fourth[ix][i]);
            if let Ok(n) = u64::from_str_radix(num.trim() , 10) {
                block[i] = n;
            }
        }
        block
    }
}

fn parse(filename: &str) -> Parsed {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut row = 1;
    let mut row2 = Vec::<ParsedNum>::new();
    let mut row3 = Vec::<ParsedNum>::new();
    let mut row4 = Vec::<ParsedNum>::new();
    let mut row5 = Vec::<ParsedNum>::new();
    let mut ops = Vec::<Op>::new();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        match row {
            1 => parse_line(&line, Either::Left(&mut ops)),
            2 => parse_line(&line, Either::Right(&mut row2)),
            3 => parse_line(&line, Either::Right(&mut row3)),
            4 => parse_line(&line, Either::Right(&mut row4)),
            5 => parse_line(&line, Either::Right(&mut row5)),
            _ => {}
        }

        row += 1;
        line.clear();
    }
    let len = row2.len();
    assert_eq!(len, row3.len());
    assert_eq!(len, row4.len());
    assert_eq!(len, row5.len());
    assert_eq!(len, ops.len());
    Parsed {first: row2, second: row3, third: row4, fourth: row5, ops}
}

fn parse_line(line: &str, row: Either::<&mut Vec<Op>, &mut Vec<ParsedNum>>) {
    match row {
        Either::Left(ops) => {
            for c in line.trim().chars() {
                match c {
                    '*' => ops.push(Op::Mul),
                    '+' => ops.push(Op::Add),
                    _  => {}
                }
            }
        }
        Either::Right(row) => {
            let mut to_parse = Cursor::new(line);
            while let Some(num) = to_parse.next_number() {
                row.push(num);
            }
        }
    }
}

fn part_one(filename: &str) {
    let parsed = parse(filename);
    let mut total = 0;
    for i in 0..parsed.first.len() {
        total += match parsed.ops[i] {
            Op::Add => parsed.first[i].value + parsed.second[i].value + parsed.third[i].value + parsed.fourth[i].value,
            Op::Mul => parsed.first[i].value * parsed.second[i].value * parsed.third[i].value * parsed.fourth[i].value,
        }
    }
    println!("Part one: {total}");
}

fn part_two(filename: &str) {
    let mut parsed = parse(filename);
    parsed.process_blocks();
    let mut total = 0;
    for (block, op) in parsed.ops.iter().enumerate() {
        let nums = parsed.fetch_block(block, *op);
        total += match op {
            Op::Add => nums.iter().sum::<u64>(),
            Op::Mul => nums.iter().product::<u64>(),
        }
    }

    println!("Part two: {total}");
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
}
