use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}


fn parse_input(filename: &str) -> HashSet<(i64, i64)> {
    let mut file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut field = HashSet::new();
    let mut row = 0i64;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        for (col, c) in line.trim().chars().enumerate() {
            if c == '#' {
                field.insert((row, col as i64));
            }
        }
        row += 1;
        line.clear();
    }
    field
}

fn neighbors((row, col): (i64, i64)) -> [(i64, i64); 8] {
    [
        (row - 1, col - 1),
        (row, col - 1),
        (row + 1, col - 1),
        (row - 1, col + 1),
        (row, col + 1),
        (row + 1, col + 1),
        (row - 1, col),
        (row + 1, col),
    ]
}

fn consider_moves(
    consider: &[Direction],
    elves: &HashSet<(i64, i64)>,
) -> HashMap<(i64, i64), (i64, i64)> {
    let mut proposals = HashMap::<(i64, i64), Vec::<(i64, i64)>>::new();
    for elf in elves {
        if neighbors(*elf)
            .iter()
            .all(|pos| !elves.contains(pos)) {
            continue;
        }
        for dir in consider {
            match dir {
                Direction::North => if !elves.contains(&(elf.0 - 1, elf.1 - 1))
                    && !elves.contains(&(elf.0 - 1, elf.1))
                    && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                {
                    proposals.entry((elf.0 - 1, elf.1))
                        .and_modify(|x| x.push(*elf))
                        .or_insert(vec![*elf]);
                    break;
                }
                Direction::South => if !elves.contains(&(elf.0 + 1, elf.1 - 1))
                    && !elves.contains(&(elf.0 + 1, elf.1))
                    && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                {
                    proposals.entry( (elf.0 + 1, elf.1))
                        .and_modify(|x| x.push(*elf))
                        .or_insert(vec![*elf]);
                    break;
                }
                Direction::West => if !elves.contains(&(elf.0 - 1, elf.1 - 1))
                    && !elves.contains(&(elf.0, elf.1 - 1))
                    && !elves.contains(&(elf.0 + 1, elf.1 - 1))
                {
                    proposals.entry((elf.0, elf.1 - 1))
                        .and_modify(|x| x.push(*elf))
                        .or_insert(vec![*elf]);
                    break;
                }
                Direction::East => if !elves.contains(&(elf.0 - 1, elf.1 + 1))
                    && !elves.contains(&(elf.0, elf.1 + 1))
                    && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                {
                    proposals.entry((elf.0, elf.1 + 1))
                        .and_modify(|x| x.push(*elf))
                        .or_insert(vec![*elf]);
                    break;
                }
            }
        }
    }
    proposals.retain(|_, v| v.len() == 1);
    proposals.into_iter().map(|(k, v)| (v[0], k)).collect()
}

fn display(elves: &HashSet<(i64, i64)>) {
    let mut min_row = i64::MAX;
    let mut min_col = i64::MAX;
    let mut max_row = i64::MIN;
    let mut max_col = i64::MIN;
    for (row, col) in elves {
        if *row < min_row {
            min_row = *row;
        }
        if *row > max_row {
            max_row = *row;
        }
        if *col < min_col {
            min_col = *col;
        }
        if *col > max_col {
            max_col = *col;
        }
    }
    for row in min_row..=max_row {
        let mut line = String::new();
        for col in min_col..=max_col {
            if elves.contains(&(row, col)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
    println!("============");
}

fn run(filename: &str) {
    let mut elves = parse_input(filename);
    let mut consider = vec![Direction::North, Direction::South, Direction::West, Direction::East];
    let mut round = 0u64;
    loop {
        let moves = consider_moves(&consider, &elves);
        if moves.is_empty() {
            break;
        }
        round += 1;
        consider.rotate_left(1);
        for (elf, new_pos) in moves {
            elves.remove(&elf);
            elves.insert(new_pos);
        }
        if round == 10 {
            let mut min_row = i64::MAX;
            let mut min_col = i64::MAX;
            let mut max_row = i64::MIN;
            let mut max_col = i64::MIN;
            for (row, col) in &elves {
                if *row < min_row {
                    min_row = *row;
                }
                if *row > max_row {
                    max_row = *row;
                }
                if *col < min_col {
                    min_col = *col;
                }
                if *col > max_col {
                    max_col = *col;
                }
            }
            let area = (1 + max_row - min_row) * (1 + max_col - min_col);
            println!("Part one: {}", area as usize - elves.len());
        }
    }

    println!("Part two: {}", round + 1);
}

fn main() {
    run("input.txt");
}
