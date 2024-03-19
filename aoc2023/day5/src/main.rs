use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;


#[derive(Debug, Clone)]
struct MapRange {
    dest: u64,
    source: u64,
    range: u64,
}

impl MapRange {
    fn map(&self, num: u64) -> Option<u64> {
        if self.source <= num && num < self.source + self.range {
            Some((num - self.source) + self.dest)
        }  else {
            None
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Map {
    ranges: Vec<MapRange>
}

impl Map {
    fn clear(&mut self) {
        self.ranges.clear()
    }

    fn map(&self, num: u64) -> u64 {
        for range in &self.ranges {
            if let Some(val) = range.map(num) {
                return val;
            }
        }
        num
    }
}

#[derive(Clone)]
struct Maps {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Maps {
    fn map(&self, seed: u64) -> u64 {
        self.humidity_to_location.map(
            self.temperature_to_humidity.map(
                self.light_to_temperature.map(
                    self.water_to_light.map(
                        self.fertilizer_to_water.map(
                            self.soil_to_fertilizer.map(
                                self.seed_to_soil.map(seed)

                            )
                        )
                    )
                )
            )
        )
    }

    fn test_points(self) -> HashSet<u64> {
        let piecewise = PiecewiseLinear::from(self.humidity_to_location)
            .compose(self.temperature_to_humidity.into())
            .compose(self.light_to_temperature.into())
            .compose(self.water_to_light.into())
            .compose(self.fertilizer_to_water.into())
            .compose(self.soil_to_fertilizer.into())
            .compose(self.seed_to_soil.into());
        let mut test_points = HashSet::new();
        for piece in piecewise.pieces.into_iter() {
            test_points.insert(piece.upper);
            test_points.insert(piece.lower);
        }
        test_points
    }
}

/// This is a function of the form f(x) = x + a, a an integer,
/// with a domain l <= x < u
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct LinearPiece {
    lower: u64,
    upper: u64,
    trans: i64,
}

impl LinearPiece {

    /// self(other)
    fn compose(self, other: Self) -> Option<Self> {
        let (lower, upper)  = if other.trans <= 0  {
            (
                u64::saturating_add(self.lower, other.trans.abs() as u64),
                u64::saturating_add(self.upper, other.trans.abs() as u64),
            )
        } else {
            (
                if self.lower < other.trans as u64 { 0 } else {self.lower - other.trans as u64},
                if self.upper < other.trans as u64 { 0 } else {self.upper - other.trans as u64},
            )
        };

        let (lower, upper) = intersection(lower, upper, other.lower, other.upper)?;
        if upper < lower {
            panic!("Composition bug!")
        }
        if lower == upper {
            None
        } else {
            Some(Self {
                lower,
                upper,
                trans: self.trans + other.trans,
            })
        }

    }
}

#[derive(Debug, PartialEq, Eq)]
struct PiecewiseLinear {
    pieces: Vec<LinearPiece>
}

impl PiecewiseLinear {
    fn compose(self, other: Self) -> Self {
        let mut pieces = vec![];
        for first in self.pieces {
            for second in &other.pieces {
                if let Some(piece) = first.compose(*second) {
                    pieces.push(piece)
                }
            }
        }
        Self{ pieces }
    }

    #[cfg(test)]
    fn sort(&mut self) {
        self.pieces.sort_by_key(|p| p.lower);
    }
}

impl From<MapRange> for LinearPiece {
    fn from(range: MapRange) -> Self {
        Self {
            lower: range.source,
            upper: range.source + range.range,
            trans: range.dest as i64 - range.source as i64,
        }
    }
}

impl From<Map> for PiecewiseLinear {
    fn from(map: Map) -> Self {
        let mut pieces: Vec<_> = map.ranges.into_iter().map(LinearPiece::from).collect();
        let existing: HashSet<_> = pieces.iter().map(|p| (p.lower, p.upper)).collect();
        pieces.sort_by_key(|p| p.lower);
        let max = pieces.last().unwrap().upper;
        let mut last_lower = 0;
        let mut missing_ranges: Vec<_> = pieces.iter()
            .filter_map(|piece|{
                let next = (last_lower, piece.lower);
                last_lower = piece.lower;
                if next.0 != next.1 && !existing.contains(&next){
                    Some(next)
                } else {
                    None
                }
        }).collect();
        missing_ranges.push((max, u64::MAX));
        let missing_ranges: Vec<_> = missing_ranges.into_iter().map(|(l, u)| LinearPiece{
            lower: l,
            upper: u,
            trans: 0,
        }).collect();
        pieces.extend_from_slice(&missing_ranges);
        Self { pieces }
    }
}

fn intersection(l1: u64, u1: u64, l2: u64, u2: u64) -> Option<(u64, u64)> {
    if l1 <= l2 && l2 <= u1 &&  u1 <= u2  {
        Some((l2, u1))
    } else if l2 <= l1 && l1 <= u2 && u2 <= u1 {
        Some((l1, u2))
    } else if l1 <= l2 && u2 <= u1 {
        Some((l2, u2))
    } else if l2 <= l1 && u1 <= u2 {
        Some((l1, u1))
    } else {
        None
    }
}

fn parse_input(filename: &str) -> Maps {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut maps = vec![];
    let mut map = Map::default();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        if line.ends_with("map:\n") {
            maps.push(map.clone());
            map.clear();
            line.clear();
            continue;
        } else {
            let vals: Vec<_> =  line.trim().split(' ')
                .map(|num| u64::from_str(num).unwrap())
                .collect();
            let vals: [u64; 3] = vals.try_into().unwrap();
            map.ranges.push(MapRange{dest: vals[0], source: vals[1], range: vals[2]});
        }
        line.clear()
    }
    maps.remove(0);
    maps.push(map);
    Maps {
        seed_to_soil: maps[0].clone(),
        soil_to_fertilizer: maps[1].clone(),
        fertilizer_to_water: maps[2].clone(),
        water_to_light: maps[3].clone(),
        light_to_temperature: maps[4].clone(),
        temperature_to_humidity: maps[5].clone(),
        humidity_to_location: maps[6].clone(),
    }
}

fn part_one(test: bool) {
    let (maps, seeds) = if test {
        (parse_input("test.txt"), vec![79u64, 14, 55, 13])
    } else {
        (parse_input("input.txt"), vec![1972667147, 405592018, 1450194064, 27782252, 348350443, 61862174,
            3911195009, 181169206, 626861593, 138786487, 2886966111, 275299008, 825403564, 478003391,
            514585599, 6102091, 2526020300, 15491453, 3211013652, 546191739])
    };
    let answer = seeds.into_iter()
        .map(|seed| maps.map(seed))
        .min()
        .unwrap();
    println!("Part one: {}", answer);
}

fn part_two(test: bool) {
    let (maps, seeds) = if test {
        (parse_input("test.txt"), vec![(79u64, 14u64), (55, 13)])
    } else {
        (parse_input("input.txt"), vec![(1972667147, 405592018), (1450194064, 27782252), (348350443, 61862174),
            (3911195009, 181169206), (626861593, 138786487), (2886966111, 275299008), (825403564, 478003391),
            (514585599, 6102091), (2526020300, 15491453), (3211013652, 546191739)])
    };
    let mut test_seeds: HashSet<_> =  maps.clone()
        .test_points()
        .into_iter()
        .filter(|point| seeds.iter().any(|(l, u)| l <= point && *point < *l + *u))
        .collect();
    for (range_start, range_len) in &seeds {
        test_seeds.insert(*range_start);
        test_seeds.insert(*range_start + *range_len - 1);
    }
    let ans = test_seeds.into_iter().map(|seed|  maps.map(seed)).min().unwrap();
    println!("Part two: {}", ans);
}

fn main() {
    part_one(false);
    part_two(false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let seed_to_soil = Map {
            ranges: vec![
                MapRange{
                    dest: 50,
                    source: 98,
                    range: 2,
                },
                MapRange {
                    dest: 52,
                    source: 50,
                    range: 48,
                }
            ]
        };
        assert_eq!(seed_to_soil.map(79), 81);
        assert_eq!(seed_to_soil.map(14), 14);
        assert_eq!(seed_to_soil.map(55), 57);
        assert_eq!(seed_to_soil.map(13), 13);
    }

    #[test]
    fn test_into_piece() {
        let seed_to_soil = Map {
            ranges: vec![
                MapRange{
                    dest: 50,
                    source: 98,
                    range: 2,
                },
                MapRange {
                    dest: 52,
                    source: 50,
                    range: 48,
                }
            ]
        };
        let mut seed_to_soil = PiecewiseLinear::from(seed_to_soil);
        seed_to_soil.sort();
        assert_eq!(seed_to_soil, PiecewiseLinear{pieces: vec![
            LinearPiece{
                lower: 0,
                upper: 50,
                trans: 0,
            },
            LinearPiece{
                lower: 50,
                upper: 98,
                trans: 2,
            },
            LinearPiece{
                lower: 98,
                upper: 100,
                trans: -48,
            },
            LinearPiece{
                lower: 100,
                upper: u64::MAX,
                trans: 0,
            },
        ]});
        let soil_to_fertilizer = Map {
            ranges: vec![
                MapRange{
                    dest: 0,
                    source: 15,
                    range: 37,
                },
                MapRange {
                    dest: 37,
                    source: 52,
                    range: 2,
                },
                MapRange {
                    dest: 39,
                    source: 0,
                    range: 15,
                },
            ]
        };
        let mut soil_to_fertilizer = PiecewiseLinear::from(soil_to_fertilizer);
        soil_to_fertilizer.sort();
        assert_eq!(soil_to_fertilizer, PiecewiseLinear{pieces: vec![
            LinearPiece{
                lower: 0,
                upper: 15,
                trans: 39,
            },
            LinearPiece{
                lower: 15,
                upper: 52,
                trans: -15,
            },
            LinearPiece{
                lower: 52,
                upper: 54,
                trans: -15,
            },
            LinearPiece{
                lower: 54,
                upper: u64::MAX,
                trans: 0,
            },
        ]});
    }

    #[test]
    fn test_piecewise_compose() {
        let soil_to_fertilizer = PiecewiseLinear{pieces: vec![
            LinearPiece{
                lower: 0,
                upper: 15,
                trans: 39,
            },
            LinearPiece{
                lower: 15,
                upper: 52,
                trans: -15,
            },
            LinearPiece{
                lower: 52,
                upper: 54,
                trans: -15,
            },
            LinearPiece{
                lower: 54,
                upper: u64::MAX,
                trans: 0,
            },
        ]};
        let seed_to_soil = PiecewiseLinear{pieces: vec![
            LinearPiece{
                lower: 0,
                upper: 50,
                trans: 0,
            },
            LinearPiece{
                lower: 50,
                upper: 98,
                trans: 2,
            },
            LinearPiece{
                lower: 98,
                upper: 100,
                trans: -48,
            },
            LinearPiece{
                lower: 100,
                upper: u64::MAX,
                trans: 0,
            },
        ]};
        let mut seed_to_fertilizer = soil_to_fertilizer.compose(seed_to_soil);
        seed_to_fertilizer.sort();
        assert_eq!(seed_to_fertilizer, PiecewiseLinear{pieces: vec![
            LinearPiece{
                lower: 0,
                upper: 15,
                trans: 39,
            },
            LinearPiece{
                lower: 15,
                upper: 50,
                trans: -15,
            },
            LinearPiece{
                lower: 50,
                upper: 52,
                trans: -13,
            },
            LinearPiece{
                lower: 52,
                upper: 98,
                trans: 2,
            },
            LinearPiece{
                lower: 98,
                upper: 100,
                trans: -63,
            },
            LinearPiece{
                lower: 100,
                upper: u64::MAX,
                trans: 0,
            },
        ]});
    }
}