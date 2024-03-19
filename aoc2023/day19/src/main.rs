use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn intersect_interval(first: [u64; 2], second: [u64; 2]) -> Option<[u64; 2]> {
    if first[0] <= second[0] && second[0] <= first[1] && first[1] <= second[1] {
        Some([second[0], first[1]])
    } else if second[0] <= first[0] && first[0] <= second[1] && second[1] <= first[1] {
        Some([first[0], second[1]])
    } else if first[0] <= second[0] && second[1] <= first[1] {
        Some(second)
    } else if second[0] <= first[0] && first[1] <= second[1] {
        Some(first)
    } else {
        None
    }
}

fn intersect(first: &[[u64; 2]], second: &[[u64; 2]]) -> Vec<[u64; 2]> {
    let mut res = vec![];
    for f in first {
        for s in second {
            if let Some(int) = intersect_interval(*f, *s) {
                res.push(int)
            }
        }
    }
    res
}

fn complement(set: &[[u64; 2]]) -> Vec<[u64; 2]> {
    let mut set = set.to_vec();
    set.sort_by_key(|int| int[0]);
    let mut last = 0;
    let mut complement = vec![];
    for s in set {
        if s[0] == 1 {
            last = s[1];
            continue
        }
        if last + 1 <= s[0] - 1 {
            complement.push([last + 1, s[0] - 1]);
        }
        last = s[1]
    }
    if last < 4000 {
        complement.push([last + 1, 4000])
    }
    complement
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Target {
    Accept,
    Reject,
    Workflow(String),
}

impl Target {
    fn to_string(&self) -> String {
        match &self {
            Target::Accept => "A".to_string(),
            Target::Reject => "R".to_string(),
            Target::Workflow(name) => name.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Lt, Gt
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ranges {
    x: Vec<[u64; 2]>,
    m: Vec<[u64; 2]>,
    a: Vec<[u64; 2]>,
    s: Vec<[u64; 2]>,
}

impl Default for Ranges {
    fn default() -> Self {
        Self {
            x: vec![[1, 4000]],
            m: vec![[1, 4000]],
            a: vec![[1, 4000]],
            s: vec![[1, 4000]],
        }
    }
}

impl Ranges {
    fn intersect(&self, other: &Self) -> Self {
        Self {
            x: intersect(&self.x, &other.x),
            m: intersect(&self.m, &other.m),
            a: intersect(&self.a, &other.a),
            s: intersect(&self.s, &other.s),
        }
    }

    fn count(&self) -> u64 {
        self.x.iter().map(|[f, s]| *s - *f + 1).sum::<u64>()
        * self.m.iter().map(|[f, s]| *s - *f + 1).sum::<u64>()
        * self.a.iter().map(|[f, s]| *s - *f + 1).sum::<u64>()
        * self.s.iter().map(|[f, s]| *s - *f + 1).sum::<u64>()
    }

    fn complement(&self, attr: char) -> Self {
        match attr {
            'x' =>  Self {
                x: complement(&self.x),
                ..self.clone()
            },
            'm' => Self {
                m: complement(&self.m),
                ..self.clone()
            },
            'a' => Self {
                a: complement(&self.a),
                ..self.clone()
            },
            's' => Self {
                s: complement(&self.s),
                ..self.clone()
            },
            _ => unreachable!()
        }
    }
}


#[derive(Debug, Clone)]
struct Check {
    var: char,
    op: Op,
    value: u64,
    target: Target
}

#[derive(Debug, Clone)]
enum Rule {
    Default(Target),
    Check(Check)
}

impl<'a> From<&'a Rule> for Ranges {
    fn from(rule: &Rule) -> Self {
        match rule {
            Rule::Default(_) => Default::default(),
            Rule::Check(check) => {
                let mut ranges = Ranges::default();
                let range = if check.op == Op::Gt {
                    vec![[check.value + 1, 4000]]
                } else {
                    if check.value == 1 {
                        vec![]
                    } else {
                        vec![[1, check.value - 1]]
                    }
                };
                match check.var {
                    'x' => ranges.x = range,
                    'm' => ranges.m = range,
                    'a' => ranges.a = range,
                    's' => ranges.s = range,
                    _ => unreachable!(),
                };
                ranges
            }
        }
    }
}

impl Rule {
    fn parse(line: &str) -> Self {
        if line.contains('<') {
            let mut split = line.split('<');
            let var = split.next().unwrap().chars().next().unwrap();
            let rest = split.next().unwrap();
            let mut split = rest.split(':');
            let value = u64::from_str(split.next().unwrap()).unwrap();
            let target = match split.next().unwrap() {
                "A" => Target::Accept,
                "R" => Target::Reject,
                target => Target::Workflow(target.to_string()),
            };
            Self::Check(Check {
                var,
                op: Op::Lt,
                value,
                target,
            })
        } else if line.contains('>') {
            let mut split = line.split('>');
            let var = split.next().unwrap().chars().next().unwrap();
            let rest = split.next().unwrap();
            let mut split = rest.split(':');
            let value = u64::from_str(split.next().unwrap()).unwrap();
            let target = match split.next().unwrap() {
                "A" => Target::Accept,
                "R" => Target::Reject,
                target => Target::Workflow(target.to_string()),
            };
            Self::Check(Check {
                var,
                op: Op::Gt,
                value,
                target,
            })
        } else {
            let target = match line {
                "A" => Target::Accept,
                "R" => Target::Reject,
                target => Target::Workflow(target.to_string()),
            };
            Self::Default(target)
        }
    }

    fn target(&self) -> &Target {
        match &self {
            Rule::Default(target) => target,
            Rule::Check(check) => &check.target,
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>
}

impl Workflow  {
    fn neighbors(&self) -> Vec<(&Target, Ranges)> {
        // the complement of all accepted ranges so far
        // i.e. the intersection of all complements
        let mut last: Option<Ranges> = None;
        let mut neighbors = vec![];
        for rule in &self.rules {
            let mut ranges: Ranges = rule.into();
            if let Some(l) = &last {
                ranges = l.intersect(&ranges);
            }
            if let Rule::Check(check) = rule {
                last = if let Some(last) = last {
                    Some(last.intersect(&ranges.complement(check.var)))
                } else {
                    Some(ranges.complement(check.var))
                };
            }
            neighbors.push((rule.target(), ranges));
        }
        neighbors
    }
}

fn parse_workflows(filename: &str) -> HashMap<String, Workflow> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut workflows = HashMap::new();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let mut split = line.trim().split('{');
        let name = split.next().unwrap().to_string();
        let mut rest = split.next().unwrap().to_string();
        rest.pop();
        let rules = rest.split(',').map(Rule::parse).collect();
        workflows.insert(name.clone(), Workflow{ rules });
        line.clear()
    }
    workflows
}


fn part_two(filename: &str) {
    let workflows = parse_workflows(filename);
    let mut stack = vec![(&workflows[&"in".to_string()], Ranges::default())] ;
    let mut total = 0;
    while let Some((next, possibiliites)) = stack.pop() {
        for (n, ranges) in next.neighbors() {
            if *n == Target::Reject {
                continue
            }
            let new_ranges = ranges.intersect(&possibiliites);
            if *n != Target::Accept {
                stack.push((workflows.get(&n.to_string()).unwrap(), new_ranges));
            } else {
                total += new_ranges.count()
            }
        }
    }

    println!("Part two: {}", total);
}
fn main() {
    part_two("input.txt");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let first = vec![[1u64, 100], [200, 300]];
        let second = vec![[50u64, 250], [251, 252], [270, 400]];
        let expected = vec![[50u64, 100], [200, 250], [251, 252], [270, 300]];
        let mut res = intersect(&first, &second);
        res.sort_by_key(|int| int[0]);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_complement() {
        let set = vec![[1u64, 100], [200, 250], [251, 252], [254, 300]];
        let expected = vec![[101u64, 199], [253, 253], [301, 4000]];
        assert_eq!(complement(&set), expected);
        assert_eq!(complement(&vec![]), vec![[1, 4000]]);
        assert!(complement(&vec![[1, 4000]]).is_empty());
    }

    #[test]
    fn test_rule_to_ranges() {
        let rule = Rule::Default(Target::Reject);
        let ranges: Ranges = (&rule).into();
        assert_eq!(ranges, Ranges {
            x: vec![[1, 4000]],
            m: vec![[1, 4000]],
            a: vec![[1, 4000]],
            s: vec![[1, 4000]],
        });

        let rule = Rule::Check(Check{
            var: 'x',
            op: Op::Gt,
            value: 10,
            target: Target::Workflow("one".to_string()),
        });
        let ranges: Ranges = (&rule).into();
        assert_eq!(ranges, Ranges {
            x: vec![[11, 4000]],
            m: vec![[1, 4000]],
            a: vec![[1, 4000]],
            s: vec![[1, 4000]],
        });
        assert_eq!(ranges.complement('x'), Ranges {
            x: vec![[1, 10]],
            m: vec![[1, 4000]],
            a: vec![[1, 4000]],
            s: vec![[1, 4000]],
        });
    }

    #[test]
    fn test_workflow_neighbors() {
        let wf = Workflow {
            rules: vec![
                Rule::parse("x>10:one"),
                Rule::parse("m<20:two"),
                Rule::parse("a>30:R"),
                Rule::Default(Target::Accept)],
        };

        let expected = vec![
            (
                Target::Workflow("one".to_string()),
                Ranges{x: vec![[11, 4000]], m: vec![[1, 4000]], a: vec![[1, 4000]], s: vec![[1, 4000]]}
            ),
            (
                Target::Workflow("two".to_string()),
                Ranges{x: vec![[1, 10]], m: vec![[1, 19]], a: vec![[1, 4000]],s: vec![[1, 4000]]}
            ),
            (
                Target::Reject,
                Ranges{x: vec![[1, 10]], m: vec![[20, 4000]], a: vec![[31, 4000]], s: vec![[1, 4000]]}
            ),
            (
                Target::Accept,
                Ranges{x: vec![[1, 10]], m: vec![[20, 4000]], a: vec![[1, 30]], s: vec![[1, 4000]]}
            ),
        ];

        let res : Vec<(Target, Ranges)> = wf.neighbors()
            .into_iter()
            .map(|(k, v)| (k.clone(), v))
            .collect();
        assert_eq!(expected, res);
        let res = Ranges{x: vec![[1, 10]], m: vec![[20, 4000]], a: vec![[1, 30]], s: vec![[1, 4000]]}.count();
        assert_eq!(10 * 3981 * 30 * 4000, res);
    }
}