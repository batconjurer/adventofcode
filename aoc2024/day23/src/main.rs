use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(Default, Clone, Debug)]
struct Graph {
    inner: HashMap<String, HashSet<String>>
}

fn parse_file(filename: &str) -> Graph  {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut graph = Graph::default();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let mut ends = line.trim().split('-');
        let s = ends.next().unwrap();
        let t = ends.next().unwrap();
        graph.inner.entry(s.to_string())
            .and_modify(|n| {
                n.insert(t.to_string());
            })
            .or_insert(HashSet::from([t.to_string()]));
        graph.inner.entry(t.to_string())
            .and_modify(|n| {
                n.insert(s.to_string());
            })
            .or_insert(HashSet::from([s.to_string()]));
        line.clear();
    }
    graph
}

struct Clique<'a, const N: usize>([&'a str; N]);

impl<'a, const N: usize> Clique<'a, N> {
    fn from(inner: [&'a str; N]) -> Self {
        Self(inner)
    }
}

impl<const N: usize> Hash for Clique<'_, N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut combined = [0u64; 2];
        for ix in 0..N {
            let next_chars = self.0[ix].chars().collect::<Vec<_>>();
            combined[0] = combined[0] ^ u64::from(next_chars[0]);
            combined[1] = combined[1] ^ u64::from(next_chars[1]);
        }
        combined.hash(state);
    }
}

impl<const N: usize> PartialEq for Clique<'_, N> {
    fn eq(&self, other: &Self) -> bool {
        let hash_self = HashSet::from(self.0);
        let hash_other = HashSet::from(other.0);
        hash_other == hash_self
    }
}

impl<const N: usize> Eq for Clique<'_, N> { }

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct DynClique<'a>(HashSet<&'a str>);

impl Hash for DynClique<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut combined = [0u64; 2];
        for node in &self.0 {
            let next_chars = node.chars().collect::<Vec<_>>();
            combined[0] = combined[0] ^ u64::from(next_chars[0]);
            combined[1] = combined[1] ^ u64::from(next_chars[1]);
        }
        combined.hash(state);
    }
}


struct State<'a>{
    r: HashSet<&'a str>,
    p: HashSet<&'a str>,
    x: HashSet<&'a str>,
    cliques: Rc<RefCell<HashSet<DynClique<'a>>>>,
}

impl<'a> State<'a> {
    fn from(graph: &'a Graph) -> Self {
        Self {
            r: Default::default(),
            p: graph.inner.keys().map(|x| x.as_str()).collect(),
            x: Default::default(),
            cliques: Rc::new(Default::default()),
        }
    }

    fn report_clique(&mut self) {
        let clique = std::mem::take(&mut self.r);
        let mut cliques = self.cliques.borrow_mut();
        cliques.insert(DynClique(clique));
    }
}

fn bron_kerbosch<'a>(mut state: State<'a>, graph: &'a Graph) {
    if state.p.is_empty() && state.x.is_empty() {
        state.report_clique();
    }
    if state.p.is_empty() {
        return;
    }
    let pivot = state.p
        .iter()
        .max_by_key(|u| graph.inner.get(&u.to_string()).unwrap().len())
        .unwrap();
    let pivot_ns = graph.inner
        .get(&pivot.to_string())
        .unwrap()
        .iter()
        .map(|s| s.as_str())
        .collect();
    for v in state.p.clone().difference(&pivot_ns) {
        let ns = graph.inner
            .get(&v.to_string())
            .unwrap()
            .iter()
            .map(|s| s.as_str())
            .collect();
        bron_kerbosch(
            State {
                r: {
                    let mut r = state.r.clone();
                    r.insert(v);
                    r
                },
                p: state.p.intersection(&ns).map(|s| *s).collect(),
                x: state.x.intersection(&ns).map(|s| *s).collect(),
                cliques: state.cliques.clone(),
            },
            graph,
        );
        state.p.remove(v);
        state.x.insert(v);
    }
}


fn part1(filename: &str) {
    let graph = parse_file(filename);
    let mut cliques = HashSet::new();
    for (node, ns) in &graph.inner {
        if let Some('t') = node.chars().next() {
            for n1 in ns {
                for n2 in ns {
                    if graph.inner.get(n1).unwrap().contains(n2) {
                        cliques.insert(Clique::<3>::from([node, n1, n2]));
                    }
                }
            }
        }
    }
    println!("Part 1: {}", cliques.len());
}

fn part2(filename: &str) {
    let graph = parse_file(filename);
    let state = State::from(&graph);
    let cliques = state.cliques.clone();
    bron_kerbosch(state, &graph);
    let max_clique = cliques
        .borrow()
        .iter()
        .max_by_key(|x| x.0.len()).map(|x| x.clone())
        .unwrap();
    let mut password_parts = max_clique.0.into_iter().collect::<Vec<_>>();
    password_parts.sort();
    let mut  password = String::new();
    for part in password_parts {
        password.push_str(part);
        password.push(',');
    }
    password.pop();
    println!("Part 2: {}", password);
}

fn main() {
    part1("input.txt");
    part2("input.txt");
}
