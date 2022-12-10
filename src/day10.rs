#[derive(Copy, Clone, Debug)]
pub enum Instr {
    Noop,
    Addx(i32),
}

impl From<&str> for Instr {
    fn from(value: &str) -> Self {
        if value.starts_with("addx") {
            Instr::Addx(value.split_once("addx ").unwrap().1.parse().unwrap())
        } else if value == "noop" {
            Instr::Noop
        } else {
            panic!("Invalid instruction {value}");
        }
    }
}

pub fn generator(input: &str) -> Vec<Instr> {
    input.lines().map(Instr::from).collect()
}

pub fn solve_part1_and_print_part2(program: &[Instr]) -> i32 {
    let mut x_reg = 1;
    let mut signal_strength = 0;
    let mut cycle = 0;

    fn measure_signal(x_reg: i32, cycle: i32) -> i32 {
        if cycle == 20 || (cycle - 20) % 40 == 0 {
            x_reg * cycle
        } else {
            0
        }
    }

    fn draw_pixel(x_reg: i32, cycle: i32) {
        let x_pos = cycle % 40;
        if x_pos == 0 {
            println!();
        }
        let sprite_min = x_reg - 1;
        let sprite_max = x_reg + 1;

        if x_pos >= sprite_min && x_pos <= sprite_max {
            print!("#");
        } else {
            print!(".");
        }
    }

    for instr in program.iter() {
        match instr {
            Instr::Addx(value) => {
                draw_pixel(x_reg, cycle);
                cycle += 1;
                signal_strength += measure_signal(x_reg, cycle);

                draw_pixel(x_reg, cycle);
                cycle += 1;
                signal_strength += measure_signal(x_reg, cycle);
                x_reg += value;
            }
            Instr::Noop => {
                draw_pixel(x_reg, cycle);
                cycle += 1;
                signal_strength += measure_signal(x_reg, cycle);
            }
        };
    }

    signal_strength
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2022/day10_test.txt");

    #[test]
    fn test_part1() {
        let program = generator(INPUT);
        assert_eq!(13140, solve_part1_and_print_part2(&program));
    }
}
