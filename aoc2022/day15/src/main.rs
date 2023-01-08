use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;
use rayon::iter;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
struct Sensor {
    pub pos: (i64, i64),
    pub beacon: (i64, i64),
    pub radius: u64,
}

impl Sensor {
    fn new(pos: (i64, i64), beacon: (i64, i64)) -> Self {
        Self {
            pos,
            beacon,
            radius: dist(pos, beacon),
        }
    }

    fn not_beacon(&self, pos: (i64, i64)) -> bool {
        if pos == self.beacon {
            false
        } else {
            dist(self.pos, pos) <= self.radius
        }
    }

    fn maybe_beacon(&self, pos: (i64, i64)) -> bool {
        !(pos == self.beacon || dist(self.pos, pos) <= self.radius)
    }
}

fn parse_input(filename: &str) -> Vec<Sensor> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut sensors = vec![];
    while let Ok(length) = reader.read_line(&mut line){
        if length == 0 {
            break;
        }
        let pieces: Vec<_> = line.split_ascii_whitespace().collect();
        let pos_x = i64::from_str_radix(
            &pieces[2][2..].trim_end_matches(","),
            10)
            .unwrap();
        let pos_y = i64::from_str_radix(
            &pieces[3][2..].trim_end_matches(":"),
            10)
            .unwrap();
        let beacon_x = i64::from_str_radix(
            &pieces[8][2..].trim_end_matches(","),
            10)
            .unwrap();
        let beacon_y = i64::from_str_radix(
            &pieces[9][2..],
            10)
            .unwrap();
        sensors.push(Sensor::new(
            (pos_x, pos_y),
            (beacon_x, beacon_y),
        ));
        line.clear();
    }
    sensors
}

fn dist(pos1: (i64, i64), pos2: (i64, i64)) -> u64 {
    ((pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()) as u64
}

fn part_one(filename: &str) {
    let sensors = parse_input(filename);
    let mut positions = HashSet::new();
    for sensor in sensors {
        let upper = sensor.pos.0 + (sensor.pos.1 - 2000000).abs() - (sensor.radius as i64);
        let lower = (sensor.radius as i64)  - (sensor.pos.1 - 2000000).abs() + sensor.pos.0;
        let x = dist((upper, 2000000), sensor.pos);
        let y = dist((lower, 2000000), sensor.pos);
        for position in lower..=upper {
            positions.insert(position);
        }
        positions.remove(&sensor.beacon.0);
    }
    println!("Part 1: {}", positions.len());
}

fn part_two(filename: &str) {
    let mut sensors = parse_input(filename);
    sensors.sort_by_key(|sensor| -(sensor.radius as i64));
    let mut x = 0i64;
    let mut  y = 0i64;
    //const MAX: i64 = 20;
    const MAX: i64 = 4000000i64;
    while (x <= MAX && y <= MAX) {
        match sensors
            .iter()
            .find(|sensor| !sensor.maybe_beacon((x, y))) {
            Some(sensor) => {
                y = sensor.radius as i64 - (sensor.pos.0 - x).abs() + sensor.pos.1 + 1;
                if y > MAX {
                    y = 0;
                    x += 1;
                }
            }
            None => {
                println!("Part 2: {}", x * 4000000 + y);
                break;
            }
        }
    }
}

fn main() {
    part_one("input.txt");
    //part_two("input.txt");
}
