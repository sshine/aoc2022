use nom::bytes::complete::{tag, take_while1};
use nom::error::VerboseError;
use nom::IResult;
use std::ops::RangeInclusive;

type ParseResult<'a, Out> = IResult<&'a str, Out, VerboseError<&'a str>>;

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(parse_section_assignment)
        .filter(|section_assignment| overlaps_completely(section_assignment.clone()))
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(parse_section_assignment)
        .filter(|section_assignment| overlaps(section_assignment.clone()))
        .count() as u64
}

fn overlaps(pair: (RangeInclusive<u64>, RangeInclusive<u64>)) -> bool {
    let (x, y) = pair;
    (x.start() <= y.end() && y.start() <= x.end()) || (y.start() <= x.end() && x.end() <= y.end())
}

fn overlaps_completely(pair: (RangeInclusive<u64>, RangeInclusive<u64>)) -> bool {
    let (x, y) = pair;
    (x.start() >= y.start() && x.end() <= y.end()) || (y.start() >= x.start() && y.end() <= x.end())
}

fn parse_section_assignment(s: &str) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
    let (s, pair) = parse_range_pair(s).expect("sloppy elves!");
    assert!(s.is_empty(), "sloppier elves!");

    pair
}

fn parse_range_pair(s: &str) -> ParseResult<(RangeInclusive<u64>, RangeInclusive<u64>)> {
    let (s, elf1) = parse_range(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, elf2) = parse_range(s)?;

    Ok((s, (elf1, elf2)))
}

fn parse_range(s: &str) -> ParseResult<RangeInclusive<u64>> {
    let (s, from) = parse_number(s)?;
    let (s, _) = tag("-")(s)?;
    let (s, to) = parse_number(s)?;

    Ok((s, from..=to))
}

fn parse_number(s: &str) -> ParseResult<u64> {
    let (s, n) = take_while1(|c: char| c.is_digit(10))(s)?;
    let n = n.parse::<u64>().expect("But it's so pretty!");

    Ok((s, n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn part2_test() {
        assert_eq!(1, part2("5-7,7-9"));
        assert_eq!(2, part2("5-7,7-9\n2-8,3-7"));
        assert_eq!(3, part2("5-7,7-9\n2-8,3-7\n6-6,4-6"));
        assert_eq!(4, part2("5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"));

        assert_eq!(0, part2("2-4,6-8"));
        assert_eq!(0, part2("2-3,4-5"));
    }
}
