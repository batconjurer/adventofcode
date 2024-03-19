use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Board {
    inner: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for row in 0..self.rows {
            let r: String = self.inner[row].iter().cloned().collect();
            str.push_str(&r);
            str.push('\n');
        }
        f.write_str(&str)
    }
}

impl Board {
    fn tilt_north(&mut self) {
        for row in 1..self.rows {
            for col in 0..self.cols {
                if self.inner[row][col] == 'O' {
                    let mut fallen = false;
                    for north in 1..=row {
                        if self.inner[row - north][col] != '.' {
                            self.inner[row][col] = '.';
                            self.inner[row - north + 1][col] = 'O';
                            fallen = true;
                            break;
                        }
                    }
                    if self.inner[0][col] == '.' && !fallen {
                        self.inner[row][col] = '.';
                        self.inner[0][col] = 'O';
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for row in 0..self.rows - 1 {
            let row = self.rows - 2 - row;
            for col in 0..self.cols {
                if self.inner[row][col] == 'O' {
                    let mut fallen = false;
                    for south in row + 1..self.rows {
                        if self.inner[south][col] != '.' {
                            self.inner[row][col] = '.';
                            self.inner[south - 1][col] = 'O';
                            fallen = true;
                            break;
                        }
                    }
                    if self.inner[self.rows - 1][col] == '.' && !fallen {
                        self.inner[row][col] = '.';
                        self.inner[self.rows - 1][col] = 'O';
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for col in 0..self.cols - 1 {
            let col = self.cols - 2 - col;
            for row in 0..self.rows {
                if self.inner[row][col] == 'O' {
                    let mut fallen = false;
                    for east in col + 1..self.cols {
                        if self.inner[row][east] != '.' {
                            self.inner[row][col] = '.';
                            self.inner[row][east -1] = 'O';
                            fallen = true;
                            break;
                        }
                    }
                    if self.inner[row][self.cols - 1] == '.' && !fallen {
                        self.inner[row][col] = '.';
                        self.inner[row][self.cols - 1] = 'O';
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for col in 1..self.cols {
            for row in 0..self.rows {
                if self.inner[row][col] == 'O' {
                    let mut fallen = false;
                    for west in 1..=col {
                        if self.inner[row][col - west] != '.' {
                            self.inner[row][col] = '.';
                            self.inner[row][col -west + 1] = 'O';
                            fallen = true;
                            break;
                        }
                    }
                    if self.inner[row][0] == '.' && !fallen {
                        self.inner[row][col] = '.';
                        self.inner[row][0] = 'O';
                    }
                }
            }
        }
    }

    fn north_load(&self) -> usize {
        let mut total = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.inner[row][col] == 'O' {
                    total += self.rows - row;
                }
            }
        }
        total
    }

    fn spin_wash(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
}


fn parse(filename: &str) -> Board {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();


    let mut board = Board{inner: vec![], rows: 0, cols: 0};
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }

        let row = line.trim().chars().collect();
        board.inner.push(row);
        line.clear();
    }
    board.rows = board.inner.len();
    board.cols = board.inner[0].len();
    board
}

fn part_one(filename: &str)  {
    let mut board= parse(filename);
    board.tilt_north();
    println!("Part one: {}", board.north_load());
}

fn part_two(filename: &str) {
    let mut board = parse(filename);
    let mut cache: HashMap<Board, usize> = HashMap::new();
    let mut cycle_length = 0;
    let mut cycle_start = 0;
    for ix in 1..=1000000000 {
        if !cache.contains_key(&board) {
            let b = board.clone();
            board.spin_wash();
            cache.insert(b, ix);
        } else {
            let s = cache.get(&board).unwrap();
            cycle_start = *s;
            cycle_length = ix - s;
            break;
        }
    }
    let remainder = (1000000000 - cycle_start) % cycle_length;
    for _ in 0..=remainder {
        board.spin_wash()
    }
    println!("Part two: {}", board.north_load());
}
fn main() {
    part_one("input.txt");
    part_two("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let mut board_original = parse("test.txt");
        let mut board = board_original.clone();
        board.tilt_north();
        let expected = Board {
            inner: vec![
                "OOOO.#.O..".chars().collect(),
                "OO..#....#".chars().collect(),
                "OO..O##..O".chars().collect(),
                "O..#.OO...".chars().collect(),
                "........#.".chars().collect(),
                "..#....#.#".chars().collect(),
                "..O..#.O.O".chars().collect(),
                "..O.......".chars().collect(),
                "#....###..".chars().collect(),
                "#....#....".chars().collect(),
            ],
            rows: board.rows,
            cols: board.cols,
        };
        assert_eq!(board, expected);
        board.tilt_west();
        let expected = Board {
            inner: vec![
                "OOOO.#O...".chars().collect(),
                "OO..#....#".chars().collect(),
                "OOO..##O..".chars().collect(),
                "O..#OO....".chars().collect(),
                "........#.".chars().collect(),
                "..#....#.#".chars().collect(),
                "O....#OO..".chars().collect(),
                "O.........".chars().collect(),
                "#....###..".chars().collect(),
                "#....#....".chars().collect(),
            ],
            rows: board.rows,
            cols: board.cols,
        };
        assert_eq!(board, expected);
        board.tilt_south();
        let expected = Board {
            inner: vec![
                ".....#....".chars().collect(),
                "....#.O..#".chars().collect(),
                "O..O.##...".chars().collect(),
                "O.O#......".chars().collect(),
                "O.O....O#.".chars().collect(),
                "O.#..O.#.#".chars().collect(),
                "O....#....".chars().collect(),
                "OO....OO..".chars().collect(),
                "#O...###..".chars().collect(),
                "#O..O#....".chars().collect(),
            ],
            rows: board.rows,
            cols: board.cols,
        };
        assert_eq!(board, expected);
        board.tilt_east();
        let expected = Board {
            inner: vec![
                ".....#....".chars().collect(),
                "....#...O#".chars().collect(),
                "...OO##...".chars().collect(),
                ".OO#......".chars().collect(),
                ".....OOO#.".chars().collect(),
                ".O#...O#.#".chars().collect(),
                "....O#....".chars().collect(),
                "......OOOO".chars().collect(),
                "#...O###..".chars().collect(),
                "#..OO#....".chars().collect(),
            ],
            rows: board.rows,
            cols: board.cols,
        };
        assert_eq!(board, expected);
        board_original.spin_wash();
        assert_eq!(board_original, expected)

    }
}