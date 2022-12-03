use itertools::Itertools;
use std::path::Path;

use crate::aux::read_number_chunks;

pub fn part1(input_path: &Path) -> u64 {
    read_number_chunks(input_path)
        .iter()
        .map(|chunk| chunk.iter().sum())
        .max()
        .unwrap_or(0)
}

pub fn part2(input_path: &Path) -> u64 {
    read_number_chunks(input_path)
        .iter()
        .map(|chunk| chunk.iter().sum::<u64>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}
