use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

use itertools::Itertools;

use crate::parsing::lines;

pub fn part1(input_path: &Path) -> u64 {
    let input = read_to_string(input_path).expect("puzzle input on disk");
    lines(&input)
        .map(partition)
        .map(|(left, right)| overlapping_items(left, right))
        .map(|overlap| overlap.into_iter().map(priority).sum::<u64>())
        .sum()
}

pub fn part2(input_path: &Path) -> u64 {
    let input = read_to_string(input_path).expect("puzzle input on disk");
    let elf_team_size = 3;
    let elf_teams = lines(&input).chunks(elf_team_size);

    elf_teams
        .into_iter()
        .map(decode_elf_team_into_sets)
        .map(|(elf1, elf2, elf3)| overlapping_items3(elf1, elf2, elf3))
        .map(|overlap| overlap.into_iter().map(priority).sum::<u64>())
        .sum()
}

fn partition(rucksack: &str) -> (HashSet<char>, HashSet<char>) {
    let left: HashSet<char> = HashSet::from_iter((&rucksack[..rucksack.len() / 2]).chars());
    let right: HashSet<char> = HashSet::from_iter((&rucksack[rucksack.len() / 2..]).chars());

    (left, right)
}

fn overlapping_items(left: HashSet<char>, right: HashSet<char>) -> HashSet<char> {
    HashSet::from_iter(left.intersection(&right).into_iter().copied())
}

fn decode_elf_team_into_sets<'a, S: Iterator<Item = &'a str>>(
    mut rucksack_set: S,
) -> (HashSet<char>, HashSet<char>, HashSet<char>) {
    let elf1: HashSet<char> = HashSet::from_iter(rucksack_set.next().expect("elf1").chars());
    let elf2: HashSet<char> = HashSet::from_iter(rucksack_set.next().expect("elf2").chars());
    let elf3: HashSet<char> = HashSet::from_iter(rucksack_set.next().expect("elf3").chars());

    (elf1, elf2, elf3)
}

fn overlapping_items3(
    elf1: HashSet<char>,
    elf2: HashSet<char>,
    elf3: HashSet<char>,
) -> HashSet<char> {
    let elf12 = HashSet::from_iter(elf1.intersection(&elf2).into_iter().copied());
    let elf123 = HashSet::from_iter(elf12.intersection(&elf3).into_iter().copied());

    elf123
}

fn priority(item: char) -> u64 {
    if item.is_ascii_lowercase() {
        let x: u64 = item.into();
        let a: u64 = 'a'.into();
        let lowercase_priority_begins = 1;
        return x - a + lowercase_priority_begins;
    }

    if item.is_ascii_uppercase() {
        let x: u64 = item.into();
        let a: u64 = 'A'.into();
        let uppercase_priority_begins = 27;
        return x - a + uppercase_priority_begins;
    }

    panic!("'{}' is not an ASCII letter", item);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_rucksack_test() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let (left, right) = partition(rucksack);

        let overlap = overlapping_items(left, right);
        let priorities = overlap.iter().copied().map(priority).collect_vec();
        let expected = HashSet::from_iter(['p'].into_iter());

        assert_eq!(expected, overlap);
        assert_eq!(vec![15], priorities);
    }

    #[test]
    fn overlapping_items3_test() {
        let (elf1, elf2, elf3) = decode_elf_team_into_sets(
            [
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
            ]
            .into_iter(),
        );
        let overlap_1 = overlapping_items3(elf1, elf2, elf3);
        let expected_1 = HashSet::from_iter(['r'].into_iter());
        assert_eq!(expected_1, overlap_1);

        let (elf4, elf5, elf6) = decode_elf_team_into_sets(
            [
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ]
            .into_iter(),
        );
        let overlap_2 = overlapping_items3(elf4, elf5, elf6);
        let expected_2 = HashSet::from_iter(['Z'].into_iter());
        assert_eq!(expected_2, overlap_2);
    }
}
