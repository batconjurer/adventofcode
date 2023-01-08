use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use num_rational::Ratio;
type SignedRational = Ratio<i64>;

#[derive(Debug, Clone)]
enum Job {
    Yell(SignedRational),
    Sum([JobStatus; 2]),
    Minus([JobStatus; 2]),
    Mult([JobStatus; 2]),
    Div([JobStatus; 2]),
    Eq([JobStatus; 2]),
}

#[derive(Debug, Clone)]
enum JobStatus {
    Pending,
    WaitingOn(String),
    Finished(SignedRational),
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    job: Job,
    status: JobStatus,
}

fn parse_input(filename: &str) -> Vec<Monkey> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut monkeys = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let [name, op]: [&str; 2] = line.split(':').collect::<Vec<&str>>().try_into().unwrap();
        let (job, status) = if op.contains('+') {
            (
                Job::Sum(
                    op.split('+')
                        .map(|x| JobStatus::WaitingOn(String::from(x.trim())))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                ),
                JobStatus::Pending,
            )
        } else if op.contains('*') {
            (
                Job::Mult(
                    op.split('*')
                        .map(|x| JobStatus::WaitingOn(String::from(x.trim())))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                ),
                JobStatus::Pending,
            )
        } else if op.contains('-') {
            (
                Job::Minus(
                    op.split('-')
                        .map(|x| JobStatus::WaitingOn(String::from(x.trim())))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                ),
                JobStatus::Pending,
            )
        } else if op.contains('/') {
            (
                Job::Div(
                    op.split('/')
                        .map(|x| JobStatus::WaitingOn(String::from(x.trim())))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                ),
                JobStatus::Pending,
            )
        } else {
            (
                Job::Yell(i64::from_str_radix(op.trim(), 10).unwrap().into()),
                JobStatus::Finished(i64::from_str_radix(op.trim(), 10).unwrap().into()),
            )
        };
        monkeys.push(Monkey {
            name: name.to_string(),
            job,
            status,
        });
        line.clear();
    }
    monkeys
}

fn update_status(
    first: &mut JobStatus,
    second: &mut JobStatus,
    monkey_map: &HashMap<String, Monkey>,
) -> [Option<SignedRational>; 2] {
    let first_status = match first {
        JobStatus::WaitingOn(monkey) => {
            if let Some(JobStatus::Finished(res)) = monkey_map.get(monkey).map(|m| &m.status) {
                *first = JobStatus::Finished(*res);
                Some(*res)
            } else {
                None
            }
        }
        JobStatus::Finished(res) => Some(*res),
        _ => None,
    };
    let second_status = match second {
        JobStatus::WaitingOn(monkey) => {
            if let Some(JobStatus::Finished(res)) = monkey_map.get(monkey).map(|m| &m.status) {
                *second = JobStatus::Finished(*res);
                Some(*res)
            } else {
                None
            }
        }
        JobStatus::Finished(res) => Some(*res),
        _ => None,
    };
    [first_status, second_status]
}

fn part_one(filename: &str) {
    let mut monkeys = parse_input(filename);
    let mut monkey_map: HashMap<String, Monkey> = monkeys
        .iter()
        .cloned()
        .map(|monk| (monk.name.clone(), monk))
        .collect();
    reduce_expr(&mut monkeys, &mut monkey_map);
    if let JobStatus::Finished(result) = monkey_map["root"].status {
        println!("Part one: {}", result);
    } else {
        println!("Oops");
    }
}

