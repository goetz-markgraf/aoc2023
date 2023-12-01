// day 1

use std::collections::HashMap;

fn solve1(input: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in input {
        // find the first digit in the string
        let first_digit = line
            .chars()
            .find(|c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap() as i32;
        // find the last digit in the string
        let second_digit = line
            .chars()
            .rev()
            .find(|c| c.is_digit(10))
            .unwrap()
            .to_digit(10)
            .unwrap() as i32;
        sum += first_digit * 10 + second_digit;
    }
    sum
}

fn solve2(input: Vec<String>) -> i32 {
    let mut sum = 0;
    let names_to_digit: HashMap<&str, i32> = [
        // ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].iter().cloned().collect();
    let regex_pattern_forward = "(".to_owned() + &names_to_digit.keys()
        .map(|k| format!("{}", k))
        .collect::<Vec<_>>()
        .join("|") + "|\\d)";
    let regex_forwards = regex::Regex::new(&regex_pattern_forward).unwrap();

    let regex_pattern_backwards = "(".to_owned() + &names_to_digit.keys()
        .map(|k| format!("{}", k.chars().rev().collect::<String>()))
        .collect::<Vec<_>>()
        .join("|") + "|\\d)";
    let regex_backwards = regex::Regex::new(&regex_pattern_backwards).unwrap();

    for line in input {
        let rev_line = line.chars().rev().collect::<String>();
        let first_result = regex_forwards.find_iter(&line).next().unwrap();
        let last_result = regex_backwards.find_iter(&rev_line).next().unwrap();

        let first_matched = first_result.as_str();
        let first_digit = convert(&names_to_digit, first_matched);
        let last_matched = last_result.as_str().chars().rev().collect::<String>();
        let last_digit = convert(&names_to_digit, &last_matched);

        let line_result = first_digit * 10 + last_digit;
        sum += line_result
    }

    sum
}

fn convert(names_to_digit: &HashMap<&str, i32>, matched: &str) -> i32 {
    if matched.chars().all(|c| c.is_digit(10)) {
        matched.parse::<i32>().unwrap()
    } else {
        *names_to_digit.get(matched).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::{parse_to_strings, solution};

    use crate::day1::{solve1, solve2};

    // test the functions
    #[test]
    fn test_solve1_test() {
        let test = solution("day1_test", parse_to_strings, solve1, 142);
        assert!(test);
    }

    #[test]
    fn test_solve1() {
        let test = solution("day1", parse_to_strings, solve1, 55130);
        assert!(test);
    }

    #[test]
    fn test_solve2_test() {
        let test = solution("day1_test2", parse_to_strings, solve2, 281);
        assert!(test);
    }

    #[test]
    fn test_solve2() {
        let test = solution("day1", parse_to_strings, solve2, 54985);
        assert!(test);
    }
}
