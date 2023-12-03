// day 3

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_NUMBER: Regex = Regex::new(r"\d+").unwrap();
    static ref REGEX_SYMBOL: Regex = Regex::new(r"[^a-zA-Z0-9.]").unwrap();
    static ref REGEX_GEAR: Regex = Regex::new(r"\*").unwrap();
}

fn extract_numbers_line(line: &str, line_no: i32) -> Vec<Number> {
    REGEX_NUMBER.find_iter(line).map(|m|
        Number {
            value: m.as_str().parse::<i32>().unwrap(),
            line_number: line_no,
            start: m.start() as i32,
            end: m.end() as i32,
        }
    ).collect()
}

fn extract_symbols_line(line: &str, line_no: i32) -> Vec<Symbol> {
    REGEX_SYMBOL
        .find_iter(line)
        .enumerate()
        .map(|(_, i)| Symbol { line_no, pos: i.start() as i32 })
        .collect()
}

fn extract_gears_line(line: &str, line_no: i32) -> Vec<Symbol> {
    REGEX_GEAR
        .find_iter(line)
        .enumerate()
        .map(|(_, i)| Symbol { line_no, pos: i.start() as i32 })
        .collect()
}

fn extract_lines(lines: &Vec<String>) -> (Vec<Number>, Vec<Symbol>) {
    (lines.iter().enumerate()
         .flat_map(|(line_no, line)| extract_numbers_line(line, line_no as i32))
         .collect(),
     lines.iter().enumerate()
         .flat_map(|(line_no, line)| extract_symbols_line(line, line_no as i32))
         .collect())
}

fn extract_gears(lines: &Vec<String>) -> (Vec<Number>, Vec<Symbol>) {
    (lines.iter().enumerate()
         .flat_map(|(line_no, line)| extract_numbers_line(line, line_no as i32))
         .collect(),
     lines.iter().enumerate()
         .flat_map(|(line_no, line)| extract_gears_line(line, line_no as i32))
         .collect())
}

fn check_part_number(num: &Number, symbol: &Symbol) -> bool {
    symbol.line_no >= num.line_number - 1 && symbol.line_no <= num.line_number + 1
        && symbol.pos >= num.start - 1 && symbol.pos <= num.end
}

pub fn solve1(input: Vec<String>) -> i32 {
    let (numbers, symbols) = extract_lines(&input);

    let mut sum = 0;
    for number in numbers {
        let adjacent =
            symbols.iter().any(|symbol| check_part_number(&number, symbol));
        if adjacent {
            sum += number.value
        }
    }

    sum
}

pub fn solve2(input: Vec<String>) -> i32 {
    let (numbers, gears) = extract_gears(&input);
    let mut sum = 0;

    for gear in gears {
        let adjacent: Vec<Number> = numbers.iter()
            .filter(|number| check_part_number(number, &gear)).cloned().collect();
        if adjacent.len() == 2 {
            sum += adjacent[0].value * adjacent[1].value
        }
    }

    sum
}

#[derive(Debug, PartialEq, Clone)]
struct Number {
    value: i32,
    line_number: i32,
    start: i32,
    end: i32,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    line_no: i32,
    pos: i32,
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use crate::day3::{extract_gears_line, extract_lines, extract_numbers_line, extract_symbols_line, Number, solve1, solve2, Symbol};

    #[test]
    fn test_extract_line() {
        let result = extract_numbers_line("467..114..", 0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Number { value: 467, line_number: 0, start: 0, end: 3 });
        assert_eq!(result[1], Number { value: 114, line_number: 0, start: 5, end: 8 });
    }

    #[test]
    fn test_extract_symbols_line() {
        let result = extract_symbols_line("467.%.114&..", 0);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Symbol { line_no: 0, pos: 4 });
        assert_eq!(result[1], Symbol { line_no: 0, pos: 9 });
    }

    #[test]
    fn test_extract_gear_line() {
        let result = extract_gears_line("467.*.114&..", 0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Symbol { line_no: 0, pos: 4 });
    }

    #[test]
    fn test_extract_lines() {
        let input = vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
        ];
        let result = extract_lines(&input);

        assert_eq!(result.0.len(), 4);
        assert_eq!(result.0[2].value, 35);
        assert_eq!(result.0[2].line_number, 2);
        assert_eq!(result.0[3].end, 9);

        assert_eq!(result.1.len(), 1);
        assert_eq!(result.1[0], Symbol { line_no: 1, pos: 3 })
    }

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day3_test", solve1, 4361);
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day3", solve1, 553079);
        assert!(result)
    }

    #[test]
    fn test_solve2_test() {
        let result = solution_lines("day3_test", solve2, 467835);
        assert!(result)
    }

    #[test]
    fn test_solve2() {
        let result = solution_lines("day3", solve2, 84363105);
        assert!(result)
    }
}
