use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    for (i, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
        if are_distinct(a, b, c, d) {
            return 4 + i;
        }
    }

    return 0;
}

pub fn part2(input: &str) -> u64 {
    todo!()
}

fn are_distinct(a: char, b: char, c: char, d: char) -> bool {
    a != b && a != c && a != d && b != c && b != d && c != d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_first_marker_test() {
        let cases = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, part1(input));
        }
    }
}
