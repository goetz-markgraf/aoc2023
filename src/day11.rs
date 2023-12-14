#[derive(Debug, Clone, PartialEq)]
struct Galaxy(usize, usize);

fn find_galaxies(input: &Vec<String>) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy(x, y));
            }
        }
    }
    galaxies
}

fn find_empty_rows(input: &Vec<String>) -> Vec<usize> {
    fn all_dot(s: &String) -> bool {
        s.chars().all(|c| c == '.')
    }

    input
        .into_iter()
        .enumerate()
        .filter(|(_, line)| all_dot(line))
        .map(|(i, _)| i)
        .collect()
}

fn find_empty_columns(input: &Vec<String>) -> Vec<usize> {
    fn all_dot(input: &Vec<String>, x: usize) -> bool {
        input.iter().all(|line| line.chars().nth(x).unwrap() == '.')
    }

    (0..input[0].len())
        .into_iter()
        .filter(|x| all_dot(input, *x))
        .collect()
}

fn enlarge_galaxy_row(galaxies: Vec<Galaxy>, row: usize) -> Vec<Galaxy> {
    galaxies
        .into_iter()
        .map(|g| if g.1 > row { Galaxy(g.0, g.1 + 1) } else { g })
        .collect()
}

fn enlarge_galaxy_column(galaxies: Vec<Galaxy>, col: usize) -> Vec<Galaxy> {
    galaxies
        .into_iter()
        .map(|g| if g.0 > col { Galaxy(g.0 + 1, g.1) } else { g })
        .collect()
}

fn enlarge_all_galaxies(
    galaxies: Vec<Galaxy>,
    rows: &Vec<usize>,
    cols: &Vec<usize>,
) -> Vec<Galaxy> {
    let mut rows = rows.clone();
    rows.sort_by(|a, b| b.cmp(a));
    let mut cols = cols.clone();
    cols.sort_by(|a, b| b.cmp(a));

    let mut galaxies = galaxies;
    for row in rows {
        galaxies = enlarge_galaxy_row(galaxies, row)
    }
    for col in cols {
        galaxies = enlarge_galaxy_column(galaxies, col)
    }

    galaxies
}

fn calculate_distance(a: &Galaxy, b: &Galaxy) -> usize {
    let dx = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let dy = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    return dx + dy;
}

fn find_all_distances(galaxies: &Vec<Galaxy>) -> Vec<usize> {
    let mut distances: Vec<usize> = Vec::new();
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(i + 1) {
            distances.push(calculate_distance(g1, g2));
        }
    }
    distances
}

pub fn solve1(input: Vec<String>) -> i64 {
    let galaxies = find_galaxies(&input);
    let empty_rows = find_empty_rows(&input);
    let empy_cols = find_empty_columns(&input);
    let galaxies = enlarge_all_galaxies(galaxies, &empty_rows, &empy_cols);

    let distances = find_all_distances(&galaxies);

    distances.iter().map(|d| *d as i64).sum()
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::{s, solution_lines};

    use crate::day11::calculate_distance;
    use crate::day11::enlarge_galaxy_column;
    use crate::day11::enlarge_galaxy_row;
    use crate::day11::find_empty_columns;
    use crate::day11::find_empty_rows;
    use crate::day11::find_galaxies;
    use crate::day11::solve1;
    use crate::day11::Galaxy;

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day11_test", solve1, 374);
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day11", solve1, 10228230);
        assert!(result)
    }

    #[test]
    fn test_find_galaxies() {
        let input = vec![s!("...#......"), s!(".........."), s!("#...#.....")];
        let result = find_galaxies(&input);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Galaxy(3, 0));
        assert_eq!(result[1], Galaxy(0, 2));
        assert_eq!(result[2], Galaxy(4, 2));
    }

    #[test]
    fn test_file_empty_rows() {
        let input = vec![
            s!("...#......"),
            s!(".........."),
            s!("#...#....."),
            s!(".........."),
        ];
        let result = find_empty_rows(&input);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn test_file_empty_columns() {
        let input = vec![
            s!("...#......"),
            s!(".........."),
            s!("#...#....."),
            s!(".........."),
        ];
        let result = find_empty_columns(&input);
        assert_eq!(result, vec![1, 2, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_enlarge_galaxy_row() {
        let input = vec![Galaxy(3, 0), Galaxy(0, 2), Galaxy(4, 2)];
        let result = enlarge_galaxy_row(input, 1);
        assert_eq!(result[0], Galaxy(3, 0));
        assert_eq!(result[1], Galaxy(0, 3));
        assert_eq!(result[2], Galaxy(4, 3));

        let input = vec![Galaxy(3, 0), Galaxy(0, 2), Galaxy(4, 2)];
        let result = enlarge_galaxy_row(input, 4);
        assert_eq!(result[0], Galaxy(3, 0));
        assert_eq!(result[1], Galaxy(0, 2));
        assert_eq!(result[2], Galaxy(4, 2));
    }

    #[test]
    fn test_enlarge_galaxy_colum() {
        let input = vec![Galaxy(3, 0), Galaxy(0, 2), Galaxy(4, 2)];
        let result = enlarge_galaxy_column(input, 1);
        assert_eq!(result[0], Galaxy(4, 0));
        assert_eq!(result[1], Galaxy(0, 2));
        assert_eq!(result[2], Galaxy(5, 2));

        let input = vec![Galaxy(3, 0), Galaxy(0, 2), Galaxy(4, 2)];
        let result = enlarge_galaxy_column(input, 4);
        assert_eq!(result[0], Galaxy(3, 0));
        assert_eq!(result[1], Galaxy(0, 2));
        assert_eq!(result[2], Galaxy(4, 2));
    }

    #[test]
    fn test_calculate_distance() {
        let g1 = Galaxy(1, 6);
        let g2 = Galaxy(5, 11);
        let result = calculate_distance(&g1, &g2);
        assert_eq!(result, 9);
    }
}
