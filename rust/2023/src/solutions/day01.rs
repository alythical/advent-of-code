pub fn input() -> &'static str {
    include_str!("../../input/day01.txt")
}

pub fn test_input_p1() -> &'static str {
    include_str!("../../input/tests/day01.txt")
}

pub fn test_input_p2() -> &'static str {
    include_str!("../../input/tests/day01.2.txt")
}

pub fn solve_p1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    lines
        .iter()
        .map(|line| {
            let first = line.chars().find_map(int).unwrap();
            let second = line.chars().rev().find_map(int).unwrap();
            first * 10 + second
        })
        .sum()
}

pub fn solve_p2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    lines.iter().map(calibration_values).sum()
}

pub fn solve(input: &str) -> (u32, u32) {
    (solve_p1(input), solve_p2(input))
}

fn calibration_values(line: &&str) -> u32 {
    let mut prefix = vec![];
    let mut matched = vec![];
    for n in 0..line.len() {
        for (n, c) in line.chars().enumerate().skip(n) {
            prefix.push((n, c));
            let chars: Vec<_> = prefix.iter().map(|(_, c)| c).copied().collect();
            if let Some(found) = fancy_int(&chars) {
                let start = prefix.first().unwrap().0;
                matched.push((start, found));
                prefix.clear();
                break;
            }
        }
        prefix.clear();
    }
    if let Some(first) = line.chars().enumerate().find_map(tagged_int) {
        matched.push(first);
    }
    let collected: Vec<_> = line.chars().enumerate().collect();
    if let Some(second) = collected.iter().rev().find_map(|&v| tagged_int(v)) {
        matched.push(second);
    }
    matched.sort_by_key(|(i, _)| *i);
    let first = matched.first().unwrap().1;
    let last = matched.last().unwrap().1;
    first * 10 + last
}

fn int(c: char) -> Option<u32> {
    if c.is_numeric() {
        Some(c.to_digit(10).unwrap())
    } else {
        None
    }
}

fn tagged_int(t: (usize, char)) -> Option<(usize, u32)> {
    int(t.1).map(|v| (t.0, v))
}

fn fancy_int(s: &[char]) -> Option<u32> {
    match s {
        ['o', 'n', 'e'] => Some(1),
        ['t', 'w', 'o'] => Some(2),
        ['t', 'h', 'r', 'e', 'e'] => Some(3),
        ['f', 'o', 'u', 'r'] => Some(4),
        ['f', 'i', 'v', 'e'] => Some(5),
        ['s', 'i', 'x'] => Some(6),
        ['s', 'e', 'v', 'e', 'n'] => Some(7),
        ['e', 'i', 'g', 'h', 't'] => Some(8),
        ['n', 'i', 'n', 'e'] => Some(9),
        _ => None,
    }
}

common::test_distinct!(day01, 142, 281, (56397, 55701));
