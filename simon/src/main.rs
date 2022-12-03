use std::path::Path;

use simon::day1;
use simon::day2;

fn main() {
    let day1_input = Path::new("../inputs/day01.txt");
    println!("Day 1, part 1: {}", day1::part1(day1_input));
    println!("Day 1, part 2: {}", day1::part2(day1_input));

    let day2_input = Path::new("../inputs/day02.txt");
    println!("Day 2, part 1: {}", day2::part1(day2_input));
    println!("Day 2, part 1: {}", day2::part2(day2_input));
}
