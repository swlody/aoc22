use itertools::Itertools;

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

    fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

impl From<&str> for ItemSet {
    fn from(s: &str) -> Self {
        let mut set = ItemSet::new();
        for c in s.chars() {
            set.insert(c);
        }
        set
    }
}

pub fn solve_part1(input: &str) -> u64 {
    let mut priority_sum = 0;
    for rucksack in input.lines() {
        let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);
        assert_eq!(first_half.len(), second_half.len());
        println!("{first_half} : {second_half}");
        let set = ItemSet::from(first_half);
        for c in second_half.chars() {
            if set.contains(c) {
                priority_sum += char_priority(c);
                break;
            }
        }
    }
    priority_sum
}

pub fn solve_part2(input: &str) -> u64 {
    let mut priority_sum = 0;
    for (elf_1, elf_2, elf_3) in input.lines().tuples() {
        let set_1 = ItemSet::from(elf_1);
        let set_2 = ItemSet::from(elf_2);
        let intersection = set_1.intersection(set_2);
        for c in elf_3.chars() {
            if intersection.contains(c) {
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
        let priority_sum = solve_part1(&INPUT);
        assert_eq!(157, priority_sum);
    }

    #[test]
    fn test_part2() {
        let priority_sum = solve_part2(&INPUT);
        assert_eq!(70, priority_sum);
    }
}
