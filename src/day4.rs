// day 4

use advent_of_code_2023::Parse;

fn count_winnings(line: &str) -> i32 {
    let split_all: Vec<&str> = line.split(": ").collect();
    let split_stacks: Vec<&str> = split_all[1].split(" | ").collect();
    let winning_cards = split_stacks[0].split_to_numbers();
    let played_cards = split_stacks[1].split_to_numbers();

    let count = winning_cards.iter()
        .filter(|&card| played_cards.contains(card))
        .count();

    count as i32
}

pub fn solve1(lines: Vec<String>) -> i32 {
    lines.iter()
        .map(|line| count_winnings(line))
        .map(|score| if score > 0 { 1 << (score - 1) } else { 0 })
        .sum()
}

pub fn solve2(lines: Vec<String>) -> i32 {
    let mut count_lines: Vec<i32> = vec![1; lines.len()];

    for (index, line) in lines.iter().enumerate() {
        let count = count_winnings(&line);
        for i in 1..=count {
            let pos = index + i as usize;
            if pos < lines.len() {
                count_lines[pos] += count_lines[index];
            }
        }
    }
    count_lines.iter().sum()
}


#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use crate::day4::{count_winnings, solve1, solve2};

    #[test]
    fn test_count_winnings() {
        let result = count_winnings("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(result, 4)
    }

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day4_test", solve1, 13);
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day4", solve1, 23028);
        assert!(result)
    }

    #[test]
    fn test_solve2_test() {
        let result = solution_lines("day4_test", solve2, 30);
        assert!(result)
    }

    #[test]
    fn test_solve2() {
        let result = solution_lines("day4", solve2, 9236992);
        assert!(result)
    }
}
