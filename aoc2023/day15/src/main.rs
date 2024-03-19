use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn parse(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut words = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        words = line.split(',').map(|s| s.trim().to_ascii_lowercase()).collect();
        line.clear();
    }

    words
}

fn hash(word: &str) -> u64 {
    let mut total = 0;
    for char in word.chars() {
        total += char as u64;
        total *= 17;
        total = total.rem_euclid(256);
    }
    total
}

#[derive(Debug, Default)]
struct HashishMap<'a> {
    map: HashMap<u64, Vec<(&'a str, u64)>>
}


impl<'a> HashishMap<'a> {
    fn update(&mut self, cmd: &'a str) {
        if cmd.contains('-') {
            let mut cmd = cmd.split('-');
            let label = cmd.next().unwrap();
            let box_no = hash(label);
            if let Some(bin) = self.map.get_mut(&box_no) {
                bin.retain(|entry| entry.0 != label);
            }
        } else {
            let mut cmd = cmd.split('=');
            let label = cmd.next().unwrap();
            let value = u64::from_str(cmd.next().unwrap()).unwrap();
            let box_no = hash(label);
            if let Some(bin) = self.map.get_mut(&box_no) {
                let mut updated = false;
                for entry in bin.iter_mut() {
                    if entry.0 == label {
                        entry.1 = value;
                        updated = true;
                        break;
                    }
                }
                if !updated {
                    bin.push((label, value));
                }
            } else {
                self.map.insert(box_no, vec![(label, value)]);
            }
        }
    }

    fn focal_power(&self) -> u64 {
        let mut total = 0;
        for (box_no, lens) in &self.map {
            if lens.is_empty() {
                continue;
            }
            total += (box_no + 1) * lens.iter().enumerate()
                .map(|(slot, (_, value))| (slot as u64 + 1) * (*value))
                .sum::<u64>();
        }
        total
    }
}

fn part_one(filename: &str) {
    let words = parse(filename);
    let res = words.iter().map(|w| hash(w)).sum::<u64>();
    println!("Part one: {}", res);
}

fn part_two(filename: &str) {
    let words = parse(filename);
    let mut map = HashishMap::default();
    for word in &words {
        map.update(&word);
    }
    println!("Part two: {}", map.focal_power());
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

}
