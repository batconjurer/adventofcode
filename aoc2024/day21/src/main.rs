use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

fn numeric(code: &str) -> u64 {
    u64::from_str_radix(&code[..3], 10).unwrap()
}

fn paths(start: char, end: char) -> impl Iterator<Item=&'static str> {
    match (start, end) {
        (f, s) if f == s => [Some("A"), None, None, None, None, None, None, None, None],
        ('7', '8') | ('8', '9') | ('4', '5') | ('5', '6') | ('1', '2') | ('2', '3') | ('0', 'A') => [Some(">A"), None, None, None, None, None, None, None, None],
        ('8', '7') | ('9', '8') | ('5', '4') | ('6', '5') | ('2', '1') | ('3', '2') | ('A', '0') => [Some("<A"), None, None, None, None, None, None, None, None],
        ('1', '4') | ('4', '7') | ('0', '2') | ('2', '5') | ('5', '8') | ('A', '3') | ('3', '6') | ('6', '9') => [Some("^A"), None, None, None, None, None, None, None, None],
        ('4', '1') | ('7', '4') | ('2', '0') | ('5', '2') | ('8', '5') | ('3', 'A') | ('6', '3') | ('9', '6') => [Some("vA"), None, None, None, None, None, None, None, None],
        ('1', '7') | ('0', '5') | ('2', '8') | ('A', '6') | ('3', '9') => [Some("^^A"), None, None, None, None, None, None, None, None],
        ('7', '1') | ('5', '0') | ('8', '2') | ('6', 'A') | ('9', '3') => [Some("vvA"), None, None, None, None, None, None, None, None],
        ('7', '9') | ('4', '6') | ('1', '3') => [Some(">>A"), None, None, None, None, None, None, None, None],
        ('9', '7') | ('6', '4') | ('3', '1') => [Some("<<A"), None, None, None, None, None, None, None, None],
        ('7', '5') | ('8', '6') | ('4', '2') | ('5', '3') | ('2', 'A') => [Some(">vA"), Some("v>A"), None, None, None, None, None, None, None],
        ('5', '7') | ('6', '8') | ('2', '4') | ('3', '5') | ('A', '2') => [Some("<^A"), Some("^<A"), None, None, None, None, None, None, None],
        ('4', '8') | ('5', '9') | ('1', '5') | ('2', '6') | ('0', '3') => [Some("^>A"), Some(">^A"), None, None, None, None, None, None, None],
        ('8', '4') | ('9', '5') | ('5', '1') | ('6', '2') | ('3', '0') => [Some("v<A"), Some("<vA"), None, None, None, None, None, None, None],
        ('1', '0') => [Some(">vA"), None, None, None, None, None, None, None, None],
        ('0', '1') => [Some("^<A"), None, None, None, None, None, None, None, None],
        ('8', '0') | ('9', 'A') => [Some("vvvA"), None, None, None, None, None, None, None, None],
        ('0', '8') | ('A', '9') => [Some("^^^A"), None, None, None, None, None, None, None, None],
        ('7', '6') | ('4', '3') => [Some(">>vA"), Some(">v>A"), Some("v>>A"), None, None, None, None, None, None],
        ('6', '7') | ('3', '4') => [Some("^<<A"), Some("<^<A"), Some("<<^A"), None, None, None, None, None, None],
        ('1', 'A') => [Some(">v>A"), Some(">>vA"), None, None, None, None, None, None, None],
        ('A', '1') => [Some("^<<A"), Some("<^<A"), None, None, None, None, None, None, None],
        ('4', '9') | ('1', '6') => [Some(">>^A"), Some(">^>A"), Some("^>>A"), None, None, None, None, None, None],
        ('9', '4') | ('6', '1') => [Some("v<<A"), Some("<v<A"), Some("<<vA"), None, None, None, None, None, None],
        ('1', '8') | ('2', '9')| ('0', '6') => [Some("^^>A"), Some("^>^A"), Some(">^^A"), None, None, None, None, None, None],
        ('8', '1') | ('9', '2') | ('6', '0') => [Some("vv<A"), Some("v<vA"), Some("<vvA"), None, None, None, None, None, None],
        ('7', '2') | ('8', '3') | ('5', 'A') => [Some(">vvA"), Some("v>vA"), Some("vv>A"), None, None, None, None, None, None],
        ('2', '7') | ('3', '8') | ('A', '5') => [Some("<^^A"), Some("^<^A"), Some("^^<A"), None, None, None, None, None, None],
        ('4', '0')  => [Some("v>vA"), Some(">vvA"), None, None, None, None, None, None, None],
        ('0', '4') => [Some("^^<A"), Some("^<^A"), None, None, None, None, None, None, None],
        ('A', '7') => [Some("^^^<<A"), Some("^^<^<A"), Some("^<^^<A"), Some("<^^^<A"), Some("<^^<^A"), Some("<^<^^A"), Some("^^<<^A"), Some("^<^<^A"), Some("^<<^^A")],
        ('7', 'A') => [Some(">>vvvA"), Some(">v>vvA"), Some(">vv>vA"), Some(">vvv>A"), Some("v>vv>A"), Some("vv>v>A"), Some("v>>vvA"), Some("v>v>vA"), Some("vv>>vA")],
        ('A', '4') => [Some("<^<^A"), Some("<^^<A"), Some("^<<^A"), Some("^<^<A"), Some("^^<<A"), None, None, None, None],
        ('4', 'A') => [Some(">v>vA"), Some(">vv>A"), Some("v>>vA"), Some("v>v>A"), Some(">>vvA"), None, None, None, None],
        ('7', '3') => [Some(">v>vA"), Some(">vv>A"), Some("v>>vA"), Some("v>v>A"), Some("vv>>A"), Some(">>vvA"), None, None, None],
        ('3', '7') => [Some("<^<^A"), Some("<^^<A"), Some("^<<^A"), Some("^<^<A"), Some("^^<<A"), Some("<<^^A"), None, None, None],
        ('1', '9') => [Some(">^>^A"), Some(">^^>A"), Some("^>>^A"), Some("^>^>A"), Some("^^>>A"), Some(">>^^A"), None, None, None],
        ('9', '1') => [Some("<v<vA"), Some("<vv<A"), Some("v<<vA"), Some("v<v<A"), Some("vv<<A"), Some("<<vvA"), None, None, None],
        ('0', '9') => [Some("^^^>A"), Some("^^>^A"), Some("^>^^A"), Some(">^^^A"), None, None, None, None, None],
        ('A', '8') => [Some("^^^<A"), Some("^^<^A"), Some("^<^^A"), Some("<^^^A"), None, None, None, None, None],
        ('9', '0') => [Some("vvv<A"), Some("vv<vA"), Some("v<vvA"), Some("<vvvA"), None, None, None, None, None],
        ('8', 'A') => [Some("vvv>A"), Some("vv>vA"), Some("v>vvA"), Some(">vvvA"), None, None, None, None, None],
        ('7', '0') => [Some("vv>vA"), Some("v>vvA"), Some(">vvvA"), None, None, None, None, None, None],
        ('0', '7') => [Some("^^^<A"), Some("^^<^A"), Some("^<^^A"), None, None, None, None, None, None],
        _ => unreachable!(),
    }.into_iter().filter_map(|x| x)
}

