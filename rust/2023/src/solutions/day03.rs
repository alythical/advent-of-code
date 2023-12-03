use std::collections::HashSet;

pub fn input() -> &'static str {
    include_str!("../../input/day03.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day03.txt")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ItemKind {
    Number(usize),
    Symbol(char),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Item {
    kind: ItemKind,
    from: usize,
    to: usize,
    row: usize,
}

impl From<Item> for usize {
    fn from(value: Item) -> Self {
        match value.kind {
            ItemKind::Number(n) => n,
            _ => panic!(),
        }
    }
}

impl Item {
    fn symbol(schematic: &Schematic, c: char) -> Self {
        Self {
            kind: ItemKind::Symbol(c),
            from: schematic.from,
            to: schematic.to,
            row: schematic.row,
        }
    }

    fn number(schematic: &Schematic, n: usize) -> Self {
        Self {
            kind: ItemKind::Number(n),
            from: schematic.from,
            to: schematic.to - 1,
            row: schematic.row,
        }
    }
}

#[derive(Debug)]
struct Schematic<'a> {
    parts: Vec<Item>,
    symbols: Vec<Item>,
    dimensions: (usize, usize),
    lines: Vec<&'a str>,
    row: usize,
    from: usize,
    to: usize,
}

impl<'a> Schematic<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let mut schematic = Self {
            dimensions: (lines.first().unwrap().len(), lines.len()),
            parts: vec![],
            symbols: vec![],
            row: 0,
            from: 0,
            to: 0,
            lines,
        };
        loop {
            match schematic.peek() {
                Some(Some('0'..='9')) => {
                    let n = schematic.num();
                    schematic.parts.push(Item::number(&schematic, n));
                }
                Some(Some('.')) => schematic.to += 1,
                Some(Some(c)) => {
                    schematic.symbols.push(Item::symbol(&schematic, c));
                    schematic.to += 1;
                }
                Some(None) => schematic.newline(),
                None => break,
            }
            schematic.from = schematic.to;
        }
        schematic
    }

    fn newline(&mut self) {
        self.row += 1;
        self.from = 0;
        self.to = 0;
    }

    fn num(&mut self) -> usize {
        let mut digits = vec![];
        while let Some(Some(c)) = self.peek() {
            if c.is_numeric() {
                digits.push(c);
                self.to += 1;
            } else {
                break;
            }
        }
        digits.iter().collect::<String>().parse().unwrap()
    }

    fn peek(&self) -> Option<Option<char>> {
        self.lines
            .get(self.row)
            .map(|line| line.chars().nth(self.to))
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let schematic = Schematic::from_str(input);
    (part_sum(&schematic), gear_ratio_sum(&schematic))
}

fn part_sum(schematic: &Schematic<'_>) -> usize {
    schematic
        .parts
        .iter()
        .filter(|part| {
            common::grid::neighbors(schematic.dimensions, (part.from, part.row), true)
                .into_iter()
                .chain(common::grid::neighbors(
                    schematic.dimensions,
                    (part.to, part.row),
                    true,
                ))
                .any(|(col, row)| {
                    schematic
                        .symbols
                        .iter()
                        .any(|part| part.from == col && part.row == row)
                })
        })
        .map(|&item| usize::from(item))
        .sum()
}

fn gear_ratio_sum(schematic: &Schematic<'_>) -> usize {
    schematic
        .symbols
        .iter()
        .filter(|item| matches!(item.kind, ItemKind::Symbol('*')))
        .filter_map(|gear| {
            let neighbors: HashSet<_> =
                common::grid::neighbors(schematic.dimensions, (gear.from, gear.row), true)
                    .into_iter()
                    .filter_map(|(col, row)| {
                        schematic
                            .parts
                            .iter()
                            .find(|part| (part.from == col || part.to == col) && part.row == row)
                    })
                    .collect();
            if neighbors.len() == 2 {
                Some(neighbors.iter().fold(1, |acc, &&e| acc * usize::from(e)))
            } else {
                None
            }
        })
        .sum()
}

common::test!(day03, (4361, 467835), (526404, 84399773));
