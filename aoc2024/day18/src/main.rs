use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(filename: &str) -> HashMap<(u64, u64), u64>  {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut row = 0;
    let mut corruptions = HashMap::new();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }

        let mut pair = line.trim().split(',');

        corruptions.insert(
            (
                u64::from_str_radix(pair.next().unwrap(), 10).unwrap(),
                u64::from_str_radix(pair.next().unwrap(), 10).unwrap()
            ),
            row
        );
        row += 1;
        line.clear();
    }
    corruptions
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    pos: (u64, u64),
    dist: u64,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl State {
    fn neighbors(
        &self,
        exit: (u64, u64),
        corruptions: &HashSet<(u64, u64)>,
    ) -> impl Iterator<Item=Self> {
        let mut ns = [const { None }; 4];
        if self.pos.0 > 0 {
            let n = (self.pos.0 - 1, self.pos.1);
            if !corruptions.contains(&n) {
                ns[0] = Some(Self {
                    pos: n,
                    dist: self.dist + 1,
                });
            }
        }
        if self.pos.1 > 0 {
            let n = (self.pos.0, self.pos.1 - 1);
            if !corruptions.contains(&n) {
                ns[1] = Some(Self {
                    pos: n,
                    dist: self.dist + 1,
                });
            }
        }
        if self.pos.0 < exit.0 {
            let n = (self.pos.0 + 1, self.pos.1);
            if !corruptions.contains(&n) {
                ns[2] = Some(Self {
                    pos: n,
                    dist: self.dist + 1,
                });
            }
        }

        if self.pos.1 < exit.1 {
            let n = (self.pos.0, self.pos.1 + 1);
            if !corruptions.contains(&n) {
                ns[3] = Some(Self {
                    pos: n,
                    dist: self.dist + 1,
                });
            }
        }
        ns.into_iter().filter_map(|x| x)
    }
}

fn search(exit: (u64, u64), corruptions: &HashSet<(u64, u64)>) -> u64 {
    let mut stack = BinaryHeap::new();
    let mut dists = HashMap::new();
    dists.insert((0u64, 0u64), 0);
    stack.push(State{
        pos: (0, 0),
        dist: 0,
    });

    while let Some(next) = stack.pop() {
        if next.pos == exit {
            return next.dist;
        }

        if next.dist > *dists.get(&next.pos).unwrap_or(&u64::MAX) {
            continue
        }

        for n in next.neighbors(exit, corruptions) {
            let dist = *dists.get(&n.pos).unwrap_or(&u64::MAX);
            if n.dist < dist {
                dists.insert(n.pos, n.dist);
                stack.push(n);
            }
        }
    }
    u64::MAX
}

fn part_1(filename: &str) {
    let corruptions = parse_file(filename)
        .into_iter()
        .filter_map(|(k,v)| if v < 1024 {
            Some(k)
        } else {
            None
        })
        .collect();
    let dist = search((70, 70), &corruptions);
    println!("Part 1: {dist}");
}

fn part_2(filename: &str) {
    let corruptions = parse_file(filename);
    for start in 1025..3450 {
        let corr = corruptions
            .iter()
            .filter_map(|(k,v)| if *v < start {
                Some(*k)
            } else {
                None
            })
            .collect();
        let dist = search((70, 70), &corr);
        if dist == u64::MAX {
            let (coord, _) = corruptions.iter().find(|(_, v)| **v == start-1).unwrap();
            println!("Part 2 {:?}", coord);
            return;
        }
    }

}

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}
