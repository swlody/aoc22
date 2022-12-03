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

    fn versus(&self, other: &Self) -> u32 {
        use Hand::*;
        match (self, other) {
            (Rock, Scissors) => 6,
            (Rock, Rock) => 3,
            (Rock, Paper) => 0,
            (Paper, Rock) => 6,
            (Paper, Paper) => 3,
            (Paper, Scissors) => 0,
            (Scissors, Paper) => 6,
            (Scissors, Scissors) => 3,
            (Scissors, Rock) => 0,
        }
    }
}

impl From<char> for Hand {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("Invalid input {value}"),
        }
    }
}

pub fn generator(input: &str) -> Vec<(Hand, Hand)> {
    input
        .lines()
        .map(|line| {
            (
                Hand::from(line.chars().nth(0).unwrap()),
                Hand::from(line.chars().nth(2).unwrap()),
            )
        })
        .collect()
}

pub fn part1(guide: &[(Hand, Hand)]) -> u32 {
    guide
        .iter()
        .map(|(their_hand, our_hand)| our_hand.versus(their_hand) + our_hand.score())
        .sum()
}
