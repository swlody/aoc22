use std::collections::VecDeque;

enum Value {
    Old,
    Imm(i64),
}

enum Operation {
    Add(Value),
    Mul(Value),
}

impl From<&str> for Operation {
    fn from(op: &str) -> Self {
        let op = op.strip_prefix("  Operation: new = old ").unwrap();
        if let Some(value) = op.strip_prefix("* ") {
            if value == "old" {
                Operation::Mul(Value::Old)
            } else {
                Operation::Mul(Value::Imm(value.parse().unwrap()))
            }
        } else if let Some(value) = op.strip_prefix("+ ") {
            if value == "old" {
                Operation::Add(Value::Old)
            } else {
                Operation::Add(Value::Imm(value.parse().unwrap()))
            }
        } else {
            panic!("Invalid operation: {op:?}");
        }
    }
}

struct ThrowTest {
    divisible_by: i64,
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
    starting_items: VecDeque<i64>,
    operation: Operation,
    throw_test: ThrowTest,
    inspected_items: i64,
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

pub fn solve_part1(input: &str) -> i64 {
    let mut monkeys = input.split("\n\n").map(Monkey::from).collect::<Vec<_>>();

    for _round in 0..20 {
        for monkey_id in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_id].starting_items.pop_front() {
                monkeys[monkey_id].inspected_items += 1;

                use {Operation::*, Value::*};
                let worry_level = match monkeys[monkey_id].operation {
                    Add(Old) => item + item,
                    Add(Imm(value)) => item + value,
                    Mul(Old) => item * item,
                    Mul(Imm(value)) => item * value,
                } / 3;

                let new_monkey = if worry_level % monkeys[monkey_id].throw_test.divisible_by == 0 {
                    monkeys[monkey_id].throw_test.if_true_monkey
                } else {
                    monkeys[monkey_id].throw_test.if_false_monkey
                };

                monkeys[new_monkey].starting_items.push_back(worry_level);
            }
        }
    }

    let mut first = 0;
    let mut second = 0;
    for monkey in &monkeys {
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

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2022/day11_test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(10605, solve_part1(INPUT));
    }
}
