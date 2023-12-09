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

fn find_start_nodes(nodes_map: &HashMap<String, Nodes>) -> Vec<String> {
    nodes_map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.clone())
        .collect()
}

fn is_finished(node_names: &Vec<String>) -> bool {
    node_names.iter().all(|n| n.ends_with("Z"))
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

pub fn solve2(lines: Vec<String>) -> i32 {
    let mut pattern = lines[0].chars().into_iter().cycle();

    let node_lines = lines[2..].to_vec();
    let nodes_map = lines_to_nodes_map(&node_lines);
    let mut count = 0;
    let mut nodes = find_start_nodes(&nodes_map);
    while !is_finished(&nodes) {
        let direction = pattern.next().unwrap();
        nodes = nodes
            .iter()
            .map(|n| {
                let this_node = nodes_map.get(n).unwrap();
                match direction {
                    'L' => this_node.left.clone(),
                    'R' => this_node.right.clone(),
                    _ => panic!("Unknown direction {}", direction),
                }
            })
            .collect::<Vec<String>>();
        count += 1
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
    fn test_solve2_test() {
        let result = solution_lines("day8_test2", solve2, 6);
        assert!(result)
    }

    #[test]
    fn test_solve2() {
        let result = solution_lines("day8", solve2, 0);
        assert!(result)
    }

    #[test]
    fn test_find_start_nodes() {
        let lines = vec!["AA = (B, C)", "B = (D, E)", "CA = (F, G)", "D = (H, I)"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let nodes_map = lines_to_nodes_map(&lines);
        let mut start_nodes = find_start_nodes(&nodes_map);
        start_nodes.sort();
        assert_eq!(start_nodes, vec!["AA".to_string(), "CA".to_string()]);
    }

    #[test]
    fn test_is_not_finished() {
        let node_names = vec!["AAA".to_string(), "ZZZ".to_string()];
        assert!(!is_finished(&node_names));
    }

    #[test]
    fn test_is_finished() {
        let node_names = vec!["AAZ".to_string(), "ZZZ".to_string()];
        assert!(is_finished(&node_names));
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
