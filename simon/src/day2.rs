use phf::phf_map;
use std::{fs::read_to_string, path::Path};

use crate::aux::lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Gesture {
    Rock,
    Paper,
    Scissors,
}
use Gesture::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Outcome {
    Lose,
    Draw,
    Win,
}
use Outcome::*;

type Score = u64;

const ROCK_SCORE: Score = 1;
const PAPER_SCORE: Score = 2;
const SCISSORS_SCORE: Score = 3;

const LOSE_SCORE: Score = 0;
const DRAW_SCORE: Score = 3;
const WIN_SCORE: Score = 6;

pub fn part1(input_path: &Path) -> Score {
    let input = read_to_string(input_path).unwrap();
    lines(&input)
        .map(lookup_part1)
        .map(|(them, you)| gesture_score(you) + outcome_score(find_outcome(them, you)))
        .sum()
}

pub fn part2(input_path: &Path) -> Score {
    let input = read_to_string(input_path).unwrap();
    lines(&input)
        .map(lookup_part2)
        .map(|(them, outcome)| {
            gesture_score(strategic_choice(them, outcome)) + outcome_score(outcome)
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

fn beats(them: Gesture) -> Gesture {
    match them {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
    }
}

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

fn lookup_part1(line: &str) -> (Gesture, Gesture) {
    PART1_TABLE.get(line).unwrap().to_owned()
}

fn lookup_part2(line: &str) -> (Gesture, Outcome) {
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
