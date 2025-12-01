use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;

fn part_1(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut sum = 0u64;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let mut stream = line.trim().chars().peekable();
        loop {
            let prod = parse_line(stream.by_ref());
            sum += prod;
            if stream.peek().is_none() {
                break
            }
        }
        line.clear();
    }
    println!("Part 1: {sum}")
}

fn part_2(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut sum = 0u64;
    let mut enable = true;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let mut stream = line.trim().chars().peekable();
        loop {
            let prod = parse_line_with_enabling(stream.by_ref(), &mut enable);
            sum += prod;
            if stream.peek().is_none() {
                break
            }
        }
        line.clear();
    }
    println!("Part 2: {sum}")
}

fn find_start(
    stream: &mut Peekable<impl Iterator<Item=char>>
) -> bool {
    loop {
        match stream.by_ref().peek() {
            None => return false,
            Some(x) if *x =='m' => {
                _ = stream.next();
                break
            },
            _ => {
                _ = stream.next();
            }
        }
    }
    if stream.next_if_eq(&'u').is_none() {
        return false
    }
    if stream.next_if_eq(&'l').is_none() {
        return false
    }
    if stream.next_if_eq(&'(').is_none() {
        return false
    }
    return true
}

fn parse_leading_number(
    stream: &mut Peekable<impl Iterator<Item=char>>,
) -> u64 {
    let mut number = String::new();
    loop {
        if let Some(c) =  stream.next_if(|x| x.is_numeric()) {
            number.push(c)
        } else {
            break
        }
    }
    if let Ok(x) = number.parse::<u64>() {
        x
    } else {
        0
    }
}

fn parse_line(
    stream: &mut Peekable<impl Iterator<Item=char>>
) -> u64 {
    let mut product = 1u64;
    let found = find_start(stream);
    if !found {
        return 0
    }
    let number = parse_leading_number(stream);
    product *= number;
    if stream.next_if_eq(&',').is_none() {
        return 0
    }
    let number = parse_leading_number(stream);
    product *= number;
    if stream.next_if_eq(&')').is_none() {
        return 0
    }
    product
}

enum Keyword {
    Mul,
    Do,
    Dont,
    None,
}

fn find_next_keyword(
    stream: &mut Peekable<impl Iterator<Item=char>>
) -> Keyword {
    loop {
        match stream.by_ref().peek() {
            None => return Keyword::None,
            Some(x) if *x =='m' => {
                _ = stream.next();
                if stream.next_if_eq(&'u').is_none() {
                    return Keyword::None
                }
                if stream.next_if_eq(&'l').is_none() {
                    return Keyword::None
                }
                if stream.next_if_eq(&'(').is_none() {
                    return Keyword::None
                }
                return Keyword::Mul;
            },
            Some(x) if *x == 'd' => {
                _ = stream.next();
                if stream.next_if_eq(&'o').is_none() {
                    return Keyword::None
                }
                if stream.next_if_eq(&'n').is_none() {
                    if stream.next_if_eq(&'(').is_none() {
                        return Keyword::None
                    }
                    if stream.next_if_eq(&')').is_none() {
                        return Keyword::None
                    }
                    return Keyword::Do
                }
                if stream.next_if_eq(&'\'').is_none() {
                    return Keyword::None
                }
                if stream.next_if_eq(&'t').is_none() {
                    return Keyword::None
                }
                if stream.next_if_eq(&'(').is_none() {
                    return Keyword::None
                }
                if stream.next_if_eq(&')').is_none() {
                    return Keyword::None
                }
                return Keyword::Dont
            }
            _ => {
                _ = stream.next();
            }
        }
    }
}

fn parse_line_with_enabling(
    stream: &mut Peekable<impl Iterator<Item=char>>,
    enabled: &mut bool,
) -> u64 {
    let mut product = 1u64;
    match find_next_keyword(stream) {
        Keyword::None => return 0,
        Keyword::Do => {
            *enabled = true;
            return 0;
        },
        Keyword::Dont => {
            *enabled = false;
            return 0;
        },
        Keyword::Mul => {}
    }
    let number = parse_leading_number(stream);
    product *= number;
    if stream.next_if_eq(&',').is_none() {
        return 0
    }
    let number = parse_leading_number(stream);
    product *= number;
    if stream.next_if_eq(&')').is_none() {
        return 0
    }
    if *enabled { product } else { 0 }
}

fn main() {
    part_1("input.txt");
    part_2("input.txt");
}
