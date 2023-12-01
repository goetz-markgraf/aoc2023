use std::fmt::Debug;

pub fn solution<I, O>(
    input_file: &str,
    parse: fn(Vec<String>) -> I,
    solve: fn(I) -> O,
    expected: O,
) -> bool
    where
        O: PartialEq + Debug,
{
    let long_file_name = format!("input/{}.txt", input_file);
    let file_content = match std::fs::read_to_string(&long_file_name) {
        Ok(content) => content,
        Err(_) => {
            println!("Could not read input file {}", &long_file_name);
            return false;
        }
    };

    let strings = file_content
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let data_structure = parse(strings);
    let result = solve(data_structure);

    if result == expected {
        println!("{}: Correct Result {:?}", input_file, result);
        true
    } else {
        println!("{}: Expected {:?}, got {:?}", input_file, expected, result);
        false
    }
}

// a function the takes a vector of strings and
// returns the same vector without changing the strings
pub fn parse_to_strings(strings: Vec<String>) -> Vec<String> {
    strings
}

pub trait ReverseString {
    fn reverse(&self) -> String;
}

impl ReverseString for String {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}

impl ReverseString for &str {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}
