use std::fs::read_to_string;
use std::path::Path;

pub fn read_number_chunks(path: &Path) -> Vec<Vec<u64>> {
    number_chunks(&read_to_string(path).unwrap())
}

pub fn number_chunks(input: &str) -> Vec<Vec<u64>> {
    input.split("\n\n").map(numbers).collect()
}

pub fn numbers(input: &str) -> Vec<u64> {
    lines(input)
        .flat_map(|n| n.parse::<u64>().map(|n_| vec![n_]).unwrap_or(vec![]))
        .collect()
}

pub fn lines(input: &str) -> impl Iterator<Item = &str> {
    input.strip_suffix("\n").unwrap_or(input).split("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_test() {
        let cases = [("", vec![]), ("0", vec![0]), ("0\n1", vec![0, 1])];
        for (input, expected) in cases {
            let actual = numbers(input);
            assert_eq!(expected, actual,);
        }
    }
}
