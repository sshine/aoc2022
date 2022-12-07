use std::{fs::read_to_string, path::Path};

pub fn read_input(input_path: &Path) -> String {
    read_to_string(input_path).expect("puzzle input on disk")
}

pub fn number_chunks(input: &str) -> impl Iterator<Item = impl Iterator<Item = u64> + '_> + '_ {
    input.split("\n\n").map(numbers)
}

pub fn numbers(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.lines().map(|n| n.parse::<u64>().expect("number"))
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn numbers_test() {
        let cases = [("0", vec![0]), ("0\n1", vec![0, 1])];
        for (input, expected) in cases {
            let actual = numbers(input).collect_vec();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    #[should_panic]
    fn numbers_empty_test() {
        let _ = numbers("").collect_vec();
    }
}
