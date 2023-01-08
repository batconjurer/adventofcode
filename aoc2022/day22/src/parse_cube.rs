use super::*;
use part2::*;

pub struct FaceBound {
    pub(crate) row_min: u64,
    row_max: u64,
    pub(crate) col_min: u64,
    col_max: u64,
}

impl FaceBound {
    fn contains(&self, pos: (u64, u64)) -> bool {
        self.row_min <= pos.0
            && pos.0 <= self.row_max
            && self.col_min <= pos.1
            && pos.1 <= self.col_max
    }
}

pub struct Configuration {
    pub faces: [FaceBound; 6],
    dim: u64,
    prefix: &'static str,
    test: bool,
}

pub const TEST_CONFIG: Configuration = Configuration {
    faces: [
        // red
        FaceBound {
            row_min: 0,
            row_max: 3,
            col_min: 8,
            col_max: 11,
        },
        // white
        FaceBound {
            row_min: 8,
            row_max: 11,
            col_min: 12,
            col_max: 15,
        },
        // green
        FaceBound {
            row_min: 4,
            row_max: 7,
            col_min: 8,
            col_max: 11,
        },
        // orange
        FaceBound {
            row_min: 8,
            row_max: 11,
            col_min: 8,
            col_max: 11,
        },
        // yellow
        FaceBound {
            row_min: 4,
            row_max: 7,
            col_min: 4,
            col_max: 7,
        },
        // blue
        FaceBound {
            row_min: 4,
            row_max: 7,
            col_min: 0,
            col_max: 3,
        }
    ],
    dim: 3,
    prefix: "test",
    test: true,
};

pub const INPUT_CONFIG: Configuration = Configuration {
    faces: [
        // red
        FaceBound {
            row_min: 0,
            row_max: 49,
            col_min: 50,
            col_max: 99,
        },
        // white
        FaceBound {
            row_min: 0,
            row_max: 49,
            col_min: 100,
            col_max: 149,
        },
        // green
        FaceBound {
            row_min: 50,
            row_max: 99,
            col_min: 50,
            col_max: 99,
        },
        // orange
        FaceBound {
            row_min: 100,
            row_max: 149,
            col_min: 50,
            col_max: 99,
        },
        // yellow
        FaceBound {
            row_min: 100,
            row_max: 149,
            col_min: 0,
            col_max: 49,
        },
        // blue
        FaceBound {
            row_min: 150,
            row_max: 199,
            col_min: 0,
            col_max: 49,
        }
    ],
    dim: 49,
    prefix: "input",
    test: false,
};

pub fn parse_cube(config: &Configuration) -> (Vec<Instruction>, Cube) {
    // parse the directions
    let mut file = File::open(format!("{}_dirs.txt", config.prefix)).unwrap();
    let mut directions = String::new();
    _ = file.read_to_string(&mut directions).unwrap();
    let directions = parse_directions(directions);

    // parse the board
    let mut cube = Cube {
        dim: config.dim,
        test: config.test,
        ..Default::default()
    };
    let file = File::open(format!("{}.txt", config.prefix)).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut row = 0u64;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        for (col, open) in line.chars().enumerate()
        {
            for (ix, face) in config.faces.iter().enumerate() {
                if face.contains((row, col as u64)) {
                    match open {
                        '.' => cube.faces[ix].face.insert((row - face.row_min, col as u64 - face.col_min), true),
                        '#' => cube.faces[ix].face.insert((row - face.row_min, col as u64 - face.col_min), false),
                        _ => unreachable!(),
                    };
                    break;
                }
            }
        }
        row += 1;
        line.clear();
    }
    (directions, cube)
}