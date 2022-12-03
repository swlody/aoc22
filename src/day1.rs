pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse().unwrap()).collect())
        .collect()
}

pub fn part1(calories: &[Vec<u32>]) -> u32 {
    calories.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

pub fn part2(calories: &[Vec<u32>]) -> u32 {
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
