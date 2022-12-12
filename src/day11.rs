use std::collections::VecDeque;

enum Operation {
    Add(Option<u64>),
    Mul(Option<u64>),
}

impl From<&str> for Operation {
    fn from(op: &str) -> Self {
        let op = op.strip_prefix("  Operation: new = old ").unwrap();
        if let Some(value) = op.strip_prefix("* ") {
            if value == "old" {
                Operation::Mul(None)
            } else {
                Operation::Mul(Some(value.parse().unwrap()))
            }
        } else if let Some(value) = op.strip_prefix("+ ") {
            if value == "old" {
                Operation::Add(None)
            } else {
                Operation::Add(Some(value.parse().unwrap()))
            }
        } else {
            panic!("Invalid operation: {op:?}");
        }
    }
}

struct ThrowTest {
    divisible_by: u64,
    if_true_monkey: usize,
    if_false_monkey: usize,
}

impl From<(&str, &str, &str)> for ThrowTest {
    fn from((divisible_by, if_true, if_false): (&str, &str, &str)) -> Self {
        let divisible_by = divisible_by
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let if_true_monkey = if_true
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let if_false_monkey = if_false
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Self {
            divisible_by,
            if_true_monkey,
            if_false_monkey,
        }
    }
}

pub struct Monkey {
    starting_items: VecDeque<u64>,
    operation: Operation,
    throw_test: ThrowTest,
    inspected_items: u64,
}

impl From<&str> for Monkey {
    fn from(monkey: &str) -> Self {
        let mut lines = monkey.lines();
        let _monkey_id = lines
            .next()
            .unwrap()
            .strip_prefix("Monkey ")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let starting_items = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let operation = Operation::from(lines.next().unwrap());

        let throw_test = ThrowTest::from((
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        ));

        Self {
            starting_items,
            operation,
            throw_test,
            inspected_items: 0,
        }
    }
}

fn monkey_business(monkeys: &mut [Monkey], control: u64, rounds: u32) -> u64 {
    // 100% not stolen from reddit I swaer
    let all_divisors: u64 = monkeys
        .iter()
        .map(|monkey| monkey.throw_test.divisible_by)
        .product();

    for _round in 0..rounds {
        for monkey_id in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_id].starting_items.pop_front() {
                monkeys[monkey_id].inspected_items += 1;

                use Operation::*;
                let worry_level = match monkeys[monkey_id].operation {
                    Add(None) => item + item,
                    Add(Some(value)) => item + value,
                    Mul(None) => item * item,
                    Mul(Some(value)) => item * value,
                } / control;

                let new_monkey = if worry_level % monkeys[monkey_id].throw_test.divisible_by == 0 {
                    monkeys[monkey_id].throw_test.if_true_monkey
                } else {
                    monkeys[monkey_id].throw_test.if_false_monkey
                };

                monkeys[new_monkey]
                    .starting_items
                    .push_back(worry_level % all_divisors);
            }
        }
    }

    let mut first = 0;
    let mut second = 0;
    for monkey in monkeys {
        if monkey.inspected_items > second {
            if monkey.inspected_items > first {
                second = first;
                first = monkey.inspected_items;
            } else {
                second = monkey.inspected_items;
            }
        }
    }

    first * second
}

pub fn solve_part1(input: &str) -> u64 {
    let mut monkeys = input.split("\n\n").map(Monkey::from).collect::<Vec<_>>();
    monkey_business(&mut monkeys, 3, 20)
}

pub fn solve_part2(input: &str) -> u64 {
    let mut monkeys = input.split("\n\n").map(Monkey::from).collect::<Vec<_>>();
    monkey_business(&mut monkeys, 1, 10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2022/day11_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(10605, solve_part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2713310158, solve_part2(INPUT));
    }
}
