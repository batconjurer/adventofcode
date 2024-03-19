use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum CardType {
    Joker = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace
}


impl From<u8> for CardType {
    fn from(value: u8) -> Self {
        match value {
            0=> Self::Joker,
            1=> Self::Two,
            2=> Self::Three,
            3=> Self::Four,
            4=> Self::Five,
            5=> Self::Six,
            6=> Self::Seven,
            7=> Self::Eight,
            8=> Self::Nine,
            9=> Self::Ten,
            10=> Self::Queen,
            11=> Self::King,
            12=> Self::Ace,
            _ => unreachable!(),
        }
    }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Card {
    value: CardType,
    is_wild: bool,
}


impl From<CardType> for Card {
    fn from(value: CardType) -> Self {
        Self {
            value,
            is_wild: false,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_wild && !other.is_wild {
            Some(Ordering::Less)
        } else if !self.is_wild && other.is_wild {
            Some(Ordering::Greater)
        } else if self.is_wild && other.is_wild {
            Some(Ordering::Equal)
        } else {
            self.value.partial_cmp(&other.value)
        }
    }
}

impl Ord for Card {
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
    cards: [Card; 5],
}

impl Hand {
    fn optimize(&mut self) {
        if let Some((ix, _)) = self.cards
            .iter()
            .enumerate()
            .find(|(_, c)| c.value == CardType::Joker) {
            *self = (1u8..=12).into_iter().map(|card| {
                let mut cards = self.cards;
                cards[ix] = Card{
                    value: CardType::from(card),
                    is_wild: true
                };
                let mut hand = Self {
                    r#type: Self::get_type(cards),
                    cards,
                };
                hand.optimize();
                hand
            }).max().unwrap();
        }
    }

    fn get_type(cards: [Card; 5]) -> HandType {
        let card_count: HashMap<_, usize> = cards.iter().map(|c| c.value).fold(HashMap::new(),  |mut acc, c| {
            acc.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
            acc
        });
        let mut counts: Vec<_> = card_count.values().cloned().collect();
        counts.sort();
        if counts.len() == 1 {
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
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = s.chars().map(|c| Card::from(CardType::from(c))).collect::<Vec<_>>().try_into().unwrap();
        Ok(Self {
            r#type: Self::get_type(cards),
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
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub(crate) fn part_two(filename: &str) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut hands = vec![];
    while let Ok(length) = reader.read_line(&mut line) {
        if length == 0 {
            break;
        }
        let [hand, val]: [&str; 2] = line.split(' ').collect::<Vec<_>>().try_into().unwrap();
        let mut hand = Hand::from_str(hand).unwrap();
        hand.optimize();
        let val = u64::from_str(val.trim()).unwrap();
        hands.push((hand, val));
        line.clear();
    }
    hands.sort_by_key(|(hand, _)| *hand );
    let result = hands.iter().enumerate().map(|(ix, (_, val))| (ix as u64 + 1) * (*val)).sum::<u64>();
    println!("Part two: {}", result);
}