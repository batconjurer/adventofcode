use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Visibility {
    Hidden(u8),
    Visible,
    Unknown,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Unknown
    }
}

enum VisibilityType {
    Above,
    Below,
    Left,
    Right,
}

#[derive(Debug, Clone, Default)]
struct TotalVisibility {
    above: Visibility,
    from_left: Visibility,
    below: Visibility,
    from_right: Visibility,
}

impl TotalVisibility {
    fn all_hidden(&self) -> bool {
        if let TotalVisibility {
            above: Visibility::Hidden(_),
            from_left: Visibility::Hidden(_),
            below: Visibility::Hidden(_),
            from_right: Visibility::Hidden(_),
        } = self {
            true
        } else {
            false
        }
    }

    fn get(&self, ty: VisibilityType) -> &Visibility {
        match ty {
            VisibilityType::Above => &self.above,
            VisibilityType::Below => &self.below,
            VisibilityType::Left => &self.from_left,
            VisibilityType::Right => &self.from_right,
        }
    }
}

fn parse_forest(file_name: &str) -> Result<Vec<Vec<u8>>, std::io::Error> {
    // open target file
    let file = File::open(file_name)?;

    let mut forest = vec![];
    // uses a reader buffer
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                // EOF: save last file address to restart from this address for next run
                if bytes_read == 0 {
                    break;
                }
                forest.push(line
                    .chars()
                    .filter_map(|c| u8::from_str_radix(&c.to_string(), 10).ok())
                    .collect());

                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
    Ok(forest)
}

fn determine_visibility(
    height: u8,
    row: usize,
    col: usize,
    forest: &[Vec<u8>],
    visible: &HashMap<(usize, usize), TotalVisibility>,
    ty: VisibilityType,
) -> Visibility {
    if let Visibility::Hidden(hght) = visible[&(row, col)].get(ty) {
        if hght >= &height {
            Visibility::Hidden(*hght)
        } else {
            Visibility::Visible
        }
    } else if forest[row][col] >= height {
        Visibility::Hidden(forest[row][col])
    } else {
        Visibility::Visible
    }
}

fn run(file_name: &str) -> HashMap<(usize, usize), TotalVisibility>
{
    let forest = parse_forest(file_name).unwrap();
    let rows = forest.len();
    let cols = forest[0].len();

    let mut visible = HashMap::new();
    for (row, line) in forest.iter().enumerate() {
        for (col, height) in line.iter().enumerate() {
            let above = if row == 0 {
                Visibility::Visible
            } else {
                determine_visibility(
                    *height,
                    row - 1,
                    col,
                    &forest,
                    &visible,
                    VisibilityType::Above,
                )
            };

            let from_left = if col == 0 {
                Visibility::Visible
            } else {
                determine_visibility(
                    *height,
                    row,
                    col - 1,
                    &forest,
                    &visible,
                    VisibilityType::Left,
                )
            };

            visible.insert((row, col), TotalVisibility {
                above,
                from_left,
                ..Default::default()
            });

        }
    }

    for (row, line) in forest.iter().rev().enumerate() {
        for (col, height) in line.iter().rev().enumerate() {
            let row = rows - row - 1;
            let col = cols - col - 1;
            let below = if row == rows - 1 {
                Visibility::Visible
            } else {
                determine_visibility(
                    *height,
                    row + 1,
                    col,
                    &forest,
                    &visible,
                    VisibilityType::Below,
                )
            };

            let from_right = if col == cols - 1 {
                Visibility::Visible
            } else {
                determine_visibility(
                    *height,
                    row,
                    col + 1,
                    &forest,
                    &visible,
                    VisibilityType::Right,
                )
            };
            visible.entry((row, col)).and_modify(|vis| {
                vis.below = below;
                vis.from_right = from_right;
            });
        }
    }
    visible
}

fn part_one(filename: &str) {
    let visible = run(filename);
    let mut count = visible.len();
    for vis in visible.values() {
        if vis.all_hidden() {
            count -= 1;
        }
    }
    println!("Hidden trees: {}", count);
}

fn part_two(filename: &str) {
    let forest = parse_forest(filename).unwrap();
    let mut max_score = 0u64;
    for row in 0..forest.len() {
        for col in 0..forest[0].len() {
            max_score = std::cmp::max(
                max_score,
                scenic_score(&forest, (row, col))
            );
        }
    }
    println!("Max scenic score: {}", max_score);
}

fn scenic_score(forest: &[Vec<u8>], pos: (usize, usize)) -> u64 {
    let height = forest[pos.0][pos.1];
    let (mut row, col) = (pos.0 as i64 - 1, pos.1);
    let mut above_dist =  0;
    while row >= 0 {
        above_dist += 1;
        if forest[row as usize][col] < height {
            row -= 1;
        } else {
            break;
        }
    }
    let (mut row, col) = (pos.0 + 1, pos.1);
    let mut below_dist = 0;
    while row < forest.len() {
        below_dist += 1;
        if forest[row][col] < height {
            row += 1;
        } else {
            break;
        }
    }

    let (row, mut col) = (pos.0, pos.1 as i64 - 1);
    let mut left_dist = 0;
    while col >= 0 {
        left_dist += 1;
        if forest[row][col as usize] < height {
            col -= 1;
        } else {
            break;
        }
    }
    let (row, mut col) = (pos.0, pos.1 + 1);
    let mut right_dist = 0;
    while col < forest[0].len() {
        right_dist += 1;
        if forest[row][col] < height {
            col += 1;
        } else {
            break;
        }
    }
    above_dist * below_dist * left_dist * right_dist
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
}
