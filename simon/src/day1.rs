use itertools::Itertools;
use std::{fs::read_to_string, path::Path};

use crate::parsing::number_chunks;

pub fn part1(input_path: &Path) -> u64 {
    let input = read_to_string(input_path).expect("puzzle input on disk");
    number_chunks(&input)
        .map(|chunk| chunk.sum())
        .max()
        .unwrap_or(0)
}

pub fn part2(input_path: &Path) -> u64 {
    let input = read_to_string(input_path).expect("puzzle input on disk");
    number_chunks(&input)
        .map(|chunk| chunk.sum::<u64>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}
