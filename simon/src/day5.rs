use std::{
    collections::VecDeque,
    fmt::{self, Display, Formatter},
};

use itertools::Itertools;
use regex::Regex;

pub fn part1(input: &str) -> String {
    let mut cargo_plan = parse_cargo_plan(input);
    cargo_plan.simulate_cratemover_9000();
    cargo_plan.decode_top_crates()
}

pub fn part2(input: &str) -> String {
    let mut cargo_plan = parse_cargo_plan(input);
    cargo_plan.simulate_cratemover_9001();
    cargo_plan.decode_top_crates()
}

#[derive(Debug)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct CargoPlan {
    stacks: Vec<Vec<char>>,
    moves: VecDeque<Move>,
}

impl Display for CargoPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, stack) in self.stacks.iter().enumerate() {
            let x: String = stack.iter().collect();
            writeln!(f, "{}: {}", i + 1, x)?;
        }

        writeln!(f)?;

        for mov in self.moves.iter() {
            writeln!(f, "move {} from {} to {}", mov.n, mov.from, mov.to)?;
        }

        Ok(())
    }
}

impl CargoPlan {
    fn simulate_cratemover_9000(&mut self) {
        while self.step_cratemover_9000() {}
    }

    fn step_cratemover_9000(&mut self) -> bool {
        if let Some(mov) = self.moves.pop_front() {
            for _ in 0..mov.n {
                let c = self.stacks[mov.from - 1].pop().expect("Christmas");
                self.stacks[mov.to - 1].push(c);
            }

            return true;
        }

        false
    }

    fn simulate_cratemover_9001(&mut self) {
        while self.step_cratemover_9001() {}
    }

    fn step_cratemover_9001(&mut self) -> bool {
        if let Some(mov) = self.moves.pop_front() {
            // always remove at the same index
            let remove_index = self.stacks[mov.from - 1].len() - mov.n;
            for _ in 0..mov.n {
                let c = self.stacks[mov.from - 1].remove(remove_index);
                self.stacks[mov.to - 1].push(c);
            }

            return true;
        }

        false
    }

    fn decode_top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&'?'))
            .collect()
    }
}

fn parse_cargo_plan(input: &str) -> CargoPlan {
    let mut cargo_plan = input.split("\n\n");
    let stacks = transpose_crate_drawing(parse_crate_drawing(
        cargo_plan.next().expect("a drawing of crates"),
    ));
    let moves = parse_move_lines(cargo_plan.next().expect("a list of moves"));

    CargoPlan { stacks, moves }
}

fn transpose_crate_drawing(mut crate_layout: Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    // determine the number of stacks based on how many labels beneath the drawing
    let num_stacks = crate_layout.remove(crate_layout.len() - 1).len();
    let mut stacks = vec![vec![]; num_stacks];

    for row_i in (0..crate_layout.len()).rev() {
        for (stack_j, c) in crate_layout[row_i].iter().enumerate() {
            if let Some(c) = c {
                stacks[stack_j].push(*c);
            }
        }
    }

    stacks
}

fn parse_crate_drawing(crate_drawing: &str) -> Vec<Vec<Option<char>>> {
    crate_drawing
        .lines()
        .map(parse_crate_drawing_line)
        .collect()
}

fn parse_crate_drawing_line(line: &str) -> Vec<Option<char>> {
    let chars_per_slot = 4;
    let chunks = &line.chars().chunks(chars_per_slot);

    // Look at second char of chunk:
    // "    " means no crate.
    // "[X] " means crate X.
    chunks
        .into_iter()
        .map(|chunk| parse_crate(chunk.skip(1).next()))
        .collect()
}

fn parse_crate(c: Option<char>) -> Option<char> {
    match c {
        Some(' ') => None,
        None => None,
        _ => c,
    }
}

fn parse_move_lines(move_plan: &str) -> VecDeque<Move> {
    let re = Regex::new(r"^move (?P<n>\d+) from (?P<from>\d+) to (?P<to>\d+)$").expect("pixies");
    move_plan
        .split("\n")
        .map(|line| parse_move_line(&re, line))
        .collect()
}

fn parse_move_line(re: &Regex, line: &str) -> Move {
    let capture = re
        .captures(line)
        .expect(&format!("a move line, '{}'", line));
    let n = (&capture["n"]).parse().expect("a number of crates");
    let from = (&capture["from"]).parse().expect("a 'from' stack'");
    let to = (&capture["to"]).parse().expect("a 'to' stack'");

    Move { n, from, to }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        let input: &str = include_str!("../inputs/day05_test.txt");
        let mut cargo_plan = parse_cargo_plan(input);

        cargo_plan.simulate_cratemover_9000();
        let top_crates = cargo_plan.decode_top_crates();
        assert_eq!("CMZ", &top_crates);
    }
}
