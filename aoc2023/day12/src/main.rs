use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Completion {
    Invalid,
    Partial,
    Finished,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Record {
    springs: Vec<char>,
    chunks: Vec<u8>,
}

impl Record {

    fn unfold(self) -> Self {
        let Self {
            mut springs,
            mut chunks,
        } = self;
        let original = springs.clone();
        let original_chunks = chunks.clone();
        for _ in 0..4 {
            springs.push('?');
            springs.extend(original.clone().into_iter());
            chunks.extend(original_chunks.clone().into_iter());
        }
        Self {
            springs,
            chunks
        }
    }

    fn neighbors(&self, cache: &mut HashSet<Record>) -> (VecDeque<Self>, u64) {
        let mut ngbrs = VecDeque::new();
        let mut finished = 0;
        for (ix, c) in self.springs.iter().enumerate() {
            if *c == '?' {
                let mut springs = self.springs.clone();
                springs[ix] = '.';
                let mut record = Record {
                    springs: springs.clone(),
                    chunks: self.chunks.clone(),
                };
                if !cache.contains(&record) {
                    match record.check_chunks(){
                        Completion::Partial => ngbrs.push_back(record),
                        Completion::Finished => {
                            record.complete();
                            if cache.insert(record) {
                                finished += 1;
                            }
                        }
                        _ => {}
                    }
                }

                springs[ix] = '#';
                let mut record = Record {
                    springs,
                    chunks: self.chunks.clone(),
                };
                if !cache.contains(&record) {
                    match record.check_chunks() {
                        Completion::Partial => ngbrs.push_back(record),
                        Completion::Finished => {
                            record.complete();
                            if cache.insert(record) {
                                finished += 1;
                            }
                        }
                        _ => {}
                    }
                }
                break;
            }
        }
        (ngbrs, finished)
    }

    fn complete(&mut self) {
        for c in self.springs.iter_mut() {
            if *c == '?' {
                *c = '.';
            }
        }
    }

    fn check_chunks(&self) -> Completion {
        let mut chunks = vec![];
        let mut next = 0;

        if self.springs.iter().filter(|c| **c == '#').count()
            > self.chunks.iter().sum::<u8>() as usize
        {
            return Completion::Invalid;
        }
        if self.springs.iter().filter(|c| **c == '#' || **c == '?').count()
            < self.chunks.iter().sum::<u8>() as usize
        {
            return Completion::Invalid;
        }
        for c in self.springs.iter() {
            match c {
                '#' => {
                    next += 1;
                }
                '.' => {
                    if next != 0 {
                        chunks.push(next);
                        next = 0;
                    }
                }
                _ => {
                    if next != 0 {
                        chunks.push(next);
                        next = 0;
                    }
                    break
                }
            }
        }
        if next != 0 {
            chunks.push(next);
        }
        if chunks.is_empty() {
            return Completion::Partial;
        }

        if chunks == self.chunks {
            Completion::Finished
        } else if !chunks.len() <= self.chunks.len()
            || !chunks[..chunks.len() - 1]
                .iter()
                .zip(self.chunks.iter())
                .all(|(a, b)| a == b)
            || chunks[chunks.len() - 1] > self.chunks[chunks.len() - 1]
        {
            Completion::Invalid
        } else {
            Completion::Partial
        }
    }

    fn count_combos(&self, cache: Option<HashSet<Record>>) -> (HashSet<Record>, u64) {
        let mut cache = cache.unwrap_or_default();
        let (mut stack, mut total) = self.neighbors(&mut cache);
        while let Some(next) = stack.pop_back() {
            if cache.contains(&next) {
                continue;
            }
            let (next_ngbrs, next_total) = next.neighbors(&mut cache);
            cache.insert(next);
            stack.extend(next_ngbrs.into_iter());
            total += next_total;
        }
        (cache, total)
    }
}



fn parse_file(filename: &str) -> Vec<Record> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut records = vec![];

    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let mut split = line.trim().split(' ');
        let springs = split.next().unwrap().to_string().chars().collect();
        let chunks = split
            .next()
            .unwrap()
            .split(',')
            .map(|c| u8::from_str(c).unwrap())
            .collect();
        records.push( Record { springs, chunks } );
        line.clear()
    }
    records
}

fn part_one() {
    let records = parse_file("input.txt");
    let ans = records.into_iter().map(|r| r.count_combos(None).1).sum::<u64>();
    println!("Part one: {}", ans);
}

fn part_two() {
    let records = parse_file("input.txt");
    let ans = records.into_par_iter().map(|r| r.unfold().count_combos(None).1).sum::<u64>();
    println!("Part two: {}", ans);
}

fn main() {
    part_one();
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combos() {
        let record = Record {
            springs: "???.###".chars().collect(),
            chunks: vec![1, 1, 3],
        };
        let combos = record.count_combos(None).1;
        assert_eq!(combos, 1);

        let record = Record {
            springs: "?###????????".chars().collect(),
            chunks: vec![3, 2, 1],
        };
        let combos = record.count_combos(None).1;
        assert_eq!(combos, 10);
    }

    #[test]
    fn test_unfold() {
        let record = Record {
            springs: "?.#".chars().collect(),
            chunks: vec![1, 1],
        };
        let record = record.unfold();
        let expected = Record {
            springs: "?.#??.#??.#??.#??.#".chars().collect(),
             chunks: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        };
        assert_eq!(record, expected);
        let combos = record.count_combos(None).1;
        assert_eq!(combos, 1);

        let record = Record {
            springs: "?###????????".chars().collect(),
            chunks: vec![3, 2, 1],
        };
        let combos = record.unfold().count_combos();
        assert_eq!(combos, 506250);
    }
}