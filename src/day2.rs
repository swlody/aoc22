use anyhow::{anyhow, Error, Result};

#[derive(Copy, Clone)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn score(self) -> u32 {
        self as u32
    }

    fn versus(self, other: Self) -> Outcome {
        use Hand::*;
        use Outcome::*;
        match (self, other) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Tie,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Loss,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
        }
    }
}

impl TryFrom<u8> for Hand {
    type Error = Error;

    fn try_from(c: u8) -> Result<Self> {
        match c {
            b'A' | b'X' => Ok(Self::Rock),
            b'B' | b'Y' => Ok(Self::Paper),
            b'C' | b'Z' => Ok(Self::Scissors),
            _ => Err(anyhow!("Invalid hand {c}")),
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
    fn score(self) -> u32 {
        self as u32
    }

    fn match_with(self, other_hand: Hand) -> Hand {
        use Hand::*;
        use Outcome::*;
        match (self, other_hand) {
            (Loss, Rock) | (Tie, Scissors) | (Win, Paper) => Scissors,
            (Loss, Paper) | (Tie, Rock) | (Win, Scissors) => Rock,
            (Loss, Scissors) | (Tie, Paper) | (Win, Rock) => Paper,
        }
    }
}

impl TryFrom<u8> for Outcome {
    type Error = Error;

    fn try_from(c: u8) -> Result<Self> {
        match c {
            b'X' => Ok(Self::Loss),
            b'Y' => Ok(Self::Tie),
            b'Z' => Ok(Self::Win),
            _ => Err(anyhow!("Invalid outcome {c}")),
        }
    }
}

pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            (
                Hand::try_from(line.as_bytes()[0]).unwrap(),
                Hand::try_from(line.as_bytes()[2]).unwrap(),
            )
        })
        .map(|(their_hand, our_hand)| our_hand.versus(their_hand).score() + our_hand.score())
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            (
                Hand::try_from(line.as_bytes()[0]).unwrap(),
                Outcome::try_from(line.as_bytes()[2]).unwrap(),
            )
        })
        .map(|(their_hand, target_outcome)| {
            target_outcome.match_with(their_hand).score() + target_outcome.score()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        let score = solve_part1(INPUT);
        assert_eq!(15, score);
    }

    #[test]
    fn test_part2() {
        let score = solve_part2(INPUT);
        assert_eq!(12, score);
    }
}
