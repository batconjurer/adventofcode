use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use either::Either;

fn parse_file(filename: &str) -> (Vec<Lock>, Vec<Key>)  {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut row = 0;

    let mut locks = vec![];
    let mut keys : Vec<Key> = vec![];
    let mut curr = None;

    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        if line.trim().is_empty() {
            match curr {
                Some(Either::Left(lock)) => locks.push(lock),
                Some(Either::Right(key)) => keys.push(key),
                None => unreachable!(),
            }
            curr = None;
            line.clear();
            continue;
        }
        if curr == None {
            if line.trim().chars().all(|c| c == '#') {
                curr = Some(Either::Left(Lock::default()));
            } else if line.trim().chars().all(|c| c == '.') {
                curr = Some(Either::Right(Key::default()));
            } else {
                unreachable!();
            }
        } else {

            for (ix, c) in line.trim().chars().enumerate() {
                match &mut curr {
                    Some(Either::Left(lock)) => {
                        if c == '#' {
                            lock.0[ix] += 1;
                        }
                    }
                    Some(Either::Right(key)) => {
                        if c == '#' {
                            key.0[ix] += 1;
                        }
                    }
                    None => unreachable!()
                }
            }
        }

        row += 1;
        line.clear();
    }
    for k in keys.iter_mut() {
        for ix in 0..5 {
            k.0[ix] -= 1;
        }
    }
    (locks, keys)
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
struct Lock([u8; 5]);

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
struct Key([u8; 5]);

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct ExpandedKey([String; 7]);

#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct ExpandedLock([String; 7]);
fn overlaps(lock: &Lock, key: &Key) -> bool {
    for ix in 0..5 {
        if lock.0[ix] + key.0[ix] > 5 {
            return true
        }
    }
    false
}

fn part1(filename: &str) {
    let (locks, keys) = parse_file(filename);

    let locks: HashSet<Lock> = HashSet::from_iter(locks.into_iter());
    let keys: HashSet<Key> = HashSet::from_iter(keys.into_iter());

    let mut pairs = HashSet::new();
    for lock in &locks {
        for key in &keys {
            if !overlaps(lock, key) {
                pairs.insert((lock, key));
            }
        }
    }
    println!("Part 1: {}", pairs.len());
}

fn main() {
    part1("input.txt");
}
