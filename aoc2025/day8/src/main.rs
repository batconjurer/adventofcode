use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = [u64; 3];
type Graph = HashMap<Point, HashSet<Point>>;

/// get the list of sizes of all connected components
fn connected_components(graph: &mut Graph) -> Vec<usize> {
    let mut ccs = vec![];
    loop {
        if graph.is_empty() {
            break;
        }
        ccs.push(next_connected_component(graph));
    }
    ccs
}

/// find a connected component using BFS and remove it from the
/// graph, returning its size
fn next_connected_component(graph: &mut Graph) -> usize {
    let start = *graph.iter().next().unwrap().0;
    let mut visited = HashSet::from([start]);
    let mut stack = vec![start];
    // do bfs
    while let Some(next) = stack.pop() {
        visited.insert(next);
        let neighbors = graph.get(&next).unwrap();
        for n in neighbors {
            if !visited.contains(n) {
                stack.push(*n);
            }
        }
    }
    for n in &visited {
        graph.remove(n);
    }
    visited.len()
}

pub struct DSU{
    ix: Vec<[usize; 2]>,
    set: HashMap<Point, usize>,
}

impl DSU {
    fn new(points: &[Point]) -> Self {
        let n = points.len();
        let set = points
            .iter()
            .enumerate()
            .map(|(ix, p)| (*p, ix))
            .collect();
        Self {
            ix: (0..n).map(|i| [i, 1]).collect(),
            set,
        }
    }

    fn parent(&self, i: usize) -> usize {
        self.ix[i][0]
    }

    fn set_parent(&mut self, i: usize, value: usize) {
        self.ix[i][0] = value;
    }

    fn rank(&self, p: &Point) -> usize {
        let i = *self.set.get(p).unwrap();
        self.ix[i][1]
    }

    fn set_rank(&mut self, i: usize, value: usize){
        self.ix[i][1] = value;
    }

    /// Find the root of union
    fn find(&self, point: &Point) -> usize {
        let mut current = *self.set.get(point).unwrap();
        loop {
            let parent = self.parent(current);
            if parent == current {
                return current
            } else {
                current = parent;
            }
        }
    }

    /// Merge to subsets
    fn unite(&mut self, x: &Point, y: &Point) {
        let set1 = self.find(x);
        let set2 = self.find(y);
        let rank1 = self.rank(x);
        let rank2 = self.rank(y);
        if set1 != set2 {
            match rank1.cmp(&rank2) {
                Ordering::Less => self.set_parent(set1, set2),
                Ordering::Greater => self.set_parent(set2, set1),
                Ordering::Equal => {
                    self.set_parent(set2, set1);
                    self.set_rank(set1, rank1 + 1);
                }
            }
        }
    }
}

fn parse_input(filename: &str) -> Vec<Point>{
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut points = vec![];

    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let coords = line.trim().split(',').enumerate();
        let mut point = [0u64; 3];
        for (ix, coord) in coords {
            point[ix] = u64::from_str_radix(coord, 10).unwrap();
        }
        points.push(point);
        line.clear();
    }
    points
}

// straight line distance between two points squared
fn dist(point1: &Point, point2: &Point) -> u64 {
    point1.iter().zip(point2.iter())
        .map(|(p1, p2)| p1.abs_diff(*p2).pow(2))
        .sum()
}

fn part_one(filename: &str, num_edges: usize) -> (Vec<Point>, Vec<[Point; 2]>) {
    let points = parse_input(filename);
    let mut edges = Vec::with_capacity(points.len() * ( points.len() - 1) / 2);
    // create a map with all distance pairs
    for i in 0..points.len() - 1 {
        for j in  i+1..points.len() {
            let first = points[i];
            let second = points[j];
            edges.push([first, second]);
        }
    }
    edges.sort_unstable_by_key(|[point1 , point2]| dist(point1, point2) );


    // build the graph
    let mut graph = Graph::default();
    for [point1, point2] in edges.iter().take(num_edges) {
        graph
            .entry(*point1)
            .and_modify(| neighbors| {
                neighbors.insert(*point2);
            })
            .or_insert_with(|| HashSet::from([*point2]));
        graph
            .entry(*point2)
            .and_modify(| neighbors| {
                neighbors.insert(*point1);
            })
            .or_insert_with(|| HashSet::from([*point1]));
    }

    let mut ccs = connected_components(&mut graph);
    ccs.sort_unstable();
    let res = ccs.pop().unwrap() * ccs.pop().unwrap() * ccs.pop().unwrap();
    println!("Part one: {res}");
    (points, edges)
}

/// Find the maximum cost edge in a minimum cost spanning tree.
/// This is an adaptation of Kruskal's algorithm. Assumes sorted
/// edges
fn part_two(points: Vec<Point>, edges: &[[Point; 2]]) -> [Point; 2] {
    let mut dsu = DSU::new(&points);
    let mut count = 0;
    let tree_size = points.len() - 1;
    for [x, y] in edges {
        if dsu.find(x) != dsu.find(y) {
            dsu.unite(x, y);
            count += 1;
            if count == tree_size {
                return [*x, *y];
            }
        }
    }
    unreachable!()
}

fn main() {
    let (points, edges) = part_one("input.txt", 1000);
    let [x, y] = part_two(points, &edges);
    println!("Part two: {}", x[0] * y[0]);
}
