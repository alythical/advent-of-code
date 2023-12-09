pub fn input() -> &'static str {
    include_str!("../../input/day09.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day09.txt")
}

type Report = Vec<Vec<isize>>;

fn extrapolate(report: Report, f: impl Fn(Report) -> isize) -> isize {
    report
        .into_iter()
        .map(|history| {
            let mut report = vec![history];
            while let Some(last) = report.last()
                && !last.iter().all(|&n| n == 0)
            {
                let history = last
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect();
                report.push(history);
            }
            f(report)
        })
        .sum()
}

fn extrapolate_left(report: Report) -> isize {
    report
        .iter()
        .rev()
        .fold(0, |acc, report| report.first().unwrap() - acc)
}

fn extrapolate_right(report: Report) -> isize {
    report
        .iter()
        .rev()
        .fold(0, |acc, report| acc + report.last().unwrap())
}

pub fn solve(input: &str) -> (isize, isize) {
    let report: Report = input
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect();
    let right = extrapolate(report.clone(), extrapolate_right);
    let left = extrapolate(report, extrapolate_left);
    (right, left)
}

common::test!(day09, (114, 2), (1702218515, 925));
