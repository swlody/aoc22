use std::ops::RangeInclusive;

fn ranges_contain<T: PartialOrd>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool {
    a.contains(b.start()) && a.contains(b.end())
}

fn ranges_overlap<T: PartialOrd>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool {
    a.contains(b.start()) || a.contains(b.end())
}

pub fn generator(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            let (first_start, first_end) = first.split_once('-').unwrap();
            let (second_start, second_end) = second.split_once('-').unwrap();
            (
                first_start.parse().unwrap()..=first_end.parse().unwrap(),
                second_start.parse().unwrap()..=second_end.parse().unwrap(),
            )
        })
        .collect()
}

pub fn solve_part1(pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    pairs
        .iter()
        .filter(|(first_range, second_range)| {
            ranges_contain(first_range, second_range) || ranges_contain(second_range, first_range)
        })
        .count()
}

pub fn solve_part2(pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    pairs
        .iter()
        .filter(|(first_range, second_range)| {
            ranges_overlap(first_range, second_range) || ranges_overlap(second_range, first_range)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let pairs = generator(INPUT);
        let contained_pairs = solve_part1(&pairs);
        assert_eq!(2, contained_pairs);
    }

    #[test]
    fn test_part2() {
        let pairs = generator(INPUT);
        let overlapping_pairs = solve_part2(&pairs);
        assert_eq!(4, overlapping_pairs);
    }
}
