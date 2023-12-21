use advent_of_code_2023::solution_both_lines;

mod day1;
mod day10;
mod day11;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    solution_both_lines("day1", day1::solve1, day1::solve2, 55130, 54985);
    solution_both_lines("day2", day2::solve1, day2::solve2, 2101, 58269);
    solution_both_lines("day3", day3::solve1, day3::solve2, 553079, 84363105);
    solution_both_lines("day4", day4::solve1, day4::solve2, 23028, 9236992);
    solution_both_lines("day6", day6::solve1, day6::solve2, Ok(131376), Ok(34123437));
    solution_both_lines("day7", day7::solve1, day7::solve2, 251121738, 251421071);
    solution_both_lines("day8", day8::solve1, day8::solve2, 18023, 14449445933179);
    solution_both_lines("day9", day9::solve1, day9::solve2, 1987402313, 900);
    solution_both_lines("day10", day10::solve1, day10::solve2, 6909, 461);
    solution_both_lines(
        "day11",
        day11::solve1,
        day11::solve2,
        10228230,
        447073334102,
    );
    solution_both_lines("day15", day15::solve1, day15::solve2, 510801, 212763);
}
