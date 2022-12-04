pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse().unwrap()).collect())
        .collect()
}

pub fn solve_part1(calories: &[Vec<u32>]) -> u32 {
    calories.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

pub fn solve_part2(calories: &[Vec<u32>]) -> u32 {
    let (mut one, mut two, mut three) = (0, 0, 0);
    for elf in calories.iter() {
        let elf_calories = elf.iter().sum();
        if elf_calories > one {
            three = two;
            two = one;
            one = elf_calories;
        } else if elf_calories > two {
            three = two;
            two = elf_calories;
        } else if elf_calories > three {
            three = elf_calories;
        }
    }
    one + two + three
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        let calories = generator(INPUT);
        let total_calories = solve_part1(&calories);
        assert_eq!(24000, total_calories);
    }

    #[test]
    fn test_part2() {
        let calories = generator(INPUT);
        let total_calories = solve_part2(&calories);
        assert_eq!(45000, total_calories);
    }
}
