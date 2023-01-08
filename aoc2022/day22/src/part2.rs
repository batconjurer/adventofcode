use super::*;
use super::parse_cube::*;

#[derive(Debug, Clone)]
pub enum Color {
    Red,
    White,
    Green,
    Orange,
    Yellow,
    Blue,
}

impl Color {
    fn order(&self) -> usize {
        match self {
            Self::Red=> 0,
            Self::White => 1,
            Self::Green => 2,
            Self::Orange => 3,
            Self::Yellow => 4,
            Self::Blue => 5,
        }
    }
}

#[derive(Debug)]
pub struct Face {
    pub color: Color,
    pub face: HashMap<(u64, u64), bool>,
}

impl Face {
    fn is_open(&self, pos: (u64, u64)) -> bool {
        *self.face.get(&pos).unwrap_or(&false)
    }
}

#[derive(Debug, Clone)]
struct Position {
    face: Color,
    pos: (u64, u64),
    heading: Heading,
}

impl Position {
    fn password(&self, config: &Configuration) -> u64 {
        let face = &config.faces[self.face.order()];
        let heading_summand = match self.heading {
            Heading::North => 3u64,
            Heading::South => 1,
            Heading::East => 0,
            Heading::West => 2,
        };
        1000 * (self.pos.0 + face.row_min + 1) + 4 * (self.pos.1 + face.col_min + 1) + heading_summand
    }

    fn perform(&mut self, inst: Instruction, cube: &Cube) {
        match inst {
            Instruction::TurnRight => {
                self.heading = match self.heading {
                    Heading::North => Heading::East,
                    Heading::East => Heading::South,
                    Heading::South => Heading::West,
                    Heading::West => Heading::North,
                };
            }
            Instruction::TurnLeft => {
                self.heading = match self.heading {
                    Heading::North => Heading::West,
                    Heading::West => Heading::South,
                    Heading::South => Heading::East,
                    Heading::East => Heading::North,
                }
            }
            Instruction::Move(amt) => self.walk(amt, cube)
        }
    }

    fn walk(&mut self, amount: u64, cube: &Cube) {
        for _ in 0..amount {
            let candidate_pos = match &self.heading  {
                Heading::North => if self.pos.0 == 0 {
                    cube.wrap(&self)
                } else {
                    Position {pos: (self.pos.0 - 1, self.pos.1), ..self.clone()}
                }
                Heading::South => if self.pos.0 == cube.dim {
                    cube.wrap(&self)
                } else {
                    Position {pos: (self.pos.0 + 1, self.pos.1), ..self.clone()}
                }
                Heading::West => if self.pos.1 == 0 {
                    cube.wrap(&self)
                } else {
                    Position{pos: (self.pos.0, self.pos.1 - 1), ..self.clone()}
                }
                Heading::East => if self.pos.1 == cube.dim {
                    cube.wrap(&self)
                } else {
                    Position{pos: (self.pos.0, self.pos.1 + 1), ..self.clone()}
                }
            };
            if cube.is_open(&candidate_pos) {
                *self = candidate_pos;
            } else {
                return;
            }
        }
    }
}

#[derive(Debug)]
pub struct Cube {
    pub faces: [Face; 6],
    pub dim: u64,
    pub test: bool,
}

impl Default for Cube {
    fn default() -> Self {
        Cube {
            faces: [
                Face {
                    color: Color::Red,
                    face: HashMap::default(),
                },
                Face {
                    color: Color::White,
                    face: HashMap::default(),
                },
                Face {
                    color: Color::Green,
                    face: HashMap::default(),
                },
                Face {
                    color: Color::Orange,
                    face: HashMap::default(),
                },
                Face {
                    color: Color::Yellow,
                    face: HashMap::default(),
                },
                Face {
                    color: Color::Blue,
                    face: HashMap::default(),
                },
            ],
            dim: 0,
            test: true,
        }
    }
}

impl Cube {
    fn is_open(&self, pos: &Position) -> bool {
        self.faces[pos.face.order()].is_open(pos.pos)
    }

    fn face_wrap(face: &Color, heading: &Heading) -> (Color, Heading) {
        match (face, heading) {
            (Color::Red, Heading::North) => (Color::Blue, Heading::East),
            (Color::Red, Heading::East) => (Color::White, Heading::East),
            (Color::Red, Heading::West) => (Color::Yellow, Heading::East),
            (Color::Red, Heading::South) => (Color::Green, Heading::South),
            (Color::Green, Heading::North) => (Color::Red, Heading::North),
            (Color::Green, Heading::East) => (Color::White, Heading::North),
            (Color::Green, Heading::West) => (Color::Yellow, Heading::South),
            (Color::Green, Heading::South) => (Color::Orange, Heading::South),
            (Color::Orange, Heading::North) => (Color::Green, Heading::North),
            (Color::Orange, Heading::West) => (Color::Yellow, Heading::West),
            (Color::Orange, Heading::East) => (Color::White, Heading::West),
            (Color::Orange, Heading::South) => (Color::Blue, Heading::West),
            (Color::Yellow, Heading::North) => (Color::Green, Heading::East),
            (Color::Yellow, Heading::East) => (Color::Orange, Heading::East),
            (Color::Yellow, Heading::West) => (Color::Red, Heading::East),
            (Color::Yellow, Heading::South) => (Color::Blue, Heading::South),
            (Color::Blue, Heading::North) => (Color::Yellow, Heading::North),
            (Color::Blue, Heading::West) => (Color::Red, Heading::South),
            (Color::Blue, Heading::South) => (Color::White, Heading::South),
            (Color::Blue, Heading::East) => (Color::Orange, Heading::North),
            (Color::White, Heading::North) => (Color::Blue, Heading::North),
            (Color::White, Heading::West) => (Color::Red, Heading::West),
            (Color::White, Heading::East) => (Color::Orange, Heading::West),
            (Color::White, Heading::South) => (Color::Green, Heading::West),
        }
    }

