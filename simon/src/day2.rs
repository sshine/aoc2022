use phf::phf_map;
use std::{fs::read_to_string, path::Path};

use crate::parsing::lines;
use crate::rock_paper_scissors::{beats, Gesture, Gesture::*, Outcome, Outcome::*};

type Score = u64;

pub fn part1(input_path: &Path) -> Score {
    let input = read_to_string(input_path).expect("puzzle input on disk");
    lines(&input)
        .map(parse_elf_codes_part1)
        .map(|(them, you)| {
            let a = gesture_score(you);
            let b = outcome_score(find_outcome(them, you));
            a + b
        })
        .sum()
}

pub fn part2(input_path: &Path) -> Score {
    let input = read_to_string(input_path).expect("puzzle input on disk");
    lines(&input)
        .map(parse_elf_codes_part2)
        .map(|(them, outcome)| {
            let a = gesture_score(strategic_choice(them, outcome));
            let b = outcome_score(outcome);
            a + b
        })
        .sum()
}

fn find_outcome(them: Gesture, you: Gesture) -> Outcome {
    if them == you {
        return Draw;
    }

    if you == beats(them) {
        return Win;
    }

    if them == beats(you) {
        return Lose;
    }

    panic!("Unknown find_outcome({:?}, {:?})", them, you);
}

fn strategic_choice(them: Gesture, outcome: Outcome) -> Gesture {
    match outcome {
        Draw => them,
        Win => beats(them),
        Lose => beats(beats(them)),
    }
}

const ROCK_SCORE: Score = 1;
const PAPER_SCORE: Score = 2;
const SCISSORS_SCORE: Score = 3;

const LOSE_SCORE: Score = 0;
const DRAW_SCORE: Score = 3;
const WIN_SCORE: Score = 6;

fn gesture_score(gesture: Gesture) -> Score {
    match gesture {
        Rock => ROCK_SCORE,
        Paper => PAPER_SCORE,
        Scissors => SCISSORS_SCORE,
    }
}

fn outcome_score(outcome: Outcome) -> u64 {
    match outcome {
        Lose => LOSE_SCORE,
        Draw => DRAW_SCORE,
        Win => WIN_SCORE,
    }
}

fn parse_elf_codes_part1(line: &str) -> (Gesture, Gesture) {
    PART1_TABLE.get(line).unwrap().to_owned()
}

fn parse_elf_codes_part2(line: &str) -> (Gesture, Outcome) {
    PART2_TABLE.get(line).unwrap().to_owned()
}

// A = rock, B = paper, C = scissors
// X = rock, Y = paper, Z = scissors
static PART1_TABLE: phf::Map<&'static str, (Gesture, Gesture)> = phf_map! {
    "A X" => (Rock, Rock),
    "A Y" => (Rock, Paper),
    "A Z" => (Rock, Scissors),
    "B X" => (Paper, Rock),
    "B Y" => (Paper, Paper),
    "B Z" => (Paper, Scissors),
    "C X" => (Scissors, Rock),
    "C Y" => (Scissors, Paper),
    "C Z" => (Scissors, Scissors),
};

// A = rock, B = paper, C = scissors
// X = lose, Y = draw, Z = win
static PART2_TABLE: phf::Map<&'static str, (Gesture, Outcome)> = phf_map! {
    "A X" => (Rock, Lose),
    "A Y" => (Rock, Draw),
    "A Z" => (Rock, Win),
    "B X" => (Paper, Lose),
    "B Y" => (Paper, Draw),
    "B Z" => (Paper, Win),
    "C X" => (Scissors, Lose),
    "C Y" => (Scissors, Draw),
    "C Z" => (Scissors, Win),
};
