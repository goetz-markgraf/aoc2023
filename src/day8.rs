use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
struct Nodes {
    left: String,
    right: String,
}

impl Nodes {
    fn new_from_string(line: &str) -> Self {
        let line = line.trim_start_matches('(').trim_end_matches(')');
        let split = line.split(", ").collect::<Vec<&str>>();
        let left = split[0].to_string();
        let right = split[1].to_string();

        Self { left, right }
    }
}

fn lines_to_nodes_map(lines: &Vec<String>) -> HashMap<String, Nodes> {
    let mut nodes_map = HashMap::new();
    for line in lines {
        let split = line.split(" = ").collect::<Vec<&str>>();
        let label = split[0].to_string();
        let node_str = split[1].to_string();
        let nodes = Nodes::new_from_string(&node_str);
        nodes_map.insert(label, nodes);
    }
    nodes_map
}

pub fn solve1(lines: Vec<String>) -> i32 {
    let mut pattern = lines[0].chars().into_iter().cycle();

    let node_lines = lines[2..].to_vec();
    let nodes_map = lines_to_nodes_map(&node_lines);

    let mut count = 0;
    let mut node = "AAA".to_string();
    while node != "ZZZ" {
        let this_node = nodes_map.get(&node).unwrap();
        let direction = pattern.next().unwrap();
        match direction {
            'L' => node = this_node.left.clone(),
            'R' => node = this_node.right.clone(),
            _ => panic!("Unknown direction {}", direction),
        }
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use super::*;

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day8_test", solve1, 6);
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day8", solve1, 18023);
        assert!(result)
    }

    #[test]
    fn test_nodes_new_from_string() {
        let line = "(A, B)";
        let nodes = Nodes::new_from_string(line);
        assert_eq!(
            nodes,
            Nodes {
                left: "A".to_string(),
                right: "B".to_string(),
            }
        );
    }

    #[test]
    fn test_lines_to_nodes_map() {
        let lines = vec!["A = (B, C)", "B = (D, E)", "C = (F, G)", "D = (H, I)"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let nodes_map = lines_to_nodes_map(&lines);
        assert_eq!(
            nodes_map,
            vec![
                (
                    "A".to_string(),
                    Nodes {
                        left: "B".to_string(),
                        right: "C".to_string(),
                    }
                ),
                (
                    "B".to_string(),
                    Nodes {
                        left: "D".to_string(),
                        right: "E".to_string(),
                    }
                ),
                (
                    "C".to_string(),
                    Nodes {
                        left: "F".to_string(),
                        right: "G".to_string(),
                    }
                ),
                (
                    "D".to_string(),
                    Nodes {
                        left: "H".to_string(),
                        right: "I".to_string(),
                    }
                ),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }
}
