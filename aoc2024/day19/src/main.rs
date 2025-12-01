use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};


fn parse_file(filename: &str) -> [Vec<String>; 2]{
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    reader.read_line(&mut line).unwrap();
    let towels: Vec<_> = line.trim()
        .split(',')
        .map(|t| t.trim().to_string())
        .collect();
    line.clear();
    let mut patterns = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        patterns.push(line.trim().to_string());
        line.clear();
    }
    [towels, patterns]
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State<'a> {
    pos: &'a str,
    dist: u64,
}

impl<'a> State<'a> {
    fn neighbors(self, towels: &'a [String]) -> impl Iterator<Item=Self> + 'a {
        let dist = self.dist;
        towels.iter()
            .filter_map(move |t| self.pos.strip_prefix(t).map(|pat|{
                Self {
                    pos: pat,
                    dist: dist + 1,
                }
            }))
    }
}

#[derive(Debug)]
struct PatternDists<'a> {
    pattern: &'a str,
    map: HashMap<&'a str, u64>,
}

impl<'a> PatternDists<'a> {

    fn new(pattern: &'a str) -> Self {
        Self {
            pattern,
            map: HashMap::from([("", 0)])
        }
    }

    fn insert(&mut self, k: &'a str, v: u64) -> Option<u64> {
        self.map.insert(self.pattern.strip_suffix(k).unwrap(), v)
    }

    fn get(&'a self, key: &'a str) -> Option<&'a u64> {
        self.map.get(self.pattern.strip_suffix(key).unwrap())
    }
}

fn search(pattern: &str, towels: &[String]) -> Option<u64> {
    let mut stack = VecDeque::new();
    let mut dists = PatternDists::new(pattern);
    stack.push_back(State{
        pos: pattern,
        dist: 0,
    });

    while let Some(next) = stack.pop_front() {
        if next.pos.is_empty() {
            return Some(next.dist);
        }

        if next.dist > *dists.get(next.pos).unwrap_or(&u64::MAX) {
            continue
        }

        for n in next.neighbors(towels) {
            let dist = *dists.get(&n.pos).unwrap_or(&u64::MAX);
            if n.dist < dist {
                dists.insert(n.pos, n.dist);
                stack.push_back(n);
            }
        }
    }
    None
}

fn rec_dfs(pattern: &str, towels: &[String], cache: &mut HashMap<String, u64>) -> u64 {
    let mut total = 0;
    for next in towels.iter().filter_map(|t| pattern.strip_prefix(t)) {
        if next == "" {
            total += 1;
        }
        let cached = cache.get(next).cloned();
        if let Some(val) = cached {
            total += val;
        } else {
            let val = rec_dfs(next, towels, cache);
            cache.insert(next.to_string(), val);
            total += val;
        }
    }
    total
}

fn part_1(filename: &str) {
    let [towels, patterns] = parse_file(filename);
    let total = patterns.iter().filter_map(|p| search(p, &towels)).count();
    println!("Part 1: {total}");
}

fn part_2(filename: &str) {
    let [towels, patterns] = parse_file(filename);
    let patterns: Vec<_> = patterns
        .into_iter()
        .filter(|p|
            search(p, &towels).is_some()
        )
        .collect();
    let mut cache = HashMap::new();
    let total: u64 = patterns.iter().map(|p| rec_dfs(p, &towels, &mut cache)).sum();
    println!("Part 2: {total}");
}


fn main() {
    part_1("input.txt");
    part_2("input.txt");
}
