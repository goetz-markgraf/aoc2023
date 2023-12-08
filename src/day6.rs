// day 6

fn calc_distance(power: i64, duration: i64) -> i64 {
    // function distance = power * (duration - power)
    return power * (duration - power);
}

fn find_powers_over_record(distance: i64, record: i64) -> i64 {
    (1..=distance)
        .filter(|&power| calc_distance(power, distance) > record)
        .count() as i64
}

pub fn solve1(input: Vec<String>) -> Result<i64, String> {
    if input.len() != 2 {
        return Err("input too long or too short".to_string());
    }

    let distance: Vec<i64> = input[0]
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();
    let records: Vec<i64> = input[1]
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    let races = distance.iter().zip(records.iter());
    let winnings: i64 = races
        .map(|(d, r)| find_powers_over_record(*d, *r))
        .product();

    Ok(winnings)
}

pub fn solve2(input: Vec<String>) -> Result<i64, String> {
    if input.len() != 2 {
        return Err("input too long or too short".to_string());
    }

    let distance: i64 = input[0]
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, x| acc + x)
        .parse::<i64>()
        .map_err(|_| "could not parse distance".to_string())?;
    let records: i64 = input[1]
        .split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, x| acc + x)
        .parse::<i64>()
        .map_err(|_| "could not parse record".to_string())?;

    let winnings = find_powers_over_record(distance, records);

    Ok(winnings)
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use super::{calc_distance, find_powers_over_record};

    #[test]
    fn test_calc_distance() {
        assert_eq!(calc_distance(1, 7), 6);
        assert_eq!(calc_distance(2, 7), 10);
    }

    #[test]
    fn test_find() {
        assert_eq!(find_powers_over_record(7, 9), 4);
        assert_eq!(find_powers_over_record(15, 40), 8);
        assert_eq!(find_powers_over_record(30, 200), 9)
    }

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day6_test", super::solve1, Ok(288));
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day6", super::solve1, Ok(131376));
        assert!(result)
    }

    #[test]
    fn test_solve2_test() {
        let result = solution_lines("day6_test", super::solve2, Ok(71503));
        assert!(result)
    }

    #[test]
    fn test_solve2() {
        let result = solution_lines("day6", super::solve2, Ok(34123437));
        assert!(result)
    }
}
