mod day1;
mod day2;
mod day3;

aoc_main::main! {
    year 2022;
    day1 : generator => solve_part1, solve_part2;
    day2 => solve_part1, solve_part2;
    day3 => solve_part1, solve_part2;
}
