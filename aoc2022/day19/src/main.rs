use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(filename: &str) -> Vec<Blueprint> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut blueprints = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let [head, rest]: [&str; 2] = line.trim().split(':')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let [ore, clay, obsidian, geode]: [Vec<u8>; 4] = rest[..rest.len() - 1].split('.')
            .map(|piece|
                piece.split(' ')
                    .filter_map(|word| u8::from_str_radix(word, 10).ok())
                    .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        blueprints.push(Blueprint{
            id: u8::from_str_radix(head.split(' ').last().unwrap(), 10).unwrap(),
            ore: ore[0],
            clay: clay[0],
            obsidian: obsidian.try_into().unwrap(),
            geode: geode.try_into().unwrap(),
        });
        line.clear();
    }
    blueprints
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Blueprint {
    pub id: u8,
    pub ore: u8,
    pub clay: u8,
    pub obsidian: [u8; 2],
    pub geode: [u8; 2],
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
    blueprint: Blueprint,
    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: u8,
    geode_robot: u8,
    minute: u8,
}

impl From<Blueprint> for State {
    fn from(blueprint: Blueprint) -> Self {
        State {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            blueprint,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
            minute: 0,
        }
    }
}

impl State {
    fn branch(&self) -> Vec<State> {
        let purchases = self.purchases();
        let mut new_state: State = (*self).clone();
        new_state.ore += self.ore_robot;
        new_state.clay += self.clay_robot;
        new_state.obsidian += self.obsidian_robot;
        new_state.geode += self.geode_robot;
        new_state.minute += 1;

        purchases.into_iter()
            .map(|robots| {
                let mut state = new_state.clone();
                state.buy(robots);
                state
            })
            .collect()
    }

    fn purchases(&self) -> HashSet<[u8; 4]> {
        let mut visited = HashSet::new();
        visited.insert([0u8, 0, 0, 0]);
        let mut purchase = [0u8, 0, 0, 0];
        for ix in 0..4 {
            purchase[ix] += 1;
            if self.can_afford(&purchase) {
                visited.insert(purchase.clone());
            }
            purchase[ix] -= 1;
        }
        visited
    }

    fn can_afford(&self, robots: &[u8; 4]) -> bool {
        if  robots[0] * self.blueprint.ore
            + robots[1] * self.blueprint.clay
            + robots[2] * self.blueprint.obsidian[0]
            + robots[3] * self.blueprint.geode[0] > self.ore {
            return false;
        }
        if robots[2] * self.blueprint.obsidian[1] > self.clay {
            return false;
        }
        if robots[3] * self.blueprint.geode[1] > self.obsidian {
            return false;
        }
        true
    }

    fn buy(&mut self, robots: [u8; 4]) {
        // update resources
        self.ore -= robots[0] * self.blueprint.ore
            + robots[1] * self.blueprint.clay
            + robots[2] * self.blueprint.obsidian[0]
            + robots[3] * self.blueprint.geode[0];
        self.clay -= robots[2] * self.blueprint.obsidian[1];
        self.obsidian -= robots[3] * self.blueprint.geode[1];
        // update robots
        self.ore_robot += robots[0];
        self.clay_robot += robots[1];
        self.obsidian_robot += robots[2];
        self.geode_robot += robots[3];
    }
}

fn optimize(state: State, time_limit: u8) -> u8 {
    let mut stack = vec![state];
    let mut visited = HashSet::new();
    let mut max_geodes = HashMap::new();
    while let Some(next) = stack.pop() {
        for branch in next.branch() {
            if !visited.contains(&branch) && branch.minute <= time_limit {
                max_geodes.entry(branch.minute)
                    .and_modify(|x| *x = std::cmp::max(branch.geode, *x))
                    .or_insert(branch.geode);
                if branch.minute < time_limit && branch.geode == max_geodes[&branch.minute] {
                    stack.push(branch.clone());
                }
                visited.insert(branch);

            }
        }
    }
    println!("{}", max_geodes[&time_limit]);
    max_geodes[&time_limit]
}

fn part_one(filename: &str) {
    let quality = parse_input(filename)
        .into_iter()
        .map(|bp| bp.id as u64 * optimize(bp.into(), 24) as u64)
        .sum::<u64>();
    println!("Part one: {}", quality);
}

fn part_two(filename: &str) {
    let max = parse_input(filename)
        .into_iter()
        .take(3)
        .map(|bp| optimize(bp.into(), 32) as u64)
        .product::<u64>();
    println!("Part two: {}", max)
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
}
