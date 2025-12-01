use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
enum Gate {
    XOR,
    OR,
    AND
}

impl Gate {
    fn op(&self, op1: bool, op2: bool) -> bool {
        match self {
            Gate::XOR => op1 != op2,
            Gate::OR => op1 || op2,
            Gate::AND => op1 && op2,
        }
    }

    fn from(s: &str) -> Self {
        match s {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => unreachable!()
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Circuit {
    inputs: HashMap<String, bool>,
    operations: VecDeque<(String, String, Gate, String)>
}

impl Circuit {
    fn swap(&mut self, first: &str, second: &str) {
        for (_, _, _, out) in self.operations.iter_mut() {
            if *out == first {
                *out = second.to_string();
            } else if *out == second {
                *out = first.to_string();
            }
        }
    }

    fn get_input(&self) -> [u64; 2] {
        let mut x = 0;
        let mut y = 0;
        for (i, val) in &self.inputs {
            if i.starts_with('x') {
                let reg = get_register(&i);
                x += if *val { 2u64.pow(reg as u32) } else { 0 };
            } else {
                let reg = get_register(&i);
                y += if *val { 2u64.pow(reg as u32) } else { 0 };
            }
        }
        [x, y]
    }
}

fn parse_file(filename: &str) -> Circuit  {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut circuit = Circuit::default();
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        if line.len() <= 7 {
            let mut parts = line.trim().split(':');
            let input_wire = parts.next().unwrap();
            let value = match parts.next().unwrap().trim() {
                "1" => true,
                "0" => false,
                _ => unreachable!()
            };
            circuit.inputs.insert(input_wire.to_string(), value);
        } else {
            let mut parts = line.trim().split(' ');
            let wire1 = parts.next().unwrap().to_string();
            let op = Gate::from(parts.next().unwrap());
            let wire2 = parts.next().unwrap().to_string();
            let _ = parts.next().unwrap();
            let output = parts.next().unwrap().to_string();
            circuit.operations.push_back((wire1, wire2, op, output));
        }

        line.clear();
    }
    circuit

}

fn eval(mut circuit: Circuit) -> HashMap<String, bool> {
    let mut evaluated = std::mem::take(&mut circuit.inputs);
    while let Some((in1, in2, op, out)) = circuit.operations.pop_front() {
        if let Some((val1, val2)) = evaluated.get(&in1).zip(evaluated.get(&in2)) {
            evaluated.insert(out, op.op(*val1, *val2));
        } else {
            circuit.operations.push_back((in1, in2, op, out))
        }
    }
    evaluated
}


fn part1(filename: &str) {
    let circuit = parse_file(filename);
    let mut values = eval(circuit)
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect::<Vec<_>>();
    values.sort();
    let mut total = 0u64;

    for (ix, (_, val)) in values.into_iter().enumerate() {
        let val = if val { 1 } else { 0 };
        let val = val << ix;
        total += val;

    }
    println!("Part 1: {total}");
}

fn get_register(reg: &str) -> u8 {
    if let Some(r) = reg.strip_prefix('x') {
        u8::from_str_radix(r, 10).unwrap()
    } else if let Some(r) = reg.strip_prefix('y') {
        u8::from_str_radix(r, 10).unwrap()
    } else if let Some(r) = reg.strip_prefix('z') {
        u8::from_str_radix(r, 10).unwrap()
    } else {
        unreachable!()
    }
}

fn part2() {
    for s in 0..45 {
        part2_aux(s);
    }
    let mut password_parts = vec!["z16", "qkf", "z24", "tgr", "cph", "jqn","kwb", "z12"];
    password_parts.sort();
    let mut password = String::new();
    for part in password_parts {
        password.push_str(part);
        password.push(',');
    }
    password.pop();
    println!("Part 2: {password}");
}

fn part2_aux(shifts: u32) {
    let mut circuit = parse_file("input.txt");
    let [mut x_input, mut y_input] =  circuit.get_input();
    x_input &= 2u64.pow(shifts) - 1;
    y_input &= 2u64.pow(shifts) - 1;

    circuit.swap("z16", "qkf");
    circuit.swap("z24", "tgr");
    circuit.swap("cph", "jqn");
    circuit.swap("kwb", "z12");

    for (reg, val) in circuit.inputs.iter_mut() {
        if get_register(reg) >= shifts as u8 {
            *val = false
        }
    }

    let mut values = eval(circuit)
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect::<Vec<_>>();
    values.sort();
    let mut total = 0u64;

    for (ix, (_, val)) in values.into_iter().enumerate() {
        let val = if val { 1 } else { 0 };
        let val = val << ix;
        total += val;
    }

    assert_eq!(x_input + y_input, total);

}

fn main() {
    part1("input.txt");
    part2();
}