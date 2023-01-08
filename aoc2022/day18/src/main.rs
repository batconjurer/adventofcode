use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

/// Given a point (x, y, z), the cube has vertices
/// (x, y, z), (x+1, y, z),
/// (x, y+1, Z), (x, y, z+1),
/// (x+1, y+1, z), (x+1, y, z+1),
/// (x, y+1, z+1), (x+1, y+1, z+1)
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cube([u8; 3]);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Axis {
    X,
    Y,
    Z,
}

/// Every side is given a canonical point to
/// be associated with. It is the point that
/// generates the cube above that also contains
/// the side. We also give the axis normal to the
/// side to make a unique identification.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Side {
    anchor: [u8; 3],
    axis: Axis,
}

impl Cube {
    fn new(x: u8, y: u8, z: u8) -> Self {
        Self([x, y, z])
    }

    fn sides(&self) -> [Side; 6] {
        [
            Side {
                anchor: self.0,
                axis: Axis::X,
            },
            Side {
                anchor: self.0,
                axis: Axis::Y,
            },
            Side {
                anchor: self.0,
                axis: Axis::Z,
            },
            Side {
                anchor: [self.0[0] + 1, self.0[1], self.0[2]],
                axis: Axis::X,
            },
            Side {
                anchor: [self.0[0], self.0[1] + 1, self.0[2]],
                axis: Axis::Y,
            },
            Side {
                anchor: [self.0[0], self.0[1], self.0[2] + 1],
                axis: Axis::Z,
            },
        ]
    }
}

fn parse_input(filename: &str) -> Vec<Cube> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut cubes = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let coordinate: Vec<_> = line
            .split(",")
            .filter_map(|coord| u8::from_str_radix(coord.trim_end(), 10).ok())
            .collect();
        cubes.push(Cube([coordinate[0], coordinate[1], coordinate[2]]));
        line.clear();
    }
    cubes
}

struct BoundingCube {
    min_x: u8,
    min_y: u8,
    min_z: u8,
    max_x: u8,
    max_y: u8,
    max_z: u8,
}

impl BoundingCube {
    fn contains(&self, Cube([x, y, z]): &Cube) -> bool {
        self.min_x <= *x
            && *x <= self.max_x
            && self.min_y <= *y
            && *y <= self.max_y
            && self.min_z <= *z
            && *z <= self.max_z
    }

    fn x_bounds(&self) -> RangeInclusive<u8> {
        self.min_x..=self.max_x
    }

    fn y_bounds(&self) -> RangeInclusive<u8> {
        self.min_y..=self.max_y
    }

    fn z_bounds(&self) -> RangeInclusive<u8> {
        self.min_z..=self.max_z
    }
}

fn compute_bounding_cube(cubes: &[Cube]) -> BoundingCube {
    let mut min_x = u8::MAX;
    let mut min_y = u8::MAX;
    let mut min_z = u8::MAX;
    let mut max_x = 0u8;
    let mut max_y = 0u8;
    let mut max_z = 0u8;
    for cube in cubes {
        let [x, y, z] = cube.0;
        min_x = std::cmp::min(min_x, x);
        min_y = std::cmp::min(min_y, y);
        min_z = std::cmp::min(min_z, z);
        max_x = std::cmp::max(max_x, x);
        max_y = std::cmp::max(max_y, y);
        max_z = std::cmp::max(max_z, z);
    }
    BoundingCube {
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
    }
}

type AirPocket = HashSet<Cube>;

fn air_pocket(cube: Cube, bounding_cube: &BoundingCube, cubes: &HashSet<Cube>) -> Option<AirPocket> {
    if cubes.contains(&cube) {
        return None;
    }

    let mut stack = vec![cube.clone()];
    let mut air_pocket = HashSet::from([cube]);
    while let Some(Cube([x, y, z])) = stack.pop() {
        if x as u64 * y as u64 * z as u64 == 0 {
            return None;
        }
        for neighbor in &[
            Cube::new(x - 1, y, z),
            Cube::new(x + 1, y, z),
            Cube::new(x, y - 1, z),
            Cube::new(x, y + 1, z),
            Cube::new(x, y, z - 1),
            Cube::new(x, y, z + 1),
        ] {
            if !bounding_cube.contains(neighbor) {
                return None;
            }
            if !cubes.contains(&neighbor) && !air_pocket.contains(&neighbor) {
                air_pocket.insert(neighbor.clone());
                stack.push(neighbor.clone());
            }
        }
    }
    Some(air_pocket)
}

fn part_one(filename: &str) {
    let cubes = parse_input(filename);
    let bounding_cube = compute_bounding_cube(&cubes);
    let mut out_sides = HashMap::new();
    for cube in &cubes {
        for side in cube.sides() {
            out_sides
                .entry(side)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }
    out_sides.retain(|_, v| *v == 1);
    println!("Surface area: {}", out_sides.len());

    let cubes: HashSet<Cube> = cubes.into_iter().collect();
    let mut interior_cubes = HashSet::new();
    for x in bounding_cube.x_bounds() {
        for y in bounding_cube.y_bounds() {
            for z in bounding_cube.z_bounds() {
                let cube_to_check = Cube::new(x, y, z);
                if interior_cubes.contains(&cube_to_check) {
                    continue;
                }
                if let Some(pocket) = air_pocket(cube_to_check, &bounding_cube, &cubes) {
                    for cube in pocket.into_iter() {
                        interior_cubes.insert(cube);
                    }
                }
            }
        }
    }
    for cube in interior_cubes {
        for side in cube.sides() {
            out_sides.remove(&side);
        }
    }
    println!("External surface area: {}", out_sides.len());

}

fn main() {
    part_one("input.txt");
}
