use std::collections::HashMap;

pub fn input() -> &'static str {
    include_str!("../../input/day04.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day04.txt")
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Card {
    winning: Vec<usize>,
    have: Vec<usize>,
}

impl Card {
    fn from_line(line: &str) -> Self {
        let (winning, have) = line.split_once(" | ").unwrap();
        Self {
            winning: winning
                .split_whitespace()
                .skip(2)
                .map(|n| n.trim().parse().unwrap())
                .collect(),
            have: have
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let cards: Vec<_> = input.lines().map(Card::from_line).collect();
    let winnings: Vec<_> = cards
        .iter()
        .map(|card| {
            card.have
                .iter()
                .filter(|n| card.winning.contains(n))
                .count()
        })
        .collect();
    let points = winnings
        .iter()
        .filter(|&&exp| exp > 0)
        .map(|&exp| 2usize.pow(exp as u32 - 1))
        .sum();
    let mut copies: HashMap<_, _> = cards.iter().map(|c| (c, 1)).collect();
    for (index, (card, winnings)) in cards.iter().zip(winnings).enumerate() {
        let qty = *copies.get(card).unwrap();
        for duplicate in &cards[index + 1..=index + winnings] {
            if let Some(count) = copies.get_mut(duplicate) {
                *count += qty;
            }
        }
    }
    (points, copies.values().sum())
}

common::test!(day04, (13, 30), (21088, 6874754));
