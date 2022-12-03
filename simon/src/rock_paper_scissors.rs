use Gesture::*;

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
