use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug)]
struct Board {
    inner: Vec<Vec<u64>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn at(&self, (row, col): (usize, usize)) -> Option<u64> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            Some(self.inner[row][col])
        }
    }
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Dir {
    Up, Down, Left, Right
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Partial {
    head: (usize, usize),
    dir: Dir,
    heat_loss: u64,
    length: i8,
    best: Rc<RefCell<Option<u64>>>,
}

impl Partial {

    fn update_best(&self) {
        let current = self.best.borrow_mut().unwrap_or(u64::MAX);
        *self.best.borrow_mut() = Some(std::cmp::min(self.heat_loss, current));
    }
    fn neighbors(&self, board: &Board) -> impl Iterator<Item=Self> {
        let mut ns = [const { None }; 4];
        ns[2] = if self.head.0 != 0 && self.dir != Dir::Down {
            // Go up
            let next_head = (self.head.0 - 1, self.head.1);
            let heat_loss = board.at(next_head).unwrap();
            let mut next = self.clone();
            next.head = next_head;
            next.dir = Dir::Up;
            next.heat_loss += heat_loss;
            if self.dir == next.dir {
                next.length += 1;
            } else {
                next.length = 1;
            }
            if next.length > 3  {
                None
            } else {
                Some(next)
            }
        } else {
            None
        };
        ns[3] = if self.head.1 != 0 && self.dir != Dir::Right {
            // Go left
            let next_head = (self.head.0, self.head.1 - 1);
            let heat_loss = board.at(next_head).unwrap();
            let mut next = self.clone();
            next.head = next_head;
            next.dir = Dir::Left;
            next.heat_loss += heat_loss;
            if self.dir == next.dir {
                next.length += 1;
            } else {
                next.length = 1;
            }
            if next.length > 3  {
                None
            } else {
                Some(next)
            }
        }  else {
            None
        };
        ns[0] = if self.dir == Dir::Up  {
            None
        } else if let Some(heat_loss) = board.at((self.head.0 + 1, self.head.1)) {
            // Go down
            let mut next = self.clone();
            next.head = (self.head.0 + 1, self.head.1);
            next.dir = Dir::Down;
            next.heat_loss += heat_loss;
            if self.dir == next.dir {
                next.length += 1;
            } else {
                next.length = 1;
            }
            if next.length > 3   {
                None
            } else {
                Some(next)
            }
        } else {
            None
        };
        ns[1] = if self.dir == Dir::Left {
            None
        } else if let Some(heat_loss) = board.at((self.head.0, self.head.1 + 1)) {
            // Go right
            let mut next = self.clone();
            next.head = (self.head.0, self.head.1 + 1);
            next.dir = Dir::Right;
            next.heat_loss += heat_loss;
            if self.dir == next.dir {
                next.length += 1;
            } else {
                next.length = 1;
            }
            if next.length > 3  {
                None
            } else {
                Some(next)
            }
        } else {
            None
        };

        ns.into_iter().filter_map(|x| x)
    }
}

fn parse(filename: &str) -> Board {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut board = vec![];
    let mut cols = 0;
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let row = line.trim()
            .chars()
            .map(|c| u64::from_str(&c.to_string()).unwrap() )
            .collect::<Vec<_>>();
        cols = row.len();
        board.push(row);
        line.clear()
    }
    Board {
        rows: board.len(),
        cols,
        inner: board,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Return {
    BadPartial,
    BestDist(u64),
}

impl PartialOrd for Return {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Return::BadPartial, Return::BestDist(_)) => Some(Ordering::Greater),
            (Return::BestDist(_), Return::BadPartial) => Some(Ordering::Less),
            (Return::BestDist(a), Return::BestDist(b)) => a.partial_cmp(b),
            _ => Some(Ordering::Equal)
        }
    }
}

