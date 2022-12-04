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

impl From<u8> for Hand {
    fn from(value: u8) -> Self {
        match value {
            b'A' | b'X' => Hand::Rock,
            b'B' | b'Y' => Hand::Paper,
            b'C' | b'Z' => Hand::Scissors,
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

impl From<u8> for Outcome {
    fn from(value: u8) -> Self {
        match value {
            b'X' => Outcome::Loss,
            b'Y' => Outcome::Tie,
            b'Z' => Outcome::Win,
            _ => panic!("Invalid outcome {value}"),
        }
    }
}

pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            (
                Hand::from(line.as_bytes()[0]),
                Hand::from(line.as_bytes()[2]),
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
                Hand::from(line.as_bytes()[0]),
                Outcome::from(line.as_bytes()[2]),
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
        let score = solve_part1(&INPUT);
        assert_eq!(15, score);
    }

    #[test]
    fn test_part2() {
        let score = solve_part2(&INPUT);
        assert_eq!(12, score);
    }
}
