use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, SeekFrom};

fn parse_file(filename: &str) -> Vec<u64>  {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut seeds = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }

        seeds.push(u64::from_str_radix(line.trim(), 10).unwrap());
        line.clear();
    }
    seeds
}


/// Bitwise XOR
fn mix(num: u64, val: u64) -> u64 {
    num ^ val
}

/// Modulo 2^24
fn prune(num: u64) -> u64 {
    const MODULATOR: u64 = 16777215;
    num & MODULATOR
}

fn evolve(mut num: u64) -> u64 {
    // ( num XOR (num * 64) ) mod (2 ^ 24)
    num = prune(mix(num, num << 6));
    // ( num XOR (num // 32) ) mod (2 ^ 24)
    num = prune(mix(num, num >> 5));
    // ( num XOR (num * 2048) ) mode (2 ^ 24)
    prune(mix(num, num << 11))
}

///  The one's place of the number
fn price(num: u64) -> i8 {
    num.rem_euclid(10) as i8
}

fn gen_diffs_and_prices(mut seed: u64) -> ([i8; DIFFS_NUM], [i8; DIFFS_NUM + 1]) {
    let mut prices = [0; DIFFS_NUM + 1];
    let mut diffs = [0; DIFFS_NUM];
    for ix in 0..DIFFS_NUM {
        let next = evolve(seed);
        prices[ix] = price(seed);
        diffs[ix] = price(next) - price(seed);
        seed = next;
    }
    prices[DIFFS_NUM] = price(seed);
    (diffs, prices)
}

fn sell_value(seq: &[i8; 4], diffs: &[i8; DIFFS_NUM], prices: &[i8; DIFFS_NUM + 1]) -> u64 {
    for (ix, diff_seq) in diffs.windows(4).enumerate() {
        if diff_seq == seq {
            return prices[ix + 4] as u64;
        }
    }
    0
}

fn part1(nums: Vec<u64>) {
    let total: u64 = nums.into_iter().map(|mut n|{
        for _ in 0..2000 {
            n = evolve(n);
        }
        n
    }).sum();
    println!("Part 1: {total}");
}

fn  part2(nums: Vec<u64>) {
    let mut avail_seqs = HashSet::<[i8; 4]>::new();
    let num_diffs = nums.iter().map(|n| {
        let (ds, ps) = gen_diffs_and_prices(*n);
        for diff_seq in ds.windows(4) {
            avail_seqs.insert(diff_seq.try_into().unwrap());
        }
        (ds, ps)
    }).collect::<Vec<_>>();
    let mut max_val = 0;
    for seq in avail_seqs.into_iter().filter(|s| s[3] >= 0) {
        let sell_value: u64 = num_diffs.iter().map(|(diffs, nums) |{
            sell_value(&seq, &diffs, &nums)
        })
        .sum();
        if sell_value > max_val {
            max_val = sell_value;
            println!("{max_val}");
        }
        if max_val == 1998 {
            println!("{:?}", seq);
        }
    }

    println!("Part 2: {max_val}");

}

const DIFFS_NUM: usize = 2000;

fn main() {
    let seeds = parse_file("input.txt");
    part1(seeds.clone());
    part2(seeds);
}