impl Ord for Return {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn find_path(
    mut partial: Partial,
    board: &Board,
    cache: &mut HashMap<((usize, usize), Dir, i8), u64>,
) -> Return {
    let heat_loss = board.at(partial.head).unwrap();
    if partial.head == (1, 4) && partial.dir == Dir::Right && partial.length == 2 {
        println!("HEY");
    }
    if partial.heat_loss > partial.best.borrow().unwrap_or(u64::MAX) {
        return Return::BadPartial;
    }

    if partial.head == (board.rows - 1, board.cols - 1) {
        partial.update_best();
        return Return::BestDist(heat_loss)
    }
    if let Some(x) = cache.get(&(partial.head, partial.dir, partial.length)) {
        partial.heat_loss = (partial.heat_loss - heat_loss).saturating_add(*x);
        partial.update_best();
        return Return::BestDist(*x);
    }

    let res = partial.neighbors(board)
        .map(|next|{
            find_path(next, board, cache)
        })
        .min()
        .unwrap_or(Return::BadPartial);

    if let Return::BestDist(d) = res {
        cache.insert((partial.head, partial.dir, partial.length), d.saturating_add(heat_loss));
        Return::BestDist(d.saturating_add(heat_loss))
    } else {
        Return::BadPartial
    }

}

fn part_one(filename: &str) {
    let board = parse(filename);
    let mut cache = HashMap::new();
    let best = Rc::new(RefCell::new(None));
    let inits = [
        Partial {
            head: (0, 0),
            dir: Dir::Right,
            heat_loss: 0,
            length: 1,
            best: best.clone(),
        },
       /* Partial {
            head: (0, 0),
            dir: Dir::Down,
            heat_loss: 0,
            length: 1,
            best: best.clone(),
        },*/
    ];
    let Return::BestDist(min_heat) = inits
        .into_iter()
        .map(|p| find_path(p, &board, &mut cache))
        .min()
        .unwrap() else {
        unreachable!()
    };
    for e in cache {
        println!("{:?}", e);
    }
    println!("Part one: {:?}", min_heat);
}

fn main() {
    part_one("test.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic1() {
        let board = Board {
            inner: vec![vec![2, 1, 4], vec![3, 2, 1]],
            rows: 2,
            cols: 3,
        };
        let mut cache = HashMap::new();
        let best = Rc::new(RefCell::new(None));
        let inits = [
            Partial {
                head: (0, 0),
                dir: Dir::Right,
                heat_loss: 2,
                length: 1,
                best: best.clone(),
            },
            Partial {
                head: (0, 0),
                dir: Dir::Down,
                heat_loss: 2,
                length: 1,
                best: best.clone(),
            },
        ];
        let min_heat = inits
            .into_iter()
            .map(|p| find_path(p, &board, &mut cache))
            .min()
            .unwrap();
        assert_eq!(min_heat, Return::BestDist(6));
    }

    #[test]
    fn basic2() {
        let board = Board {
            inner: vec![vec![5, 3], vec![8, 7], vec![5, 3], vec![6, 3], vec![3, 5], vec![3, 3]],
            rows: 6,
            cols: 2,
        };
        let mut cache = HashMap::new();
        let best = Rc::new(RefCell::new(None));
        let inits = [
            Partial {
                head: (0, 0),
                dir: Dir::Right,
                heat_loss: 5,
                length: 1,
                best: best.clone(),
            },
            Partial {
                head: (0, 0),
                dir: Dir::Down,
                heat_loss: 5,
                length: 1,
                best: best.clone(),
            },
        ];
        let min_heat = inits
            .into_iter()
            .map(|p| find_path(p, &board, &mut cache))
            .min()
            .unwrap();
        assert_eq!(min_heat, Return::BestDist(32));
    }
    #[test]
    fn basic3() {
        let board = Board {
            inner: vec![vec![2, 4, 1, 3, 4, 3, 2, 3, 1], vec![3, 2, 1, 5, 4, 5, 3, 5, 3]],
            rows: 2,
            cols: 9,
        };
        let mut cache = HashMap::new();
        let best = Rc::new(RefCell::new(None));
        let inits = [
            Partial {
                head: (0, 0),
                dir: Dir::Right,
                heat_loss: 2,
                length: 1,
                best: best.clone(),
            },
            Partial {
                head: (0, 0),
                dir: Dir::Down,
                heat_loss: 2,
                length: 1,
                best: best.clone(),
            },
        ];
        let min_heat = inits
            .into_iter()
            .map(|p| find_path(p, &board, &mut cache))
            .min()
            .unwrap();
        assert_eq!(min_heat, Return::BestDist(34));
    }
}