pub fn neighbors(
    (width, height): (usize, usize),
    (row, col): (usize, usize),
    diagonals: bool,
) -> Vec<(usize, usize)> {
    let mut checks: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    if diagonals {
        checks.extend([(1, 1), (1, -1), (-1, 1), (-1, -1)].iter())
    }

    if !exists((width, height), (row, col)) {
        panic!(
            "Invalid (row, col) pair: ({row}, {col}) for size: ({}, {})",
            width, height
        );
    }

    let mut neighbors: Vec<(usize, usize)> = vec![];
    for (radjust, cadjust) in checks {
        let r = (row as isize + radjust).try_into();
        let c = (col as isize + cadjust).try_into();

        if let (Ok(r), Ok(c)) = (r, c) {
            neighbors.push((r, c));
        } else {
            continue;
        };
    }

    neighbors
}

fn exists((width, height): (usize, usize), (row, col): (usize, usize)) -> bool {
    row < height && col < width
}