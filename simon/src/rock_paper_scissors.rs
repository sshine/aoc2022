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
