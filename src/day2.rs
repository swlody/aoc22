#[derive(Copy, Clone)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn score(&self) -> u32 {
        *self as u32
    }

    fn versus(&self, other: Self) -> Outcome {
        use Hand::*;
        use Outcome::*;
        match (self, other) {
            (Rock, Rock) => Tie,
            (Rock, Paper) => Loss,
            (Rock, Scissors) => Win,

            (Paper, Rock) => Win,
            (Paper, Paper) => Tie,
            (Paper, Scissors) => Loss,

            (Scissors, Rock) => Loss,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Tie,
        }
    }
}

impl From<char> for Hand {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("Invalid hand {value}"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Outcome {
    Loss = 0,
    Tie = 3,
    Win = 6,
}

impl Outcome {
    fn score(&self) -> u32 {
        *self as u32
    }

    fn match_with(&self, other_hand: Hand) -> Hand {
        use Hand::*;
        use Outcome::*;
        match (self, other_hand) {
            (Loss, Rock) => Scissors,
            (Loss, Paper) => Rock,
            (Loss, Scissors) => Paper,

            (Tie, Rock) => Rock,
            (Tie, Paper) => Paper,
            (Tie, Scissors) => Scissors,

            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
        }
    }
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Outcome::Loss,
            'Y' => Outcome::Tie,
            'Z' => Outcome::Win,
            _ => panic!("Invalid outcome {value}"),
        }
    }
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            (
                Hand::from(line.chars().nth(0).unwrap()),
                Hand::from(line.chars().nth(2).unwrap()),
            )
        })
        .map(|(their_hand, our_hand)| our_hand.versus(their_hand).score() + our_hand.score())
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            (
                Hand::from(line.chars().nth(0).unwrap()),
                Outcome::from(line.chars().nth(2).unwrap()),
            )
        })
        .map(|(their_hand, target_outcome)| {
            target_outcome.match_with(their_hand).score() + target_outcome.score()
        })
        .sum()
}
