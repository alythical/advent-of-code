use std::collections::HashMap;

use common::num::integer::lcm;

pub fn input() -> &'static str {
    include_str!("../../input/day08.txt")
}

pub fn test_input_p1() -> &'static str {
    include_str!("../../input/tests/day08.txt")
}

pub fn test_input_p2() -> &'static str {
    include_str!("../../input/tests/day08.2.txt")
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

struct DesertStorm<'a> {
    instrs: Vec<Direction>,
    graph: HashMap<&'a str, Node<'a>>,
}

impl<'a> DesertStorm<'a> {
    fn new(input: &'a str) -> Self {
        let (instrs, rest) = input.split_once("\n\n").unwrap();
        let instrs = instrs
            .chars()
            .map(|char| match char {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("invalid input"),
            })
            .collect();
        let graph = rest
            .lines()
            .map(|line| {
                let (src, dst) = line.split_once(" = ").unwrap();
                let (left, right) = dst.split_once(", ").unwrap();
                let (_, left) = left.split_once('(').unwrap();
                let (right, _) = right.split_once(')').unwrap();
                (src, Node { left, right })
            })
            .collect();
        Self { instrs, graph }
    }

    fn with_single_start(&self, start: &str) -> usize {
        let mut cur = start;
        self.instrs
            .iter()
            .cycle()
            .take_while(|instr| {
                let node = self.graph.get(cur).unwrap();
                cur = match instr {
                    Direction::Left => node.left,
                    Direction::Right => node.right,
                };
                cur != "ZZZ"
            })
            .count()
            + 1
    }

    fn with_multiple_starts(&self, mut starts: Vec<String>) -> usize {
        let cycles: Vec<_> = starts
            .iter_mut()
            .enumerate()
            .map(|(_, cur)| {
                self.instrs
                    .iter()
                    .cycle()
                    .take_while(|instr| {
                        let node = self.graph.get(cur.as_str()).unwrap();
                        *cur = String::from(match instr {
                            Direction::Left => node.left,
                            Direction::Right => node.right,
                        });
                        !cur.ends_with('Z')
                    })
                    .count()
                    + 1
            })
            .collect();
        cycles.iter().fold(cycles[0], |x, &y| lcm(x, y))
    }
}

pub fn solve_p1(input: &str) -> usize {
    let storm = DesertStorm::new(input);
    storm.with_single_start("AAA")
}

pub fn solve_p2(input: &str) -> usize {
    let storm = DesertStorm::new(input);
    let starts: Vec<_> = storm
        .graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_string())
        .collect();
    storm.with_multiple_starts(starts)
}

pub fn solve(input: &str) -> (usize, usize) {
    (solve_p1(input), solve_p2(input))
}

common::test_distinct!(day08, 2, 6, (16343, 15299095336639));
