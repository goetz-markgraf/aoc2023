use advent_of_code_2023::Parse;

fn find_diffs(input: &Vec<i64>) -> Vec<i64> {
    let mut diffs = Vec::new();
    for i in 1..input.len() {
        diffs.push(input[i] - input[i - 1]);
    }
    diffs
}

fn is_all_zero(diff: &Vec<i64>) -> bool {
    diff.iter().all(|&x| x == 0)
}

fn find_all_diffs(line: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut all_diffs: Vec<Vec<i64>> = Vec::new();
    all_diffs.push(line.clone());
    let mut input = line;
    let mut diffs = find_diffs(input);
    all_diffs.push(diffs.clone());
    while !is_all_zero(&diffs) {
        input = &diffs;
        diffs = find_diffs(input);
        all_diffs.push(diffs.clone());
    }

    all_diffs
}

fn add_numbers_last(mut lines: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    lines.reverse();
    let mut add: i64 = 0;
    for line in &mut lines {
        add = line.last().unwrap() + add;
        line.push(add);
    }

    lines
}

fn add_numbers_first(mut lines: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    lines.reverse();
    let mut sub: i64 = 0;
    for line in &mut lines {
        sub = line.first().unwrap() - sub;
        line.insert(0, sub);
    }

    lines
}

fn find_next_number(line: &Vec<i64>) -> i64 {
    let diffs = find_all_diffs(line);
    let diffs = add_numbers_last(diffs);
    diffs.last().unwrap().last().unwrap().clone()
}

fn find_first_number(line: &Vec<i64>) -> i64 {
    let diffs = find_all_diffs(line);
    let diffs = add_numbers_first(diffs);
    diffs.last().unwrap().first().unwrap().clone()
}

pub fn solve1(lines: Vec<String>) -> i64 {
    lines
        .iter()
        .map(|line| {
            line.split_to_numbers()
                .iter()
                .map(|&x| x as i64)
                .collect::<Vec<i64>>()
        })
        .map(|line| find_next_number(&line))
        .sum()
}

pub fn solve2(lines: Vec<String>) -> i64 {
    lines
        .iter()
        .map(|line| {
            line.split_to_numbers()
                .iter()
                .map(|&x| x as i64)
                .collect::<Vec<i64>>()
        })
        .map(|line| find_first_number(&line))
        .sum()
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use super::*;

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day9_test", solve1, 114);
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day9", solve1, 1987402313);
        assert!(result)
    }

    #[test]
    fn test_solve2_test() {
        let result = solution_lines("day9_test", solve2, 2);
        assert!(result)
    }

    #[test]
    fn test_solve2() {
        let result = solution_lines("day9", solve2, 900);
        assert!(result)
    }

    #[test]
    fn test_find_diffs() {
        let input = vec![1, 2, 3, 4, 7, 8, 9, 10, 11, 14];
        let diffs = find_diffs(&input);
        assert_eq!(diffs, vec![1, 1, 1, 3, 1, 1, 1, 1, 3]);
    }

    #[test]
    fn test_find_all_diffs() {
        let input = vec![0, 3, 6, 9, 12, 15];
        let all_diffs = find_all_diffs(&input);
        assert_eq!(
            all_diffs,
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_add_numbers_last() {
        let input = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ];
        let output = add_numbers_last(input);
        assert_eq!(
            output,
            vec![
                vec![0, 0, 0, 0, 0],
                vec![3, 3, 3, 3, 3, 3],
                vec![0, 3, 6, 9, 12, 15, 18],
            ]
        );
    }

    #[test]
    fn test_find_numbers_first() {
        let input = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ];
        let output = add_numbers_first(input);
        assert_eq!(
            output,
            vec![
                vec![0, 0, 0, 0, 0],
                vec![3, 3, 3, 3, 3, 3],
                vec![-3, 0, 3, 6, 9, 12, 15],
            ]
        );
    }

    #[test]
    fn test_find_next_number() {
        let input = vec![0, 3, 6];
        let next_number = find_next_number(&input);
        assert_eq!(next_number, 9);
    }

    #[test]
    fn test_find_first_number() {
        let input = vec![0, 3, 6];
        let next_number = find_first_number(&input);
        assert_eq!(next_number, -3);
    }
}
