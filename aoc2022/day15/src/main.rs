use std::cmp::{max, min};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
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
        let pos_y = i64::from_str_radix(qq
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

/// Checks if two intervals intersect
fn intersects((min1, max1): &(i64, i64), (min2, max2): &(i64, i64)) -> bool {
    // left side of second interval contained in first
    if min1 <= min2 && min2 <= max1 {
        return true;
    }
    // right side of second interval contained in first
    if min1 <= max2 && max2 <= max1 {
        return true;
    }
    // right side contains left as subset
    if min2 <= min1 && min1 <= max2 {
        return true;
    }
    false
}

/// Computes the union of two intersecting intervals
fn union((min1, max1): (i64, i64), (min2, max2): (i64, i64)) -> (i64, i64) {
    (min(min1, min2), max(max1, max2))
}

#[derive(Debug, Clone, Default)]
struct DisjointUnion {
    intervals: Vec<(i64, i64)>
}

impl DisjointUnion {

    fn new(intervals: Vec<(i64, i64)>) -> Self {
        let mut du = DisjointUnion::default();
        for interval in intervals {
            du.push(interval);
        }
        du
    }

    fn push(&mut self, interval: (i64, i64)) {
        let union = self.intervals
            .iter()
            .filter(|i| intersects(i, &interval))
            .fold(interval, |acc, int| union(acc, *int));
        self.intervals.retain(|int| !intersects(int,&interval));
        self.intervals.push(union);
    }
}

fn produce_intervals(sensors: &[Sensor], y: i64) -> Vec<(i64, i64)> {
    sensors.iter()
        .filter_map(|s| {
            if (s.pos.1 - y).abs() > s.radius as i64 {
                None
            } else {
                let radius = s.radius as i64;
                let min = (s.pos.1 - y).abs() - radius + s.pos.0;
                let max = radius - (s.pos.1 - y).abs() + s.pos.0;
                Some((min, max))
            }
        })
        .collect()
}

fn part_one(filename: &str) {
    let sensors = parse_input(filename);
    let intervals = produce_intervals(&sensors, 2000000);
    let beacons =  sensors
        .iter()
        .filter(|s| s.beacon.1 == 2000000)
        .dedup_by(|s, t| s.beacon == t.beacon)
        .count() as i64;
    let seen = DisjointUnion::new(intervals)
        .intervals
        .into_iter()
        .map(|(a, b)| 1 + b - a )
        .sum::<i64>() - beacons;

    println!("Part 1: {}", seen);
}

fn part_two(filename: &str) {
    let sensors = parse_input(filename);
    let (intervals, y) = (0..4000001i64).into_par_iter()
        .map(|y| (DisjointUnion::new(produce_intervals(&sensors, y)).intervals, y))
        .find_any(|(ints, _)| !ints.is_empty() && !ints
            .iter()
            .any(|(min, max)| *min <= 0 && 4000000 <= *max)
        ).unwrap();
    let x = intervals.into_iter()
        .find(|(_, max)| 0 <= *max && *max < 4000000)
        .unwrap().1 + 1;
    println!("Part two: {}", x * 4000000 + y);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disjoint_union() {
        let mut du = DisjointUnion {
            intervals: vec![(1, 2), (3, 4), (5, 6), (7, 8)],
        };

        du.push((4, 7));
        assert_eq!(du.intervals, vec![(1, 2), (3, 8)]);
        du.push((9, 10));
        assert_eq!(du.intervals, vec![(1, 2), (3, 8), (9, 10)]);
    }
}