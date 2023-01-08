use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(filename: &str) -> Vec<(u32, i64)> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut numbers = vec![];
    let mut line_number = 0;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        numbers.push((line_number, i64::from_str_radix(&line.trim(), 10).unwrap()));
        line.clear();
        line_number += 1;
    }
    numbers
}

fn mix(numbers: &[(u32, i64)], times: u8) -> Vec<(u32, i64)>{
    let length = numbers.len() as i64;
    let mut decrypted = numbers.to_owned();
    for _ in 0..times {
        for shift in numbers {
            let (ix, _) = decrypted
                .iter()
                .enumerate()
                .find(|(_, val)| **val == *shift)
                .unwrap();

            let next_ix = (shift.1 + ix as i64).rem_euclid(length - 1) as usize;
            decrypted.remove(ix);
            decrypted.insert(next_ix, *shift);
        }
    }
    decrypted
}

fn part_one(filename: &str) {
    let numbers = parse_input(filename);
    let decrypted = mix(&numbers, 1);
    let length = numbers.len();
    let (ix, _) = decrypted
        .iter()
        .enumerate()
        .find(|(_, val)| val.1 == 0)
        .unwrap();
    let first = (ix + 1000).rem_euclid(length as usize);
    let second = (ix + 2000).rem_euclid(length as usize);
    let third = (ix + 3000).rem_euclid(length as usize);
    println!("{}", decrypted[first].1 + decrypted[second].1 + decrypted[third].1);
}

fn part_two(filename: &str) {

    let numbers = parse_input(filename)
        .into_iter()
        .map(|(l, x)| (l, x * 811589153))
        .collect::<Vec<_>>();
    let decrypted = mix(&numbers, 10);
    let length = numbers.len();
    let (ix, _) = decrypted
        .iter()
        .enumerate()
        .find(|(_, val)| val.1 == 0)
        .unwrap();
    let first = (ix + 1000).rem_euclid(length as usize);
    let second = (ix + 2000).rem_euclid(length as usize);
    let third = (ix + 3000).rem_euclid(length as usize);
    println!("{}", decrypted[first].1 + decrypted[second].1 + decrypted[third].1);
}

fn main() {
   part_one("input.txt");
    part_two("input.txt");
}
