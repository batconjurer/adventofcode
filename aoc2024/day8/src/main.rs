use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
struct Antennae {
    max_row: i64,
    max_col: i64,
    kinds: HashMap<char, Vec<(i64, i64)>>
}


fn parse_file(filename: &str) -> Antennae {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut antennae = Antennae::default();
    let mut row = 0i64;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let mut entries = line.trim().chars().enumerate();
        for (col, c) in entries.by_ref() {
            let col = col as i64;
            if c.is_alphanumeric() {
                antennae.kinds.entry(c)
                    .and_modify(|v| v.push((row, col)))
                    .or_insert(vec![(row, col)]);
            }
        }

        row += 1;
        line.clear();
    }
    antennae.max_row = row;
    antennae.max_col = row;

    antennae
}

fn find_antinodes(nodes: &[(i64, i64)], max_row: i64, max_col: i64, antis: &mut HashSet<(i64, i64)>) {
    for (ix, n1)  in nodes.iter().enumerate() {
        for n2 in  &nodes[ix+1..] {

            let dy = n2.0  - n1.0;
            let dx = n2.1 - n1.1;

            if n1.0  >=  dy && n1.0 - dy < max_row && n1.1 >= dx &&  n1.1 - dx < max_col {
                antis.insert((n1.0 - dy, n1.1 - dx));
            }

            if n2.0  + dy >=  0 && n2.0 + dy < max_row && n2.1 + dx >= 0 &&  n2.1 + dx < max_col {
                antis.insert((n2.0 + dy, n2.1 + dx));
            }
        }
    }
}

fn find_antinodes_resonant(nodes: &[(i64, i64)], max_row: i64, max_col: i64, antis: &mut HashSet<(i64, i64)>) {
    for (ix, i)  in nodes.iter().enumerate() {
        for j in  &nodes[ix+1..] {
            let mut n1 = *i;
            let mut n2 = *j;
            let dy = n2.0  - n1.0;
            let dx = n2.1 - n1.1;
            antis.insert(n1);
            antis.insert(n2);
            while n1.0  >=  dy && n1.0 - dy < max_row && n1.1 >= dx &&  n1.1 - dx < max_col {
                    antis.insert((n1.0 - dy, n1.1 - dx));
                    n1.0 = n1.0 - dy;
                    n1.1 = n1.1 - dx;
            }


            while n2.0  + dy >=  0 && n2.0 + dy < max_row && n2.1 + dx >= 0 &&  n2.1 + dx < max_col {
                antis.insert((n2.0 + dy, n2.1 + dx));
                n2.0 = n2.0 + dy;
                n2.1 = n2.1 + dx;
            }
        }
    }
}


fn part_1(filename: &str) {
    let mut antennae = parse_file(filename);
    let mut antis = HashSet::default();
    for (_, positions) in &antennae.kinds {
        find_antinodes(positions, antennae.max_row, antennae.max_col, &mut antis);
    }
    println!("Part 1: {}", antis.len())
}

fn part_2(filename: &str) {
    let mut antennae = parse_file(filename);
    let mut antis = HashSet::default();
    for (_, positions) in &antennae.kinds {
        find_antinodes_resonant(positions, antennae.max_row, antennae.max_col, &mut antis);
    }
    println!("Part 2: {}", antis.len())
}

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}
