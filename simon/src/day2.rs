use phf::phf_map;
use Gesture::*;
use Outcome::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gesture {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

pub fn beats(them: Gesture) -> Gesture {
    match them {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
    }
}

pub fn find_outcome(them: Gesture, you: Gesture) -> Outcome {
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

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(parse_elf_codes_part1)
        .map(|(them, you)| {
            let a = gesture_score(you);
            let b = outcome_score(find_outcome(them, you));
            a + b
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(parse_elf_codes_part2)
        .map(|(them, outcome)| {
            let a = gesture_score(strategic_choice(them, outcome));
            let b = outcome_score(outcome);
            a + b
        })
        .sum()
}

fn strategic_choice(them: Gesture, outcome: Outcome) -> Gesture {
    match outcome {
        Draw => them,
        Win => beats(them),
        Lose => beats(beats(them)),
    }
}

const ROCK_SCORE: u64 = 1;
const PAPER_SCORE: u64 = 2;
const SCISSORS_SCORE: u64 = 3;

const LOSE_SCORE: u64 = 0;
const DRAW_SCORE: u64 = 3;
const WIN_SCORE: u64 = 6;

fn gesture_score(gesture: Gesture) -> u64 {
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
