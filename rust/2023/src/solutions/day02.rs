pub fn input() -> &'static str {
    include_str!("../../input/day02.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day02.txt")
}

const RED_MAX: usize = 12;
const GREEN_MAX: usize = 13;
const BLUE_MAX: usize = 14;

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    fn from_str(line: &str) -> Self {
        let (left, rest) = line.split_once(':').unwrap();
        Self {
            rounds: rest.split(';').map(Round::from_str).collect(),
            id: left
                .split_once(' ')
                .map(|(_, n)| n.trim().parse::<usize>().unwrap())
                .unwrap(),
        }
    }
}

#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    fn from_str(round: &str) -> Round {
        let (red, green, blue) =
            round
                .split(',')
                .map(|x| x.trim())
                .fold((0, 0, 0), |acc, color| {
                    let (n, color) = color
                        .trim()
                        .split_once(' ')
                        .map(|(n, color)| (n.parse::<usize>().unwrap(), color))
                        .unwrap();
                    match color {
                        "red" => (n, acc.1, acc.2),
                        "green" => (acc.0, n, acc.2),
                        "blue" => (acc.0, acc.1, n),
                        _ => unreachable!(),
                    }
                });
        Round { red, green, blue }
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let possible: usize = input
        .lines()
        .map(Game::from_str)
        .filter_map(|game| {
            game.rounds
                .iter()
                .find(|round| {
                    round.red > RED_MAX || round.green > GREEN_MAX || round.blue > BLUE_MAX
                })
                .map_or(Some(game.id), |_| None)
        })
        .sum();
    let power = input
        .lines()
        .map(Game::from_str)
        .map(|game| {
            let (red, green, blue) = game.rounds.iter().fold((0, 0, 0), |acc, round| {
                (
                    acc.0.max(round.red),
                    acc.1.max(round.green),
                    acc.2.max(round.blue),
                )
            });
            red * green * blue
        })
        .sum();
    (possible, power)
}

common::test!(day02, (8, 2286), (2551, 62811));
