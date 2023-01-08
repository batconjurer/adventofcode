use std::fs::File;
use std::io::{BufRead, BufReader};

fn snafu_to_base10(snafu: &str) -> u64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(pow, coeff)| 5i64.pow(pow as u32) * match coeff {
            '2' => 2i64,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!()
        })
        .sum::<i64>() as u64
}

fn base5_to_10(base5: &[u8]) -> u64 {
    if base5.len() == 0 {
        return 0;
    }
    base5
        .into_iter()
        .rev()
        .enumerate()
        .map(|(pow, coeff)| 5u64.pow(pow as u32) * (*coeff as u64))
        .sum()
}

fn to_base5(mut base10: u64) -> Vec<u8> {
    let mut power = 1;
    let mut pow = loop {
        if 5u64.pow(power) > base10 {
            break power - 1;
        }
        power += 1;
    };
    let mut digits = vec![];
    while pow > 0 {
        let place = base10 / 5u64.pow(pow);
        base10 = base10.rem_euclid(5u64.pow(pow));
        digits.push(place as u8);
        pow -= 1;
    }
    digits.push(base10 as u8);
    digits
}

fn to_snafu(base10: u64) -> String {
    let mut base5: Vec<u8> = to_base5(base10);
    let mut snafu = vec![];
    while let Some((char, rest)) = process_digit(&base5) {
        snafu.push(char);
        base5 = rest;
    }
    snafu.into_iter().rev().collect::<String>()
}

fn process_digit(tail: &[u8]) -> Option<(char, Vec<u8>)> {
    if tail.len() == 0 {
        None
    } else {
        Some(match *tail.last().unwrap() {
            0 => ('0', tail[..tail.len()-1].to_vec()),
            1 => ('1', tail[..tail.len()-1].to_vec()),
            2 => ('2', tail[..tail.len()-1].to_vec()),
            3 => (
                '=',
                to_base5(base5_to_10(&tail[..tail.len()-1]) + 1)
            ),
            4 => (
                '-',
                to_base5(base5_to_10(&tail[..tail.len()-1]) + 1)
            ),
            _ => unreachable!(),
        })
    }
}

fn part_one(filename: &str)  {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut base10_sum = 0u64;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        base10_sum += snafu_to_base10(line.trim());
        line.clear();
    }
    println!("Part one: {}", to_snafu(base10_sum));
}
fn main() {
    part_one("input.txt");
}
