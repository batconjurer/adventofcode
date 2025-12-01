use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(filename: &str) -> Vec<ClawGame> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut games = vec![];
    let mut row = 0u64;
    let mut game = ClawGame::default();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        match row.rem_euclid(4) {
            0 => {
                let mut parts = line.trim().split(':');
                assert_eq!(parts.next().unwrap(), "Button A");
                let mut coords = parts.next().unwrap().split(',');
                let x = coords.next().unwrap().split('+').last().unwrap();
                let y = coords.next().unwrap().split('+').last().unwrap();
                game.a_button = (
                    u64::from_str_radix(x, 10).unwrap(),
                    u64::from_str_radix(y, 10).unwrap(),
                );
            }
            1 => {
                let mut parts = line.trim().split(':');
                assert_eq!(parts.next().unwrap(), "Button B");
                let mut coords = parts.next().unwrap().split(',');
                let x = coords.next().unwrap().split('+').last().unwrap();
                let y = coords.next().unwrap().split('+').last().unwrap();
                game.b_button = (
                    u64::from_str_radix(x, 10).unwrap(),
                    u64::from_str_radix(y, 10).unwrap(),
                );
            }
            2 => {
                let mut parts = line.trim().split(':');
                assert_eq!(parts.next().unwrap(), "Prize");
                let mut coords = parts.next().unwrap().split(',');
                let x = coords.next().unwrap().split('=').last().unwrap();
                let y = coords.next().unwrap().split('=').last().unwrap();
                game.prize = (
                    u64::from_str_radix(x, 10).unwrap(),
                    u64::from_str_radix(y, 10).unwrap(),
                );
            }
            _ => {
                games.push(game);
                game = ClawGame::default();
            }
        }
        row += 1;
        line.clear();
    }
    games
}

#[derive(Copy, Clone, Debug, Default)]
struct ClawGame {
    a_button: (u64, u64),
    b_button: (u64, u64),
    prize: (u64, u64),
}

impl ClawGame {
    fn soln(&self) -> Option<u64> {
        let det =
            (self.a_button.0 * self.b_button.1) as i64 - (self.a_button.1 * self.b_button.0) as i64;
        if det != 0 {
            let mut a_button_presses =
                (self.b_button.1 * self.prize.0) as i64 - (self.b_button.0 * self.prize.1) as i64;
            let mut b_button_presses =
                (self.a_button.0 * self.prize.1) as i64 - (self.a_button.1 * self.prize.0) as i64;
            if a_button_presses.rem_euclid(det) == 0 && b_button_presses.rem_euclid(det) == 0 {
                a_button_presses /= det;
                b_button_presses /= det;
                if a_button_presses > 0 && b_button_presses > 0 {
                    return Some(
                        a_button_presses.unsigned_abs() * 3 + b_button_presses.unsigned_abs(),
                    );
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            // A and B are multiples
            let a_button_cost = if self.prize.0.rem_euclid(self.a_button.0) == 0
                && self.prize.1.rem_euclid(self.a_button.1) == 0
            {
                Some((3 * self.prize.0) / self.a_button.0)
            } else {
                None
            };
            let b_button_cost = if self.prize.0.rem_euclid(self.b_button.0) == 0
                && self.prize.1.rem_euclid(self.b_button.1) == 0
            {
                Some(self.prize.0 / self.b_button.0)
            } else {
                None
            };
            std::cmp::min(a_button_cost, b_button_cost)
        }
    }
}

fn part_1(filename: &str) {
    let games = parse_file(filename);
    let score: u64 = games.iter().filter_map(|g| g.soln()).sum();
    println!("Part 1: {score}");
}

fn part_2(filename: &str) {
    let mut games = parse_file(filename);
    for g in games.iter_mut() {
        g.prize.0 += 10000000000000;
        g.prize.1 += 10000000000000;
    }
    let score: u64 = games.iter().filter_map(|g| g.soln()).sum();
    println!("Part 2: {score}")
}

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}

