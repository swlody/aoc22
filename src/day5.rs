pub struct Movement {
    quantity: u32,
    start_stack: usize,
    end_stack: usize,
}

pub fn solve_part1(input: &str) -> String {
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

    let movements = {
        let mut movements = Vec::new();
        for movement in move_procedure.lines() {
            let (quantity, rest) = movement
                .strip_prefix("move ")
                .unwrap()
                .split_once(" from ")
                .unwrap();
            let (start_stack, end_stack) = rest.split_once(" to ").unwrap();

            movements.push(Movement {
                quantity: quantity.parse().unwrap(),
                start_stack: start_stack.parse::<usize>().unwrap() - 1,
                end_stack: end_stack.parse::<usize>().unwrap() - 1,
            });
        }
        movements
    };

    for movement in movements.iter() {
        for _ in 0..movement.quantity {
            let crate_name = stacks[movement.start_stack].pop().unwrap();
            stacks[movement.end_stack].push(crate_name);
        }
    }

    let mut tops_of_stacks = String::new();
    for stack in stacks {
        tops_of_stacks.push(stack[stack.len() - 1]);
    }
    tops_of_stacks
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
}
