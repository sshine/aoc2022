use std::fs::read_to_string;
use std::path::Path;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub fn read_input(input_path: &Path) -> String {
    read_to_string(input_path).expect("puzzle input on disk")
}
