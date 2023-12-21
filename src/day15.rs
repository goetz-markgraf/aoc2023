use std::str::FromStr;

fn hash_str(s: &str) -> i64 {
    let mut hash = 0;

    for c in s.chars() {
        hash = ((hash + c as i64) * 17) % 256
    }

    hash
}

pub fn solve1(lines: Vec<String>) -> i64 {
    let line = &lines[0];
    let parts = line.split(',').map(|s| hash_str(s)).sum();
    parts
}

#[derive(Debug, PartialEq)]
struct Label {
    label: String,
    value: i64,
}

impl Label {
    fn from_set(cmd: Command) -> Option<Self> {
        match cmd {
            Command::Remove(_) => None,
            Command::Set(label, value) => Some(Label { label, value }),
        }
    }
}

type Box = Vec<Label>;

fn new_box() -> Box {
    Vec::new()
}

fn remove_label(abox: &mut Box, label: &str) {
    abox.retain(|b| b.label != label);
}

fn set_label(abox: &mut Box, label: &str, value: i64) {
    let index = {
        abox.iter()
            .enumerate()
            .find(|(_, b)| b.label == label)
            .map(|(i, _)| i)
    };
    match index {
        None => abox.push(Label {
            label: String::from(label),
            value,
        }),
        Some(i) => {
            abox[i] = Label {
                label: String::from(label),
                value,
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Remove(String),
    Set(String, i64),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('-') {
            Ok(Command::Remove(s.trim_end_matches('-').to_owned()))
        } else {
            let mut split = s.split('=');
            Ok(Command::Set(
                split.next().unwrap().to_owned(),
                split.next().unwrap().parse::<i64>().unwrap(),
            ))
        }
    }
}

fn calculate_box(abox: &Box, box_no: usize) -> i64 {
    if abox.is_empty() {
        return 0;
    }
    let value: i64 = abox
        .iter()
        .enumerate()
        .map(|(slot, label)| (box_no + 1) as i64 * (slot + 1) as i64 * label.value)
        .sum();
    value
}

pub fn solve2(lines: Vec<String>) -> i64 {
    let line = &lines[0];
    let parts: Vec<String> = line.split(',').map(|s| s.to_owned()).collect();

    let mut boxes = Vec::new();
    for _ in 0..256 {
        let abox = new_box();
        boxes.push(abox);
    }

    for part in parts {
        let cmd = Command::from_str(&part).unwrap();
        match cmd {
            Command::Remove(label) => {
                let box_index = hash_str(&label) as usize;
                let abox = &mut boxes[box_index];
                remove_label(abox, &label)
            }
            Command::Set(label, value) => {
                let box_index = hash_str(&label) as usize;
                let abox = &mut boxes[box_index];
                set_label(abox, &label, value)
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| calculate_box(b, i))
        .sum()
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

    #[test]
    fn test_solve2() {
        let result = solution_lines("day15", solve2, 212763);
        assert!(result)
    }

    #[test]
    fn test_solve2_test() {
        let result = solution_lines("day15_test", solve2, 145);
        assert!(result)
    }

    #[test]
    fn test_command_from_str() {
        assert_eq!(
            Command::from_str("ad-").unwrap(),
            Command::Remove("ad".to_owned())
        );
        assert_eq!(
            Command::from_str("asdf=10").unwrap(),
            Command::Set("asdf".to_owned(), 10)
        );
    }
}
