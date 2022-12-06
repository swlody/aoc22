pub struct Movement {
    quantity: usize,
    start_stack: usize,
    end_stack: usize,
}

fn get_stacks_and_procedure(input: &str) -> (Vec<Vec<char>>, Vec<Movement>) {
    let (original_stacks, move_procedure) = input.split_once("\n\n").unwrap();
    let mut stacks = Vec::new();

    for line in original_stacks.lines().rev() {
        if line.starts_with(" 1 ") {
            // read to end of line to figure out how many stacks we need
            let num_stacks = (line.len() + 3) / 4;
            for _ in 0..num_stacks {
                stacks.push(Vec::new());
            }
        } else {
            for (stack_number, i) in (1..line.len()).step_by(4).enumerate() {
                let crate_name = line.as_bytes()[i];
                if crate_name != b' ' {
                    stacks[stack_number].push(char::from(crate_name));
                }
            }
        }
    }

    let mut procedure = Vec::new();
    for movement in move_procedure.lines() {
        let (quantity, rest) = movement
            .strip_prefix("move ")
            .unwrap()
            .split_once(" from ")
            .unwrap();
        let (start_stack, end_stack) = rest.split_once(" to ").unwrap();

        procedure.push(Movement {
            quantity: quantity.parse().unwrap(),
            start_stack: start_stack.parse::<usize>().unwrap() - 1,
            end_stack: end_stack.parse::<usize>().unwrap() - 1,
        });
    }

    (stacks, procedure)
}

fn get_tops_of_stacks(stacks: &Vec<Vec<char>>) -> String {
    let mut tops_of_stacks = String::new();
    for stack in stacks {
        tops_of_stacks.push(stack[stack.len() - 1]);
    }
    tops_of_stacks
}

pub fn solve_part1(input: &str) -> String {
    let (mut stacks, procedure) = get_stacks_and_procedure(input);

    for movement in &procedure {
        for _ in 0..movement.quantity {
            let crate_name = stacks[movement.start_stack].pop().unwrap();
            stacks[movement.end_stack].push(crate_name);
        }
    }

    get_tops_of_stacks(&stacks)
}

pub fn solve_part2(input: &str) -> String {
    let (mut stacks, procedure) = get_stacks_and_procedure(input);

    for movement in &procedure {
        let mut moving_crates = Vec::with_capacity(movement.quantity);
        for _ in 0..movement.quantity {
            let crate_name = stacks[movement.start_stack].pop().unwrap();
            moving_crates.push(crate_name);
        }
        stacks[movement.end_stack].extend(moving_crates.iter().rev());
    }

    get_tops_of_stacks(&stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let tops_of_stacks = solve_part1(INPUT);
        assert_eq!("CMZ", tops_of_stacks);
    }

    #[test]
    fn test_part2() {
        let tops_of_stacks = solve_part2(INPUT);
        assert_eq!("MCD", tops_of_stacks);
    }
}
