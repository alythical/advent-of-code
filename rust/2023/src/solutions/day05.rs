use std::{collections::HashMap, ops::Range};

use common::par::{IntoParallelIterator, ParallelIterator};

pub fn input() -> &'static str {
    include_str!("../../input/day05.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day05.txt")
}

#[derive(Debug)]
struct SeedMap {
    inner: Vec<HashMap<Range<usize>, Range<usize>>>,
}

struct Seeds;

impl Seeds {
    fn individuals(seeds: &str) -> impl IntoIterator<Item = usize> + '_ {
        seeds.split(' ').skip(1).map(|n| n.parse().unwrap())
    }

    fn ranges(ranges: &str) -> Vec<Range<usize>> {
        ranges
            .split(' ')
            .skip(1)
            .array_chunks::<2>()
            .map(|[start, len]| [start.parse().unwrap(), len.parse().unwrap()])
            .map(|[start, len]| start..start + len)
            .collect()
    }
}

impl SeedMap {
    fn from_str(input: &str) -> Self {
        Self {
            inner: input
                .split("\n\n")
                .map(|mapping| mapping.lines().skip(1).map(Self::range).collect())
                .collect(),
        }
    }

    fn range(line: &str) -> (Range<usize>, Range<usize>) {
        let nums: Vec<_> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        let (&dst, &src, &range) = (&nums[0], &nums[1], &nums[2]);
        (src..src + range, dst..dst + range)
    }

    fn process(&self, seed: usize) -> usize {
        self.inner.iter().fold(seed, |acc, mapping| {
            mapping
                .iter()
                .find(|(k, _)| k.contains(&acc))
                .map_or(acc, |(src, dst)| dst.start + (acc - src.start))
        })
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let map = SeedMap::from_str(rest);
    let first = Seeds::individuals(seeds)
        .into_iter()
        .map(|seed| map.process(seed))
        .min()
        .unwrap();
    // No, I don't care
    let second = Seeds::ranges(seeds)
        .into_par_iter()
        .flat_map(|seeds| seeds.into_par_iter().map(|seed| map.process(seed)))
        .min()
        .unwrap();
    (first, second)
}

common::test!(day05, (35, 46), (486613012, 56931769));
