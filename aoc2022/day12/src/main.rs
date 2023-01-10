use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use rayon::prelude::*;

fn height(character: char) -> u8 {
    if character == 'S' {
        return 0;
    } else if character == 'E' {
        return 25;
    }
    character.to_string().as_bytes()[0] - 97
}

#[derive(Debug, Clone)]
struct Graph {
    pub topo: Vec<Vec<char>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Debug, Clone)]
struct ScenicGraph {
    pub topo: Vec<Vec<char>>,
    pub starts: Vec<(usize, usize)>,
    pub end: (usize, usize)
}

fn parse_input(filename: &str) -> Graph {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut graph = vec![];
    let mut start = (0usize, 0usize);
    let mut end = (0usize, 0usize);
    let mut line_num = 0usize;
    while let Ok(length) = reader.read_line(&mut line) {
        let mut chars: Vec<char> = line.chars().collect();
        chars.pop();
        if chars.is_empty() {
            break;
        }
        if let Some((col, _)) = chars
            .iter()
            .enumerate()
            .find(|(_, element)| **element == 'S') {
            start = (line_num, col);
        }
        if let Some((col, _)) = chars
            .iter()
            .enumerate()
            .find(|(_, element)| **element == 'E') {
            end = (line_num, col);
        }
        graph.push(chars);
        line.clear();
        line_num += 1;
        if length == 0 {
            break;
        }
    }
    Graph {
        topo: graph,
        start,
        end
    }
}

fn parse_scenic(filename: &str) -> ScenicGraph {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut graph = vec![];
    let mut starts = vec![];
    let mut end = (0usize, 0usize);
    let mut line_num = 0usize;
    while let Ok(length) = reader.read_line(&mut line) {
        let mut chars = vec![];
        for (col, c) in line.chars().enumerate() {
            if c == 'a' || c == 'S' {
                starts.push((line_num, col));
            } else if c == 'E' {
                end = (line_num, col);
            }
            if c.is_alphabetic() {
                chars.push(c);
            }
        }
        if chars.is_empty() {
            break;
        }
        graph.push(chars);
        line.clear();
        line_num += 1;
        if length == 0 {
            break;
        }
    }
    ScenicGraph {
        topo: graph,
        starts,
        end
    }
}

fn adjacent((x, y): (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut adj = vec![];
    if 0 < x {
        adj.push((x - 1, y));
    }
    if x < max_x {
        adj.push((x + 1, y));
    }
    if 0 < y {
        adj.push((x, y - 1));
    }
    if y < max_y {
        adj.push((x, y + 1));
    }
    adj
}

fn bfs(graph: &Vec<Vec<char>>, starts: Vec<(usize, usize)>, end: (usize, usize)) -> u64 {
    let starts = starts.into_iter().map(|x| (x, 0u64)).collect::<Vec<_>>();
    let mut queue = VecDeque::from(starts);
    let mut distances = HashMap::new();
    let max_x = graph.len() - 1;
    let max_y = graph[0].len() - 1;
    while let Some((next, dist)) = queue.pop_front() {
        if &dist > distances.get(&next).unwrap_or(&u64::MAX) {
            continue;
        }
        if next == end {
            return dist;
        }
        for pos in adjacent(next, max_x, max_y) {
            if height(graph[pos.0][pos.1]) <= height(graph[next.0][next.1]) + 1 {
                if let Some(prev) = distances.get(&pos) {
                    // we have found a shorter path
                    if *prev > dist + 1 {
                        distances.insert(pos, dist + 1);
                        queue.push_back((pos, dist + 1));
                    }
                } else {
                    distances.insert(pos, dist + 1);
                    queue.push_back((pos, dist + 1));
                }
            }
        }
    }
    u64::MAX
}

fn part_two(graph: ScenicGraph) -> u64 {
    bfs(&graph.topo, graph.starts, graph.end)
}

fn main() {
    let graph = parse_input("input.txt");
    let length = bfs(&graph.topo, vec![graph.start], graph.end);
    println!("Part one: {}", length);
    let graph = parse_scenic("input.txt");
    println!("Part two: {}", part_two(graph));
}
