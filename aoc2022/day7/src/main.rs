use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// The different commands for changing
/// directories
#[derive(Debug)]
enum ChangeDir<'a> {
    /// Go up one level
    Up,
    /// Go down one to level to directory of
    /// provided name
    Down(&'a str)
}

/// A terminal command
#[derive(Debug)]
enum Command<'a> {
    /// Change directory
    CD(ChangeDir<'a>),
    /// List contents of directory
    LS,
}

impl<'a> Command<'a> {
    fn parse(subs: Vec<&'a str>) -> Self {
        if subs[1] == "ls" {
            Self::LS
        } else {
            match subs[2] {
                ".." => Self::CD(ChangeDir::Up),
                dir => Self::CD(ChangeDir::Down(dir))
            }
        }
    }
}

/// The results of calling `ls`
#[derive(Debug)]
enum Content {
    /// A directory with a name
    Dir(String),
    /// A file (whose name doesn't matter) with a size
    File(u64),
}

impl Content {
    fn parse(subs: &[&str]) -> Self {
        if subs[0] == "dir" {
            Self::Dir(subs[1].to_string())
        } else {
            Self::File(u64::from_str_radix(subs[0], 10).unwrap())
        }
    }
}

/// A parsed view of a line on the screen
#[derive(Debug)]
enum ParsedLine<'a> {
    /// A command
    Command(Command<'a>),
    /// The output from a command
    Content(Content),
}

impl<'a> ParsedLine<'a> {
    fn parse(subs: Vec<&'a str>) -> Self {
        if subs[0] == "$" {
            ParsedLine::Command(Command::parse(subs))
        } else {
            ParsedLine::Content(Content::parse(&subs))
        }
    }
}

/// A stateful accumulator
#[derive(Debug, Default, Clone)]
struct DirContents {
    current_dir: Vec<String>,
    sizes: HashMap<String, u64>,
}

fn parse_line(accumulator: &mut DirContents, line: &str) {
    let subs: Vec<_> = line.split_ascii_whitespace().collect();
    match ParsedLine::parse(subs) {
        ParsedLine::Command(cmd) => {
            match cmd {
                Command::LS => return,
                Command::CD(cd) => {
                    match cd {
                        ChangeDir::Up => _ = accumulator.current_dir.pop(),
                        ChangeDir::Down(dir) => accumulator.current_dir.push(format!("{}/", dir)),
                    }
                }
            }
        }
        ParsedLine::Content(cnt) => {
            match cnt {
                Content::Dir(_) => return,
                Content::File(size) => {
                    for i in 0..accumulator.current_dir.len() {
                        accumulator
                            .sizes
                            .entry(accumulator.current_dir[0..=i].concat())
                            .and_modify(|old_size| *old_size += size)
                            .or_insert(size);
                    }
                },
            }
        }
    }
}


fn parse_subdir(file_name: &str) -> Result<DirContents, std::io::Error> {
    // open target file
    let file = File::open(&file_name)?;

    // uses a reader buffer
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut accumulator = DirContents::default();
    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                // EOF: save last file address to restart from this address for next run
                if bytes_read == 0 {
                    break;
                }
                parse_line(&mut accumulator, &line);
                // do not accumulate data
                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
    Ok(accumulator)
}

fn part_one(file_name: &str) -> u64 {
    let accumulator = parse_subdir(file_name).unwrap();
    accumulator
        .sizes
        .values()
        .filter(|size| size <= &&100000)
        .sum()
}

fn part_two(file_name: &str) -> u64 {
    let accumulator = parse_subdir(file_name).unwrap();
    let unused_space = 70000000 - accumulator.sizes["//"];
    *accumulator
        .sizes
        .values()
        .filter(|size| unused_space + **size >= 30000000)
        .min()
        .unwrap()
}

fn main() {
    let contents = part_one("input.txt");
    println!("{:?}", contents);
    println!("{}", part_two("input.txt"))
}
