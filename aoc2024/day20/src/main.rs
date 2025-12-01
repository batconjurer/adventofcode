use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone)]
struct RaceTrack {
    start: (u64, u64),
    end: (u64, u64),
    walls: HashSet<(u64, u64)>
}

fn parse_file(filename: &str) -> RaceTrack  {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut row = 0;
    let mut race = RaceTrack::default();

    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        for (col, c) in line.trim().chars().enumerate() {
            match c {
                'S' => race.start = (row, col as u64),
                'E' => race.end = (row, col as u64),
                '#' => {
                    race.walls.insert((row, col as u64));
                },
                _ => {}
            }
        }
        row += 1;
        line.clear();
    }
    race
}

fn neighbors(pos: (u64, u64), track: &RaceTrack) -> impl Iterator<Item=(u64, u64)> {
    let mut ns = [None; 4];
    if pos.0 > 0 {
        if !track.walls.contains(&(pos.0 - 1, pos.1)) {
            ns[0] = Some((pos.0 - 1, pos.1));
        }
    }
    if pos.1 > 0 {
        if !track.walls.contains(&(pos.0, pos.1 - 1)) {
            ns[1] = Some((pos.0, pos.1 - 1));
        }
    }
    if !track.walls.contains(&(pos.0 + 1, pos.1)) {
        ns[2] = Some((pos.0 + 1, pos.1));
    }
    if !track.walls.contains(&(pos.0, pos.1 + 1)) {
        ns[3] = Some((pos.0, pos.1 + 1));
    }
    ns.into_iter().filter_map(|x| x)
}

fn find_dists(track: &RaceTrack) -> HashMap<(u64, u64), u64> {
    let mut next = Some(track.start);
    let mut dists = HashMap::new();
    dists.insert(track.start, 0);

    let mut curr_dist = 1;
    while let Some(pos) = next {
        if pos == track.end {
            dists.insert(pos, curr_dist);
            return dists;
        }

        for n in neighbors(pos, track) {
            if dists.contains_key(&n) {
                continue;
            }
            dists.insert(pos, curr_dist);
            curr_dist += 1;
            next = Some(n);
            break;
        }
    }
    dists
}

fn cheat_dests(pos: &(u64, u64)) -> impl Iterator<Item=(u64, u64)> {
    let mut ns = [None; 8];
    if pos.0 > 1 {
        ns[0] = Some((pos.0 - 2, pos.1));
    }
    if pos.0 > 0 && pos.1 > 0 {
        ns[1] = Some((pos.0 - 1, pos.1 - 1));
    }
    if pos.0 > 0 {
        ns[2] = Some((pos.0 - 1, pos.1 + 1));
    }
    if pos.1 > 1 {
        ns[3] = Some((pos.0, pos.1 - 2));
    }
    if pos.1 > 0 {
        ns[4] = Some((pos.0 + 1, pos.1 - 1));
    }
    ns[5] = Some((pos.0 + 2, pos.1));
    ns[6] = Some((pos.0 + 1, pos.1 + 1));
    ns[7] = Some((pos.0, pos.1 + 2));
    ns.into_iter().filter_map(|x| x)
}

fn find_cheats(dists: &HashMap<(u64, u64), u64>) {
    let mut total = 0;
    for (pos, dist) in dists.iter() {
        for n in cheat_dests(pos) {
            if let Some(d) = dists.get(&n) {
                if d > dist && d - dist - 2 >= 100 {
                    total += 1;
                }
            }
        }
    }
    println!("Part 1: {total}");
}


fn metric(pos1: (u64, u64), pos2: (u64, u64)) -> u64 {
    pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)
}
fn find_long_cheats(dists: &HashMap<(u64, u64), u64>) {
    let mut cheats = HashSet::new();
    let mut total = 0;
    for (pos1, dist1) in dists.iter() {
        for (pos2, dist2) in dists.iter() {
            if cheats.contains(&[pos2, pos1]) {
                continue;
            } else {
                cheats.insert([pos1, pos2]);
            }
            let m = metric(*pos1, *pos2);
            let savings = dist1.abs_diff(*dist2).saturating_sub(m);
            if m <= 20 && savings >= 100 {
                total += 1;
            }
        }
    }
    println!("Part 2: {total}")
}


fn part_1(filename: &str) {
    let track = parse_file(filename);
    let dists = find_dists(&track);
    find_cheats(&dists);
}

fn part_2(filename: &str) {
    let track = parse_file(filename);
    let dists = find_dists(&track);
    find_long_cheats(&dists);
}

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}
