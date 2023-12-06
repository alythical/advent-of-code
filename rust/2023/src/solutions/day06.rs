use std::{
    iter::{Skip, Zip},
    str::SplitWhitespace,
};

pub fn input() -> &'static str {
    include_str!("../../input/day06.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day06.txt")
}

struct Run {
    time: usize,
    best: usize,
}

impl From<(&str, &str)> for Run {
    fn from((time, best): (&str, &str)) -> Self {
        Self {
            time: time.parse().unwrap(),
            best: best.parse().unwrap(),
        }
    }
}

impl From<(String, String)> for Run {
    fn from(value: (String, String)) -> Self {
        Self::from((value.0.as_str(), value.1.as_str()))
    }
}

impl Run {
    fn ways(&self) -> usize {
        (1..=self.time - 1)
            .map(move |duration| duration * (self.time - duration))
            .filter(|&time| time > self.best)
            .count()
    }
}

fn merge(input: &str) -> Skip<Zip<SplitWhitespace<'_>, SplitWhitespace<'_>>> {
    let mut lines = input.lines();
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .zip(lines.next().unwrap().split_whitespace())
        .skip(1)
}

fn kerning(input: &str) -> (String, String) {
    merge(input).fold((String::new(), String::new()), |acc, (time, distance)| {
        (acc.0 + time, acc.1 + distance)
    })
}

pub fn solve(input: &str) -> (usize, usize) {
    let product = merge(input).map(Run::from).map(|run| run.ways()).product();
    let run = Run::from(kerning(input));
    (product, run.ways())
}

common::test!(day06, (288, 71503), (625968, 43663323));
