use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Shape {
    pub parts: HashSet<(u64, u64)>,
}

impl Shape {
    fn horizontal(height: u64) -> Self {
        Self {
            parts: HashSet::from([
                (2, height + 3),
                (3, height + 3),
                (4, height + 3),
                (5, height + 3)
            ]),
        }
    }

    fn plus(height: u64) -> Self {
        Self {
            parts: HashSet::from([
                (2, height + 4),
                (3, height + 3),
                (3, height + 4),
                (3, height + 5),
                (4, height + 4),
            ])
        }
    }

    fn ell(height: u64) -> Self {
        Self {
            parts: HashSet::from([
                (2, height + 3),
                (3, height + 3),
                (4, height + 3),
                (4, height + 4),
                (4, height + 5),
            ])
        }
    }

    fn vertical(height: u64) -> Self {
        Self {
            parts: HashSet::from([
                (2, height + 6),
                (2, height + 5),
                (2, height + 4),
                (2, height + 3)
            ]),
        }
    }

    fn square(height: u64) -> Self {
        Self {
            parts: HashSet::from([
                (2, height + 3),
                (3, height + 3),
                (2, height + 4),
                (3, height + 4),
            ])
        }
    }
}

#[derive(Debug, Default)]
struct RockFall {
    pub height: u64,
    pub num: u64,
    pub rocks: HashSet<(u64, u64)>
}

impl RockFall {


    /// Shape gets pushed by jets respecting walls
    fn push(&self, shape: &mut Shape, jet: char) {
        match jet {
            '<' => {
                let moved: HashSet<(u64, u64)> = shape.parts
                    .iter()
                    .filter_map(|(x, y)|  x.checked_sub(1).map(|x| (x, *y)))
                    .collect();
                if moved.len() != shape.parts.len() || !self.rocks.is_disjoint(&moved) {
                    return;
                } else {
                    shape.parts = moved;
                }
            }
            '>' => {
                let moved: HashSet<(u64, u64)> = shape.parts
                    .iter()
                    .filter_map(|(x, y)| {
                        if *x == 6 {
                            None
                        } else {
                            Some((*x + 1, *y))
                        }
                    })
                    .collect();
                if moved.len() != shape.parts.len() || !self.rocks.is_disjoint(&moved) {
                    return;
                } else {
                    shape.parts = moved;
                }
            }
            _ => {}
        }
    }

    /// Rock falls one unit. Returns bool indicating if it came
    /// to rest.
    fn fall(&mut self, shape: &mut Shape) -> bool {
        let moved: HashSet<(u64, u64)> = shape.parts
            .iter()
            .filter_map(|(x, y)| y.checked_sub(1).map(|y| (*x, y)))
            .collect();
        // shape was on the floor or ran into resting rocks
        if moved.len() != shape.parts.len() || !moved.is_disjoint(&self.rocks) {
            // insert new rocks and update the height
            for rock in &shape.parts {
                self.rocks.insert(*rock);
                self.height = std::cmp::max(self.height, rock.1 + 1);
            }
            self.num += 1;
            true
        } else {
            // shape moves down one unit
            shape.parts = moved;
            false
        }
    }

}

fn parse_input(filename: &str) -> Vec<char> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    _ = reader.read_line(&mut line).unwrap();
    line.chars().collect()
}

fn get_height(filename: &str, num: u64) -> u64 {
    let jet_stream = parse_input(filename);
    let stream_len = jet_stream.len();
    let mut jet_ix = 0usize;
    let mut shape_ix = 0usize;
    let mut rockfall = RockFall::default();
    let shapes = [Shape::horizontal, Shape::plus, Shape::ell, Shape::vertical, Shape::square];
    loop {
        // stop if the right number of rocks have come to rest
        if rockfall.num == num {
            break;
        }
        // get the next shape and advance the index
        let mut next_rock = shapes[shape_ix](rockfall.height);
        shape_ix = (shape_ix + 1).rem_euclid(5);
        loop {
            // get the next the next jet instruction and advance the index
            let next_jet = jet_stream[jet_ix];
            jet_ix = (jet_ix + 1).rem_euclid(stream_len);
            // move the piece
            rockfall.push(&mut next_rock, next_jet);
            // if the rocks came to rest, move on to next rock
            if rockfall.fall(&mut next_rock) {
                break;
            }
        }
    }
    rockfall.height
}

fn part_one(filename: &str) {
    println!("Part one: {}", get_height(filename, 2022));
}

fn part_two(filename: &str) {
    let start = get_height(filename, 290);
    let repeating_height = get_height(filename, 1995) - start;
    let remainder = get_height(filename, 1585) - start;
    println!("Part two: {}", start + 586510263 * repeating_height + remainder);
}
fn main() {
    part_one("input.txt");
    part_two("input.txt");
}