fn part_two(filename: &str) {
    let mut monkeys = parse_input(filename);

    let root = monkeys
        .iter_mut()
        .find(|monkey| monkey.name.as_str() == "root")
        .unwrap();
    root.job = Job::Eq(match root.job.clone() {
        Job::Sum(deps) => deps,
        Job::Mult(deps) => deps,
        Job::Minus(deps) => deps,
        Job::Div(deps) => deps,
        Job::Eq(deps) => deps,
        Job::Yell(_) => unreachable!(),
    });
    let (ix, _) = monkeys
        .iter()
        .enumerate()
        .find(|monkey| monkey.1.name.as_str() == "humn")
        .unwrap();
    let mut human = monkeys.remove(ix);
    let mut monkey_map: HashMap<String, Monkey> = monkeys
        .iter()
        .cloned()
        .map(|monk| (monk.name.clone(), monk))
        .collect();

    reduce_expr(&mut monkeys, &mut monkey_map);
    human.job = Job::Yell(0.into());
    human.status = JobStatus::Finished(0.into());
    monkeys.push(human.clone());
    monkey_map.insert(human.name.clone(), human);
    let factors = get_factorized_const(monkeys.clone(), monkey_map.clone());
    let mut tried = HashSet::new();
    for set in pow_set(&factors) {
        let val: i64 = set.into_iter().product::<u64>() as i64;
        if !tried.insert(val) {
            continue;
        }
        let mut test_monkeys = monkeys.clone();
        let mut test_map = monkey_map.clone();
        test_monkeys.last_mut().map(|human| {
            human.job = Job::Yell(val.into());
            human.status = JobStatus::Finished(val.into());
            test_map
                .get_mut("humn")
                .map(|h| h.job = Job::Yell(val.into()));
            test_map
                .get_mut("humn")
                .map(|h| h.status = JobStatus::Finished(val.into()));
        });
        reduce_expr(&mut test_monkeys, &mut test_map);
        let const_term = if let JobStatus::Finished(res) = test_map["root"].status {
            *res.reduced().numer() as u64
        } else {
            unreachable!()
        };
        if const_term == 1 {
            println!("Part two: {}", val);
            return;
        }
    }
}

fn get_factorized_const(
    mut monkeys: Vec<Monkey>,
    mut monkey_map: HashMap<String, Monkey>,
) -> Vec<u64> {
    reduce_expr(&mut monkeys, &mut monkey_map);
    let const_term = if let Job::Eq([JobStatus::Finished(first), JobStatus::Finished(second)]) =
        monkey_map["root"].job
    {
        if first > second {
            first - second
        } else {
            second - first
        }
    } else {
        unreachable!()
    };
    let const_term = *const_term.reduced().numer() as u64 * 784;
    primes::factors(const_term)
}

fn pow_set<T>(s: &[T]) -> Vec<Vec<T>>
where
    T: Copy,
{
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| *element)
                .collect()
        })
        .collect()
}

fn reduce_expr(monkeys: &mut Vec<Monkey>, monkey_map: &mut HashMap<String, Monkey>) {
    let mut completed = HashSet::new();
    let mut finished;
    let mut reduced;
    loop {
        finished = true;
        reduced = true;
        for monkey in monkeys.iter_mut() {
            if let JobStatus::Pending = monkey.status {
                finished = false;
                match &mut monkey.job {
                    Job::Sum([first, second]) => {
                        if let [Some(res1), Some(res2)] = update_status(first, second, &monkey_map)
                        {
                            monkey.status = JobStatus::Finished(res1 + res2);
                            reduced = false;
                        }
                    }
                    Job::Minus([first, second]) => {
                        if let [Some(res1), Some(res2)] = update_status(first, second, &monkey_map)
                        {
                            monkey.status = JobStatus::Finished(res1 - res2);
                            reduced = false;
                        }
                    }
                    Job::Mult([first, second]) => {
                        if let [Some(res1), Some(res2)] = update_status(first, second, &monkey_map)
                        {
                            monkey.status = JobStatus::Finished(res1 * res2);
                            reduced = false;
                        }
                    }
                    Job::Div([first, second]) => {
                        if let [Some(res1), Some(res2)] = update_status(first, second, &monkey_map)
                        {
                            monkey.status = JobStatus::Finished(res1 / res2);
                            reduced = false;
                        }
                    }
                    Job::Eq([first, second]) => {
                        if let [Some(res1), Some(res2)] = update_status(first, second, &monkey_map)
                        {
                            monkey.status = JobStatus::Finished(
                                if res1.reduced() == res2.reduced() {
                                    1
                                } else {
                                    0
                                }
                                .into(),
                            );
                            reduced = false;
                        }
                    }
                    _ => {}
                }

                if let JobStatus::Finished(_) = monkey.status {
                    completed.insert(monkey.name.clone());
                }
                let mapped = monkey_map.get_mut(&monkey.name).unwrap();
                mapped.status = monkey.status.clone();
                mapped.job = monkey.job.clone();
            }
            if let JobStatus::Finished(_) = monkey.status {
                completed.insert(monkey.name.clone());
            }
        }
        if finished || reduced {
            break;
        }
    }
    if !finished {
        completed.remove("root");
        monkeys.retain(|monkey| !completed.contains(&monkey.name));
        monkey_map.retain(|monkey, _| !completed.contains(monkey));
    }
}

fn main() {
    part_one("input.txt");
    part_two("input.txt");
}
