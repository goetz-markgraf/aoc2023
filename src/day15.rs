fn hash_str(s: &str) -> i32 {
    let mut hash = 0;

    for c in s.chars() {
        hash = ((hash + c as i32) * 17) % 256
    }

    hash
}

pub fn solve1(lines: Vec<String>) -> i32 {
    let line = &lines[0];
    let parts = line.split(',').map(|s| hash_str(s)).sum();
    parts
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use super::*;

    #[test]
    fn test_hash_str() {
        assert_eq!(hash_str("HASH"), 52);
    }

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day15_test", solve1, 1320);
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day15", solve1, 510801);
        assert!(result)
    }
}
