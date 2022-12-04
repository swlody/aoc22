fn char_priority(c: char) -> u64 {
    if c.is_ascii_uppercase() {
        u64::from(c) - b'A' as u64 + 27
    } else if c.is_ascii_lowercase() {
        u64::from(c) - b'a' as u64 + 1
    } else {
        panic!("Invalid char {c}")
    }
}

struct ItemSet(u64);

impl ItemSet {
    fn new() -> Self {
        Self(0)
    }

    fn insert(&mut self, c: char) {
        self.0 |= 1 << char_priority(c);
    }

    fn contains(&self, c: char) -> bool {
        self.0 & (1 << char_priority(c)) != 0
    }
}

pub fn solve_part1(input: &str) -> u64 {
    let mut priority_sum = 0;
    for rucksack in input.lines() {
        let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);
        assert_eq!(first_half.len(), second_half.len());
        println!("{first_half} : {second_half}");
        let mut set = ItemSet::new();
        for c in first_half.chars() {
            set.insert(c);
        }
        for c in second_half.chars() {
            if set.contains(c) {
                priority_sum += char_priority(c);
                break;
            }
        }
    }
    priority_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let answer = solve_part1(&INPUT);
        assert_eq!(157, answer);
    }
}
