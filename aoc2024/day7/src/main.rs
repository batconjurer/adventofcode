use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    Mult,
    Add,
    Concat,
    None,
    First,
}

#[derive(Copy, Clone, Debug)]
struct Part {
    val: u64,
    op: Op,
}

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: VecDeque<u64>,
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn eval(parts: &[Part]) -> u64 {
    let mut acc = parts.first().unwrap().val;
    for part in parts.iter().skip(1) {
        match part.op {
            Op::Mult => {
                acc = acc * part.val;
            }
            Op::Add => {
                acc += part.val;
            }
            Op::Concat => {
                acc = concat(acc, part.val);
            }
            Op::None => unreachable!(),
            Op::First => unreachable!(),
        }
    }
    acc
}

fn has_sat(result: u64, operands: &mut Vec<Part>, allow_concat: bool) -> bool {
    let next = operands
        .iter_mut()
        .enumerate()
        .find(|p| p.1.op == Op::None);
    if next.is_none() {
        result == eval(operands)
    } else {
        let (ix, next) = next.unwrap();
        next.op = Op::Add;
        if has_sat(result, operands, allow_concat) {
            return true;
        }
        operands.get_mut(ix).unwrap().op = Op::Mult;
        if has_sat(result, operands, allow_concat) {
            return true;
        } else if !allow_concat {
            operands.get_mut(ix).unwrap().op = Op::None;
            return false;
        }
        operands.get_mut(ix).unwrap().op = Op::Concat;
        if !has_sat(result, operands, allow_concat) {
            operands.get_mut(ix).unwrap().op = Op::None;
            false
        } else {
            true
        }
    }
}

fn parse_file(filename: &str) -> Vec<Equation> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut equations = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }

        let mut parts = line.trim().split(':');
        let result = parts.next().unwrap().parse().unwrap();
        let operands = parts
            .next()
            .unwrap()
            .trim();
        let operands = operands
            .split(' ')
            .map(|op| u64::from_str(op).unwrap())
            .collect();
        equations.push(Equation{
            result,
            operands
        });
        line.clear();
    }
    equations
}


fn find_sat_sum(filename: &str, allow_concat: bool) -> u64 {
    let eqs = parse_file(filename);
    eqs.into_iter()
        .fold(
            0u64,
            |acc, eqn| {
                let (result, ops) = (eqn.result, eqn.operands);
                let mut ops = ops
                    .into_iter()
                    .map(|op| Part{ val: op, op: Op::None })
                    .collect::<Vec<_>>();
                ops.first_mut().map(|p| p.op = Op::First);
                if has_sat(result, &mut ops, allow_concat) {
                    acc + result
                } else {
                    acc
                }
            })
}

fn part_1(filename: &str) {
    let sum = find_sat_sum(filename, false);
    println!("Part 1: {sum}");
}

fn part_2(filename: &str) {
    let sum = find_sat_sum(filename, true);
    println!("Part 2: {sum}");
}


fn main() {
    part_1("input.txt");
    part_2("input.txt");
}