    fn test_wrap(face: &Color, heading: &Heading) -> (Color, Heading) {
        match (face, heading) {
            (Color::Red, Heading::North) => (Color::Blue, Heading::South),
            (Color::Red, Heading::East) => (Color::White, Heading::West),
            (Color::Red, Heading::West) => (Color::Yellow, Heading::South),
            (Color::Red, Heading::South) =>  (Color::Green, Heading::South),
            (Color::Green, Heading::North) => (Color::Red, Heading::North),
            (Color::Green, Heading::East) => (Color::White, Heading::South),
            (Color::Green, Heading::West) => (Color::Yellow, Heading::West),
            (Color::Green, Heading::South) => (Color::Orange, Heading::South),
            (Color::Orange, Heading::North) => (Color::Green, Heading::North),
            (Color::Orange, Heading::West) => (Color::Yellow, Heading::North),
            (Color::Orange, Heading::East) => (Color::White, Heading::East),
            (Color::Orange, Heading::South) => (Color::Blue, Heading::North),
            (Color::Yellow, Heading::North) => (Color::Red, Heading::East),
            (Color::Yellow, Heading::East) => (Color::Green, Heading::East),
            (Color::Yellow, Heading::West) => (Color::Blue, Heading::West),
            (Color::Yellow, Heading::South) => (Color::Orange, Heading::East),
            (Color::Blue, Heading::North) => (Color::Red, Heading::South),
            (Color::Blue, Heading::West) => (Color::White, Heading::North),
            (Color::Blue, Heading::South) => (Color::Orange, Heading::North),
            (Color::Blue, Heading::East) => (Color::Yellow, Heading::East),
            (Color::White, Heading::North) => (Color::Green, Heading::West),
            (Color::White, Heading::West) => (Color::Orange, Heading::West),
            (Color::White, Heading::East) => (Color::Red, Heading::West),
            (Color::White, Heading::South) => (Color::Blue, Heading::East),
        }
    }

    fn wrap(&self, position: &Position) -> Position {
        let (new_face, new_heading) = if self.test {
            Self::test_wrap(&position.face, &position.heading)
        } else {
            Self::face_wrap(&position.face, &position.heading)
        };
        let new_position = match (&position.heading, &new_heading) {
            (Heading::North, Heading::West) => (self.dim - position.pos.1 , self.dim),
            (Heading::South, Heading::West) => (position.pos.1, self.dim),
            (Heading::North, Heading::East) => (position.pos.1, 0),
            (Heading::South, Heading::East) => (self.dim - position.pos.1, 0),
            (Heading::East, Heading::North) => (self.dim, position.pos.0),
            (Heading::West, Heading::North) => (self.dim, self.dim - position.pos.0),
            (Heading::East, Heading::South) => (0, self.dim - position.pos.0),
            (Heading::West, Heading::South) => (0, position.pos.0),
            (Heading::North, Heading::North) => (self.dim, position.pos.1),
            (Heading::South, Heading::North) => (self.dim, self.dim - position.pos.1),
            (Heading::South, Heading::South) => (0, position.pos.1),
            (Heading::North, Heading::South) => (0, self.dim - position.pos.1),
            (Heading::East, Heading::East) => (position.pos.0, 0),
            (Heading::West, Heading::East) => (self.dim - position.pos.0, 0),
            (Heading::West, Heading::West) => (position.pos.0, self.dim),
            (Heading::East, Heading::West) => (self.dim - position.pos.0, self.dim),
        };
        Position {
            face: new_face,
            heading: new_heading,
            pos: new_position,
        }
    }
}

pub fn part_2() {
    let (directions, cube) = parse_cube(&INPUT_CONFIG);
    let start = &cube.faces[0].face.keys()
        .filter(|(row, _)| *row == 0)
        .min_by_key(|x| x.1)
        .unwrap();
    let mut position = Position {
        face: Color::Red,
        pos: **start,
        heading: Heading::East,
    };
    for inst in directions {
        position.perform(inst, &cube);
    }
    println!("{:?}", position.password(&INPUT_CONFIG));
}