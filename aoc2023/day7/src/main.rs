use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod part_two;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum CardType {
    Two = 0,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace
}

impl From<char> for CardType {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            other => panic!("Unrecognized card char: {}", other)
        }
    }
}

impl PartialOrd for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((*self as u8).cmp(&(*other as u8)))
    }
}

impl Ord for CardType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HandType {
    High = 0,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((*self as u8).cmp(&(*other as u8)))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand {
    r#type: HandType,
    cards: [CardType; 5],
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [CardType; 5] = s.chars().map(CardType::from).collect::<Vec<_>>().try_into().unwrap();
        let card_count: HashMap<_, usize> = cards.iter().fold(HashMap::new(),  |mut acc, c| {
            acc.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
            acc
        });
        let mut counts: Vec<_> = card_count.values().cloned().collect();
        counts.sort();
        let hand_type = if counts.len() == 1 {
            HandType::FiveKind
        } else if counts == vec![1, 4,] {
            HandType::FourKind
        } else if counts == vec![2, 3, ] {
            HandType::FullHouse
        } else if counts == vec![1, 1, 3] {
            HandType::ThreeKind
        } else if counts == vec![1, 2, 2] {
            HandType::TwoPair
        } else if counts == vec![1, 1, 1, 2] {
            HandType::OnePair
        } else {
            HandType::High
        };
        Ok(Self {
            r#type: hand_type,
            cards,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.r#type > other.r#type {
            Some(Ordering::Greater)
        } else if self.r#type < other.r#type {
            Some(Ordering::Less)
        } else {
            if self.cards == other.cards {
                return Some(Ordering::Equal)
            }
            for ix in 0usize..5 {
                if self.cards[ix] > other.cards[ix] {
                    return Some(Ordering::Greater)
                } else if self.cards[ix] < other.cards[ix] {
                    return Some(Ordering::Less)
                }
            }
            None
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part_one(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut hands = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let [hand, val]: [&str; 2] = line.split(' ').collect::<Vec<_>>().try_into().unwrap();
        let hand = Hand::from_str(hand).unwrap();
        let val = u64::from_str(val.trim()).unwrap();
        hands.push((hand, val));
        line.clear();
    }
    hands.sort_by_key(|(hand, _)| *hand );
    let result = hands.iter().enumerate().map(|(ix, (_, val))| (ix as u64 + 1) * (*val)).sum::<u64>();
    println!("Part one: {}", result);
}

fn main() {
    part_one("input.txt");
    part_two::part_two("input.txt");
}
