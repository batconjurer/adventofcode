mod part2;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use crate::part2::part_two;

type Id = [char; 2];
type Distances = HashMap<(Id, Id), u64>;
struct Graph {
    adjacency: HashMap<Node, Vec<Id>>,
    nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    pub flow_rate: u64,
    pub id: Id,
}

fn parse_input(filename: &str) -> Graph {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut graph = HashMap::new();
    let mut nodes = HashSet::new();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let words: Vec<_> = line.split_whitespace().collect();
        let node = Node {
            flow_rate: u64::from_str_radix(&words[4][5..].trim_end_matches(";"), 10).unwrap(),
            id: words[1].chars().collect::<Vec<char>>().try_into().unwrap(),
        };
        nodes.insert(node.clone());
        let adjacent: Vec<[char; 2]> = words[9..]
            .iter()
            .map(|node| {
                node.trim_end_matches(",")
                    .chars()
                    .collect::<Vec<char>>()
                    .try_into()
                    .unwrap()
            })
            .collect();
        graph.insert(node, adjacent);
        line.clear()
    }
    Graph {
        adjacency: graph,
        nodes: nodes.into_iter().collect(),
    }
}

fn get_all_distances(graph: &Graph) -> Distances {
    let mut distances = HashMap::new();
    for (node, neighbors) in &graph.adjacency {
        distances.insert((node.id, node.id), 0);
        for neighbor in neighbors {
            distances.insert((node.id, *neighbor), 1);
        }
    }
    for k in 0..graph.nodes.len() {
        for i in 0..graph.nodes.len() {
            for j in 0..graph.nodes.len() {
                let dist_ij = distances
                    .get(&(graph.nodes[i].id, graph.nodes[j].id))
                    .cloned()
                    .unwrap_or(u64::MAX);
                let dist_ik = distances
                    .get(&(graph.nodes[i].id, graph.nodes[k].id))
                    .cloned()
                    .unwrap_or(u64::MAX);
                let dist_kj = distances
                    .get(&(graph.nodes[k].id, graph.nodes[j].id))
                    .cloned()
                    .unwrap_or(u64::MAX);
                if dist_ij > dist_ik.saturating_add(dist_kj) {
                    distances.insert(
                        (graph.nodes[i].id, graph.nodes[j].id),
                        dist_ik.saturating_add(dist_kj),
                    );
                }
            }
        }
    }
    distances
}

const START_ID: Id = ['A', 'A'];

fn part_one(filename: &str) {
    let mut graph = parse_input(filename);
    let distances = get_all_distances(&graph);

    graph.nodes = graph
        .nodes
        .into_iter()
        .filter(|n| n.flow_rate > 0 || n.id == START_ID)
        .collect();
    let start_node = graph.nodes.iter().find(|n| n.id == START_ID).unwrap();
    println!(
        "Part 1: {}",
        search_tree(start_node, &graph, &distances)
    );
}

fn score_path(path: &[&Node], distances: &Distances, max_time: u64) -> u64 {
    let mut score = 0;
    let mut windows = path.windows(2);
    let mut total_time = 0;
    while let Some([prev, next]) = windows.next() {
        let dist = *distances.get(&(prev.id, next.id)).unwrap();
        total_time += dist + 1;
        if total_time > max_time {
            return u64::MAX;
        }
        let flow = total_time * next.flow_rate;
        score += flow;
    }
    score
}

#[derive(Debug, Clone)]
struct PartialSolution<'a> {
    path: Vec<&'a Node>,
}

fn search_tree(start_node: &Node, graph: &Graph, distances: &Distances) -> u64 {
    let mut score = 0;
    let partial = PartialSolution {
        path: vec![start_node],
    };

    let mut stack = VecDeque::from([partial]);
    while let Some(mut partial) = stack.pop_front() {
        for node in &graph.nodes {
            let mut saturated = true;
            if !partial.path.contains(&node) {
                partial.path.push(node);
                let score = score_path(&partial.path, distances, 30);
                if score < u64::MAX {
                    stack.push_back(partial.clone());
                    saturated = false;
                }
                partial.path.pop();
            }
            if saturated {
                let path_score = score_path(&partial.path, distances, 30);
                let new_score =
                    partial.path.iter().map(|n| 30 * n.flow_rate).sum::<u64>()
                        - path_score;
                score = std::cmp::max(score, new_score);
            }
        }
    }
    score
}
fn main() {
    part_two("input.txt");
}
