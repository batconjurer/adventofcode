use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Block {
    File(u64),
    Free,
}
#[derive(Debug, Default)]
struct Compactor {
    current_id: u64,
    current_layout: VecDeque<Block>,
}

impl Compactor {
    fn prune_trailing_free_space(&mut self) {
        while let Some(Block::Free) = self.current_layout.back() {
            self.current_layout.pop_back();
        }
    }

    fn display(&self) {
        let s = self.current_layout
            .iter()
            .fold(String::new(), | mut acc, b| {
                match b {
                    Block::File(id) => acc.push_str(&id.to_string()),
                    Block::Free => acc.push('.'),
                 }
                acc
            });
        println!("{}", s);
    }

    fn checksum(&self) -> u64 {
        self.current_layout
            .iter()
            .enumerate()
            .fold(0, |mut acc, (ix, b)| {
                match b {
                    Block::File(id) => {
                        acc += ix as u64 * id;
                    }
                    Block::Free => {unreachable!()}
                }
                acc
            })
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct BlockSpan {
    block: Block,
    len: u64,
}
#[derive(Debug, Default)]
struct DeFrag {
    current_id: u64,
    current_layout: Vec<BlockSpan>,
}

impl DeFrag {

    fn display(&self) {
        let mut str = String::new();
        for span in &self.current_layout {
            for _ in 0..span.len {
                match span.block{
                    Block::File(id) => str.push_str(&id.to_string()),
                    Block::Free => str.push_str("."),
                }
            }
        }
        println!("{str}");
    }

    fn checksum(&self) -> u64 {
        let mut acc = 0;
        let mut ix = 0;
        for span in &self.current_layout {
            let val = match span.block {
                Block::File(id) => id,
                Block::Free => {
                    ix += span.len;
                    continue
                }
            };
            for _ in 0..span.len {
                acc += val * ix;
                ix += 1;
            }
        }
        acc
    }

    fn move_largest(&mut self) {
        let Some((ix, span)) = self.current_layout
            .iter()
            .enumerate()
            .find_map(|(ix, b)| if b.block == Block::File(self.current_id) {
                Some((ix, *b))
            } else {
                None
            })
        else {
            return
        };
        let Some((free_ix,free)) = self.current_layout
            .iter()
            .enumerate()
            .find_map(|(idx, b )| if b.block == Block::Free && b.len >= span.len && ix > idx {
                Some((idx, *b))
            } else {
                None
            })
        else {
            return
        };

        let moved = self.current_layout.get_mut(ix).unwrap();
        let mut span = BlockSpan{block: Block::Free, len: span.len};
        std::mem::swap(moved, &mut span);
        self.current_layout.insert(free_ix, span);
        if free.len == span.len {
            self.current_layout.remove(free_ix + 1);
        } else {
            self.current_layout.get_mut(free_ix + 1).unwrap().len -= span.len;
        }

    }
}

fn part1(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut defragger = Compactor::default();

    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        for (ix, c) in line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
        {
            for _ in 0..c {
                if ix & 1 == 0 {
                    defragger.current_layout.push_back(Block::File(defragger.current_id));
                } else {
                    defragger.current_layout.push_back(Block::Free);
                }
            }
            if ix & 1 == 0 {
                defragger.current_id += 1;
            }
        }
        line.clear();
    }

    let mut idx = 0usize;
    loop {
        defragger.prune_trailing_free_space();
        let maybe_next =defragger.current_layout.get(idx);
        if  maybe_next == Some(&Block::Free) {
            defragger.current_layout.swap_remove_back(idx);
        } else if maybe_next.is_none() {
            break;
        }
        idx += 1;
    }
    println!("Part 1: {}", defragger.checksum());
}

fn part2(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut defragger = DeFrag::default();

    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        for (ix, c) in line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .enumerate()
        {
            defragger.current_layout.push(BlockSpan{
                block: if ix & 1 == 0 {
                    Block::File(defragger.current_id)
                } else {
                    Block::Free
                },
                len: c,
            });
            if ix & 1 == 0 {
                defragger.current_id += 1;
            }
        }
        line.clear();
    }
    defragger.current_id -= 1;
    loop {
        defragger.move_largest();
        if defragger.current_id == 0 {
            break;
        } else {
            defragger.current_id -= 1;
        }
    }
    println!("Part 2: {:?}", defragger.checksum());
}

fn main() {
    part1("input.txt");
    part2("input.txt");
}
