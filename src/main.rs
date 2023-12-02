use advent_of_code_2023::solution_both_lines;

mod day1;
mod day2;

fn main() {
    solution_both_lines("day1", day1::solve1, day1::solve2, 55130, 54985);
    solution_both_lines("day2", day2::solve1, day2::solve2, 2101, 58269);
}
