// day 5

#[derive(Debug, PartialEq, Clone)]
struct Mapping {
    offset: i64,
    begin: i64,
    end: i64,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    message: String,
}

impl Error {
    fn from(message: &str) -> Error {
        Error {
            message: format!("Invalid Input: {}", message),
        }
    }
}

fn str_to_i64(input: &str) -> Result<i64, Error> {
    input.parse().map_err(|_| Error::from(input))
}

fn parse_mapping_line(line: &str) -> Result<Mapping, Error> {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 3 {
        return Err(Error::from(line));
    }
    let part1: i64 = str_to_i64(parts[0])?;
    let part2: i64 = str_to_i64(parts[1])?;
    let part3: i64 = str_to_i64(parts[2])?;

    Ok(Mapping {
        offset: part1 - part2,
        begin: part2,
        end: part2 + part3,
    })
}

fn parse_mappings(input: &Vec<String>) -> Result<(Vec<String>, Vec<Mapping>), Error> {
    let title = input[0].clone();
    let mut input = input[1..].to_vec();

    if !title.ends_with("map:") {
        return Err(Error::from(&title));
    }
    let mut mappings: Vec<Mapping> = Vec::new();

    while !input.is_empty() && !input[0].is_empty() {
        let line = input[0].clone();
        input = input[1..].to_vec();
        let mapping = parse_mapping_line(&line)?;
        mappings.push(mapping);
    }

    if input.is_empty() {
        Ok((input, mappings))
    } else {
        Ok((input[1..].to_vec(), mappings))
    }
}

fn parse_input(input: Vec<String>) -> Result<(Vec<i64>, Vec<Vec<Mapping>>), Error> {
    let mut input = input;
    let mut result: Vec<Vec<Mapping>> = Vec::new();

    let seeds = input[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().map_err(|_| Error::from(x)))
        .collect::<Result<Vec<i64>, Error>>()?;

    input = input[2..].to_vec();

    while !input.is_empty() {
        let (new_input, mappings) = parse_mappings(&input)?;
        input = new_input;
        result.push(mappings);
    }

    Ok((seeds, result))
}

fn transform(input: i64, mappings: Vec<Mapping>) -> i64 {
    match mappings
        .into_iter()
        .find(|m| input >= m.begin && input < m.end)
    {
        Some(mapping) => input + mapping.offset,
        None => input,
    }
}

fn expand_seeds(seeds: &Vec<i64>) -> Vec<i64> {
    let seed_ranges: Vec<(i64, i64)> = seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    seed_ranges
        .into_iter()
        .flat_map(|(begin, end)| begin..(begin + end - 1))
        .collect()
}

pub fn solve1(input: Vec<String>) -> Result<i64, Error> {
    let (seeds, mappings) = parse_input(input)?;

    let result = seeds
        .iter()
        .map(|seed| {
            mappings
                .clone()
                .into_iter()
                .fold(*seed, |acc, mappings| transform(acc, mappings.clone()))
        })
        .min();

    match result {
        Some(result) => Ok(result),
        None => Err(Error::from("No result")),
    }
}

pub fn solve2(input: Vec<String>) -> Result<i64, Error> {
    let (seeds, mappings) = parse_input(input)?;
    let seeds = expand_seeds(&seeds);

    let result = seeds
        .iter()
        .map(|seed| {
            mappings
                .clone()
                .into_iter()
                .fold(*seed, |acc, mappings| transform(acc, mappings.clone()))
        })
        .min();

    match result {
        Some(result) => Ok(result),
        None => Err(Error::from("No result")),
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use crate::day5::{parse_mapping_line, solve1, solve2, Mapping};

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day5_test", solve1, Ok(35));
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day5", solve1, Ok(331445006));
        assert!(result)
    }

    #[test]
    fn test_solve2_test() {
        let result = solution_lines("day5_test", solve2, Ok(46));
        assert!(result)
    }

    // #[test]
    fn test_solve2() {
        let result = solution_lines("day5", solve2, Ok(0));
        assert!(result)
    }

    #[test]
    fn test_mapping() {
        let result = parse_mapping_line("0 15 37").unwrap();
        assert_eq!(
            result,
            Mapping {
                offset: -15,
                begin: 15,
                end: 52,
            }
        );
    }

    #[test]
    fn test_mapping_invalid() {
        let result = parse_mapping_line("0 15").unwrap_err();
        assert_eq!(result.message, "Invalid Input: 0 15");
    }

    #[test]
    fn test_mapping_invalid2() {
        let result = parse_mapping_line("0 15 a").unwrap_err();
        assert_eq!(result.message, "Invalid Input: a");
    }

    #[test]
    fn test_parse_mappings() {
        let input = vec![
            "a-to-b map:".to_string(),
            "0 15 37".to_string(),
            "1 15 37".to_string(),
            "2 15 37".to_string(),
            "".to_string(),
            "b-to-c map:".to_string(),
            "0 16 37".to_string(),
            "1 16 37".to_string(),
            "2 16 37".to_string(),
        ];
        let result = super::parse_mappings(&input).unwrap();
        assert_eq!(
            result.0,
            vec![
                "b-to-c map:".to_string(),
                "0 16 37".to_string(),
                "1 16 37".to_string(),
                "2 16 37".to_string(),
            ]
        );
        assert_eq!(
            result.1,
            vec![
                Mapping {
                    offset: -15,
                    begin: 15,
                    end: 52,
                },
                Mapping {
                    offset: -14,
                    begin: 15,
                    end: 52,
                },
                Mapping {
                    offset: -13,
                    begin: 15,
                    end: 52,
                },
            ]
        );
    }

    #[test]
    fn test_parse_input() {
        let input = vec![
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
        ];
        let (seeds, mappings) = super::parse_input(input).unwrap();
        assert_eq!(seeds, vec![79, 14, 55, 13]);

        assert_eq!(
            mappings[0],
            vec![
                Mapping {
                    offset: -48,
                    begin: 98,
                    end: 100,
                },
                Mapping {
                    offset: 2,
                    begin: 50,
                    end: 98,
                },
            ]
        );
    }
}
