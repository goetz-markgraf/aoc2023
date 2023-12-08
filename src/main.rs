extern crate core;

use advent_of_code_2023::solution_both_lines;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    solution_both_lines("day1", day1::solve1, day1::solve2, 55130, 54985);
    solution_both_lines("day2", day2::solve1, day2::solve2, 2101, 58269);
    solution_both_lines("day3", day3::solve1, day3::solve2, 553079, 84363105);
    solution_both_lines("day4", day4::solve1, day4::solve2, 23028, 9236992);
    solution_both_lines("day6", day6::solve1, day6::solve2, Ok(131376), Ok(34123437));
}