fn get_expansions(pair: &[char; 2]) -> impl Iterator<Item=&'static str> {
    match pair {
        ['^', '^'] => [Some("A"), None],
        ['^', '>'] => [Some(">vA"), Some("v>A")],
        ['^', '<'] => [Some("v<A"), None],
        ['v', 'v'] => [Some("A"), None],
        ['v', '<'] => [Some("<A"), None],
        ['v', '>'] => [Some(">A"), None],
        ['>', '>'] => [Some("A"), None],
        ['>', 'v'] => [Some("<A"), None],
        ['>', '^'] => [Some("^<A"), Some("<^A")],
        ['<', '^'] => [Some(">^A"), None],
        ['<', 'v'] => [Some(">A"), None],
        ['<', '<'] => [Some("A"), None],
        ['A', 'A'] => [Some("A"), None],
        ['A', 'v'] => [Some("<vA"), Some("v<A")],
        ['A', '^'] => [Some("<A"), None],
        ['A', '>'] => [Some("vA"),  None],
        ['A', '<'] => [Some("<v<A"), Some("v<<A")],
        ['v', 'A'] => [Some(">^A"), Some("^>A")],
        ['^', 'A'] => [Some(">A"), None],
        ['<', 'A'] => [Some(">^>A"), Some(">>^A")],
        ['>', 'A'] => [Some("^A"), None],
        _ => unreachable!()
    }.into_iter().filter_map(|x| x)
}

struct Pairs<'a> {
    char_iter: Peekable<Chars<'a>>,
    first: bool,
}

impl<'a> Pairs<'a> {
    fn new(inner: &'a str) -> Self {
        Self {
            char_iter: inner.chars().peekable(),
            first: true,
        }
    }
}

impl<'a> Iterator for Pairs<'a> {
    type Item = [char; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            let f = self.char_iter.peek()?;
            self.first = false;
            return Some(['A', *f]);
        }
        let f = self.char_iter.next()?;
        if let Some(s) = self.char_iter.peek() {
           Some([f, *s])
        } else {
            None
        }
    }
}

fn find_shortest_expansion(
    instructions: &'static str,
    depth: u8,
    cache: &mut HashMap<(String, u8), u64>
) -> u64 {

    if let Some(exp) = cache.get(&(instructions.to_string(), depth)) {
        return *exp
    }
    if depth == 0 {
        return instructions.len() as u64;
    }

    let best = Pairs::new(instructions).map(|pair|{
        let  best = get_expansions(&pair)
            .map(|p| find_shortest_expansion(p, depth - 1, cache))
            .min()
            .unwrap();
        //println!("Best expansion for {:?} at depth {} is {} with length {}", pair, depth, st, best);
        best

    }).sum();
    //println!("Best expansion for {} is length {}", instructions, best);
    cache.insert((instructions.to_string(), depth ), best);
    best
}


fn expand_code(code: &'static str, cache: &mut HashMap<(String, u8), u64>, depth: u8) -> u64 {
    Pairs::new(code).map(|[f, s]| {
        paths(f, s)
            .map(|x| find_shortest_expansion(x,  depth, cache))
            .min()
            .unwrap()
    }).sum()
}

fn part1(codes: &'static [&'static str]) {
    let mut cache = HashMap::new();
    let mut total = 0;
    for code in codes {
        total += numeric(code) * expand_code(*code, &mut cache, 2);
    }
    println!("Part 1: {total}");
}

fn part2(codes: &'static [&'static str]) {
    let mut cache = HashMap::new();
    let mut total = 0;
    for code in codes {
        total += numeric(code) * expand_code(*code, &mut cache, 25);
    }
    println!("Part 2: {total}");
}

fn main() {
    part1(&["789A", "540A", "285A", "140A", "189A"]);
    part2(&["789A", "540A", "285A", "140A", "189A"]);
}
