use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Module {
    FlipFlop(FlipFlop),
    Conj(Conjunction),
}

impl Module {
    fn receive(&mut self, src: String, pulse: bool) -> Option<(&[String], bool)> {
        match self {
            Self::FlipFlop(flip) => flip.receive(pulse),
            Self::Conj(conj) => conj.receive(src, pulse),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FlipFlop {
    status: bool,
    targets: Vec<String>,
}

impl FlipFlop {
    fn receive(&mut self, pulse: bool) -> Option<(&[String], bool)> {
        if !pulse {
            self.status = !self.status;
            Some((&self.targets, self.status))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Conjunction {
    remembered: BTreeMap<String, bool>,
    targets: Vec<String>,
}

impl Conjunction {
    fn receive(&mut self, src: String, pulse: bool) -> Option<(&[String], bool)> {
        self.remembered.insert(src, pulse);
        if self.remembered.values().all(|f| *f) {
            Some((&self.targets, false))
        } else {
            Some((&self.targets, true))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pulse {
    pulse: bool,
    src: String,
    target: String,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = format!("{} sends {} -> {}", self.src, if self.pulse {"high"} else {"low"}, self.target);
        f.write_str(&str)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Circuit {
    broadcaster: Vec<String>,
    pulse_queue: VecDeque<Pulse>,
    modules: BTreeMap<String, Module>
}

enum ButtonResult {
    Pulse(bool),
    Halt,
}

impl Circuit {

    fn initialize(&mut self) {
        for target in &self.broadcaster {
            self.pulse_queue.push_back(Pulse {
                pulse: false,
                src: "broadcaster".to_string(),
                target: target.to_string(),
            });
        }
    }

    fn push_button(&mut self, stop_on: &str) -> Option<(u64, u64)> {
        let mut lows = 1;
        let mut highs = 0;
        loop {
            if self.pulse_queue.is_empty() {
                break;
            }
            match self.pulse(stop_on) {
                Some(ButtonResult::Halt) => return None,
                Some(ButtonResult::Pulse(pulse)) => if pulse {
                    highs += 1;
                } else {
                    lows += 1;
                }
                None => {}
            }
        }
        self.initialize();
        Some((lows, highs))
    }

    fn pulse(&mut self, stop_on: &str) -> Option<ButtonResult> {
        let Some(pulse) = self.pulse_queue.pop_front() else {
            return None
        };
        if pulse.target.as_str() == stop_on && !pulse.pulse {
            return Some(ButtonResult::Halt)
        }
        if let Some(module) = self.modules.get_mut(&pulse.target) {
            if let Some((targets, new_pulse)) = module
                .receive(pulse.src, pulse.pulse) {
                for target in targets {
                    self.pulse_queue.push_back(Pulse{
                        pulse: new_pulse,
                        src: pulse.target.to_string(),
                        target: target.to_string(),
                    });
                }
            }
        }
        Some(ButtonResult::Pulse(pulse.pulse))
    }
}

fn parse(filename: &str) -> Circuit {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut circuit = Circuit {
        broadcaster: vec![],
        pulse_queue: Default::default(),
        modules: Default::default(),
    };
    let mut conjs = HashMap::<String, Conjunction>::new();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        if line.contains("broadcaster") {
            let broadcaster = line.trim().split("->").nth(1).unwrap();
            circuit.broadcaster = broadcaster.split(',')
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        } else if line.contains('%') {
            let flip_flop = line.trim().replace('%', "");
            let mut flip_flop = flip_flop.split("->");
            let name = flip_flop.next().unwrap();
            let targets = flip_flop.next().unwrap();
            let targets = targets.split(',').map(|t| t.to_string()).collect();
            circuit.modules.insert(name.to_string(), Module::FlipFlop(FlipFlop{
                status: false,
                targets,
            }));
        } else if line.contains('&') {
            let conj = line.trim().replace('&', "");
            let mut conj = conj.split("->");
            let name = conj.next().unwrap();
            let targets = conj.next().unwrap();
            let targets = targets.split(',').map(|t| t.to_string()).collect();
            conjs.insert(name.to_string(),Conjunction{
                remembered: Default::default(),
                targets,
            });
        }
        line.clear();
    }
    for (conj, m) in conjs.iter_mut() {
        let inputs: Vec<_> = circuit.modules
            .iter()
            .filter_map(|(n, m)| if let Module::FlipFlop(flip) = m {
                flip.targets.contains(conj).then_some(n)
            } else {
                None
            }).collect();
        m.remembered = inputs
            .into_iter()
            .map(|n| (n.to_string(), false))
            .collect();

    }
    for (name, conj) in conjs {
        circuit.modules.insert(name, Module::Conj(conj));
    }
    circuit
}

fn part_one(filename: &str) {
    let mut circuit = parse(filename);
    circuit.initialize();
    let mut lows = 0;
    let mut highs = 0;
    for _ in 0..1000 {
        let (new_lows, new_highs) = circuit.push_button("").unwrap();
        lows += new_lows;
        highs += new_highs;
    }
    println!("Part one: {}", lows * highs);
}


/// The `rx` module receives output when all of `ks`, `jf`, `qs` and
/// `zk` do. We calculate the number of iterations for each of these
///  and compute the lcm (they all turn out to be prime).
fn part_two(filename: &str) {
    let mut circuit = parse(filename);
    circuit.initialize();
    let compute_iters = |stop_on: &str, mut circuit: Circuit| {
        let mut iters = 1u64;
        loop {
            if circuit.push_button(stop_on).is_none() {
                break;
            }
            iters += 1;
        }
        iters
    };

    let res = compute_iters("ks", circuit.clone())
        * compute_iters("jf", circuit.clone())
        * compute_iters("qs", circuit.clone())
        * compute_iters("zk", circuit.clone());
    println!("Part two: {}", res);
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
}
