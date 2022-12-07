use std::path::Path;

use simon::day1;
use simon::day2;
use simon::day3;
use simon::day4;
use simon::day5;
use simon::read_input;

fn main() {
    let day1_input = read_input(Path::new("./inputs/day01.txt"));
    println!("Day 1, part 1: {}", day1::part1(&day1_input));
    println!("Day 1, part 2: {}", day1::part2(&day1_input));

    let day2_input = read_input(Path::new("./inputs/day02.txt"));
    println!("Day 2, part 1: {}", day2::part1(&day2_input));
    println!("Day 2, part 2: {}", day2::part2(&day2_input));

    let day3_input = read_input(Path::new("./inputs/day03.txt"));
    println!("Day 3, part 1: {}", day3::part1(&day3_input));
    println!("Day 3, part 2: {}", day3::part2(&day3_input));

    let day4_input = read_input(Path::new("./inputs/day04.txt"));
    println!("Day 4, part 1: {}", day4::part1(&day4_input));
    println!("Day 4, part 2: {}", day4::part2(&day4_input));

    let day5_input = read_input(Path::new("./inputs/day05.txt"));
    println!("Day 5, part 1: {}", day5::part1(&day5_input));
    println!("Day 5, part 2: {}", day5::part2(&day5_input));
}
