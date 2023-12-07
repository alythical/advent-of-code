use std::{cmp::Ordering, collections::HashSet};

pub fn input() -> &'static str {
    include_str!("../../input/day07.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day07.txt")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    FiveOfA = 1,
    FourOfA,
    FullHouse,
    ThreeOfA,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rank {
    Ace = 1,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Rank {
    fn new(value: char, jokers: bool) -> Self {
        match value {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => {
                if jokers {
                    Self::Joker
                } else {
                    Self::Jack
                }
            }
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!(),
        }
    }
}

impl From<&Vec<Rank>> for Kind {
    fn from(value: &Vec<Rank>) -> Self {
        let distinct: HashSet<_> = value.iter().collect();
        let mut counts = distinct
            .iter()
            .map(|c| value.iter().filter(|cc| c == cc).count());
        match distinct.len() {
            5 => Self::HighCard,
            4 => Self::OnePair,
            3 => counts
                .find(|&n| n == 3)
                .map_or(Self::TwoPair, |_| Self::ThreeOfA),
            2 => counts
                .find(|&n| n == 4)
                .map_or(Self::FullHouse, |_| Self::FourOfA),
            1 => Self::FiveOfA,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    hand: Vec<Rank>,
    kind: Kind,
    bid: usize,
}

impl Card {
    fn new(value: &str, jokers: bool) -> Self {
        let (hand, bid) = value.split_once(' ').unwrap();
        let hand = hand.chars().map(|c| Rank::new(c, jokers)).collect();
        Card {
            kind: Kind::from(&hand),
            bid: bid.parse().unwrap(),
            hand,
        }
    }

    fn upgrade(self) -> Self {
        let jokers = self
            .hand
            .iter()
            .filter(|c| matches!(c, Rank::Joker))
            .count();
        Self {
            kind: match jokers {
                0 => self.kind,
                1.. => match self.kind {
                    Kind::FourOfA | Kind::FullHouse => Kind::FiveOfA,
                    Kind::ThreeOfA => Kind::FourOfA,
                    Kind::TwoPair => match jokers {
                        1 => Kind::FullHouse,
                        2 => Kind::FourOfA,
                        _ => panic!(),
                    },
                    Kind::OnePair => Kind::ThreeOfA,
                    Kind::HighCard => Kind::OnePair,
                    _ => self.kind,
                },
            },
            ..self
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.hand.cmp(&other.hand),
            cmp => cmp,
        }
    }
}

fn winnings(cards: &mut [Card]) -> usize {
    cards.sort_by(|a, b| b.cmp(a));
    cards
        .iter()
        .enumerate()
        .map(|(rank, card)| card.bid * (rank + 1))
        .sum()
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut initial: Vec<_> = input.lines().map(|line| Card::new(line, false)).collect();
    let mut jokers: Vec<_> = input
        .lines()
        .map(|line| Card::new(line, true).upgrade())
        .collect();
    (winnings(&mut initial), winnings(&mut jokers))
}

common::test!(day07, (6440, 5905), (250957639, 251515496));
