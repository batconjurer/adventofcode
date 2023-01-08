use std::hash::Hasher;
use crate::*;

pub fn part_two(filename: &str) {
    let mut graph = parse_input(filename);
    let distances = get_all_distances(&graph);

    graph.nodes = graph
        .nodes
        .into_iter()
        .filter(|n| n.flow_rate > 0 || n.id == START_ID)
        .collect();
    graph.nodes.sort_by_key(|n| -(n.flow_rate as i64));
    let start_node = graph.nodes.iter().find(|n| n.id == START_ID).unwrap();
    println!(
        "Part 2: {}",
        branch_and_bound2(start_node, &graph, &distances)
    );
}

fn score_path2(path: [&[&Node]; 2], distances: &Distances) -> u64 {
    let flows = path[0].iter().map(|n| 26 * n.flow_rate).sum::<u64>()
        + path[1].iter().map(|n| 26 * n.flow_rate).sum::<u64>();
    let score = score_path(path[0], distances, 26)
        .saturating_add(score_path(path[1], distances, 26));
    if score > flows {
        0
    } else {
        flows - score
    }
}

enum BestIx {
    You(usize),
    Elephant(usize),
}


fn initial_solution2<'a>(
    start_node: &'a Node,
    graph: &'a Graph,
    distances: &Distances,
) -> [Vec<&'a Node>; 2] {
    let mut your_path = Vec::with_capacity(graph.nodes.len());
    let mut elephant_path = Vec::with_capacity(graph.nodes.len());
    let mut nodes: Vec<_> = graph.nodes.iter().collect();
    nodes.sort_by_key(|node| -(node.flow_rate as i64));

    your_path.insert(0, start_node);
    elephant_path.insert(0, start_node);

    for node in &nodes {
        if node.id == START_ID {
            continue;
        }
        let mut best_ix = None;
        let mut best_score = 0;
        for ix in 1..=your_path.len() {
            your_path.insert(ix, node);
            let score = score_path2([&your_path, &elephant_path], distances);
            if score > best_score {
                best_ix = Some(BestIx::You(ix));
                best_score = score;
            }
            your_path.remove(ix);
        }
        for ix in 1..=elephant_path.len() {
            elephant_path.insert(ix, node);
            let score = score_path2([&your_path, &elephant_path], distances);
            if score > best_score {
                best_ix = Some(BestIx::Elephant(ix));
                best_score = score;
            }
            elephant_path.remove(ix);
        }
        match best_ix {
            Some(BestIx::You(ix)) => your_path.insert(ix, node),
            Some(BestIx::Elephant(ix)) => elephant_path.insert(ix, node),
            None => {}
        }
    }
    [your_path, elephant_path]
}


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct PartialSolution2<'a> {
    you: Vec<&'a Node>,
    elephant: Vec<&'a Node>,
}

impl<'a> PartialSolution2<'a> {
    fn contains(&self, node: &Node) -> bool {
        self.you.contains(&node) || self.elephant.contains(&node)
    }

    fn score(&self, distances: &Distances) -> u64 {
        score_path2([&self.you, &self.elephant], distances)
    }

    fn swapped(&self) -> Self {
        Self {
            you: self.elephant.clone(),
            elephant: self.you.clone(),
        }
    }

    fn heuristic(&self, distances: &Distances, graph: &Graph) -> u64 {
        let mut your_score = 0;
        let mut elephant_score = 0;
        let mut your_time = 0;
        let mut elephant_time = 0;
        let mut windows = self.you.windows(2);
        while let Some([prev, next]) = windows.next() {
            let dist = *distances.get(&(prev.id, next.id)).unwrap();
            your_time += dist + 1;
            if your_time > 26 {
                return 0;
            }
            let flow = (26 - your_time) * next.flow_rate;
            your_score += flow;
        }
        let mut windows = self.elephant.windows(2);
        while let Some([prev, next]) = windows.next() {
            let dist = *distances.get(&(prev.id, next.id)).unwrap();
            elephant_time += dist + 1;
            if elephant_time > 26 {
                return 0;
            }
            let flow = (26 - elephant_time) * next.flow_rate;
            elephant_score += flow;
        }
        for unvisited in graph.nodes
            .iter()
            .filter(|n| !self.contains(n)) {
            if your_time < 25 {
                your_time += 2;
                your_score += (26 - your_time) * unvisited.flow_rate;
            } else if elephant_time < 25 {
                elephant_time += 2;
                elephant_score += (26 - elephant_time) * unvisited.flow_rate;
            } else {
                break;
            }
        }
        your_score + elephant_score
    }
}


fn branch_and_bound2(start_node: &Node, graph: &Graph, distances: &Distances) -> u64 {
    let partial = PartialSolution2 {
        you: vec![start_node],
        elephant: vec![start_node],
    };
    let [first, second] = initial_solution2(start_node, graph, distances);
    let mut best_score = score_path2([&first, &second], distances);
    let mut stack = VecDeque::from([partial]);
    let mut explored = HashSet::new();
    while let Some(mut partial) = stack.pop_front() {
        for node in &graph.nodes {
            let mut saturated = true;
            if !partial.contains(&node) {
                partial.you.push(node);
                if !explored.contains(&partial) && !explored.contains(&partial.swapped()) {
                    let score = partial.heuristic(distances,  &graph);
                    if score > best_score {
                        stack.push_back(partial.clone());
                        explored.insert(partial.clone());
                        saturated = false;
                    }
                }
                partial.you.pop();


                partial.elephant.push(node);
                if !explored.contains(&partial) && !explored.contains(&partial.swapped()) {
                    let score = partial.heuristic(distances, &graph);
                    if score > best_score {
                        stack.push_back(partial.clone());
                        explored.insert(partial.clone());
                        saturated = false;
                    }
                }
                partial.elephant.pop();
            }
            if saturated {
                let new_score = partial.score(distances);
                if new_score > best_score {
                    best_score = new_score;
                }
            }
        }
    }
    best_score
}
