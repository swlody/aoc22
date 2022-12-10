mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

aoc_main::main! {
    year 2022;
    day1 : generator => solve_part1, solve_part2;
    day2 => solve_part1, solve_part2;
    day3 => solve_part1, solve_part2;
    day4 : generator => solve_part1, solve_part2;
    day5 => solve_part1, solve_part2;
    day6 => solve_part1, solve_part2;
    day10 : generator => solve_part1_and_print_part2;
}
