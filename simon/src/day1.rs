use itertools::Itertools;

use crate::parsing::number_chunks;

pub fn part1(input: &str) -> u64 {
    number_chunks(&input)
        .map(|chunk| chunk.sum())
        .max()
        .unwrap_or(0)
}

pub fn part2(input: &str) -> u64 {
    number_chunks(&input)
        .map(|chunk| chunk.sum::<u64>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}
