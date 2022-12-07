use std::collections::{HashSet, VecDeque};

fn find_packet_start(input: &str, distinct_count: usize) -> usize {
    let mut last_n = VecDeque::from_iter(input.chars().take(distinct_count));
    for (i, c) in input.chars().skip(distinct_count).enumerate() {
        if last_n.iter().collect::<HashSet<_>>().len() == distinct_count {
            return i + distinct_count;
        }

        last_n.pop_front();
        last_n.push_back(c);
    }

    panic!("No valid signal marker found");
}

pub fn solve_part1(input: &str) -> usize {
    find_packet_start(input, 4)
}

pub fn solve_part2(input: &str) -> usize {
    find_packet_start(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(7, solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, solve_part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(19, solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, solve_part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
