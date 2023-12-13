#[derive(Debug, PartialEq)]
struct Field(Vec<Vec<char>>);

#[derive(Debug, PartialEq)]
struct Pos(usize, usize);

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &Vec<String>) -> Field {
    Field(input.iter().map(|l| l.chars().collect()).collect())
}

fn find_start_field(f: &Field) -> Pos {
    for (y, row) in f.0.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                return Pos(x, y);
            }
        }
    }
    panic!("No start found");
}

fn find_start_direction(f: &Field, pos: &Pos) -> Direction {
    let Pos(x, y) = pos;
    if y > &0 && "7|F".contains(f.0[y - 1][*x]) {
        Direction::Up
    } else if y < &(f.0.len() - 1) && "L|J".contains(f.0[y + 1][*x]) {
        Direction::Down
    } else if x > &0 && "L-F".contains(f.0[*y][x - 1]) {
        Direction::Left
    } else if x < &(f.0[*y].len() - 1) && "J-7".contains(f.0[*y][x + 1]) {
        Direction::Right
    } else {
        panic!("No start direction found");
    }
}

fn get_char(f: &Field, pos: &Pos) -> char {
    let Pos(x, y) = pos;
    f.0[*y][*x]
}

fn do_step(field: &Field, pos: &Pos, dir: &Direction) -> (Pos, Direction) {
    match dir {
        Direction::Up => {
            if get_char(&field, &Pos(pos.0, pos.1 - 1)) == '|' {
                (Pos(pos.0, pos.1 - 1), Direction::Up)
            } else if get_char(&field, &Pos(pos.0, pos.1 - 1)) == 'F' {
                (Pos(pos.0, pos.1 - 1), Direction::Right)
            } else {
                (Pos(pos.0, pos.1 - 1), Direction::Left)
            }
        }
        Direction::Down => {
            if get_char(&field, &Pos(pos.0, pos.1 + 1)) == '|' {
                (Pos(pos.0, pos.1 + 1), Direction::Down)
            } else if get_char(&field, &Pos(pos.0, pos.1 + 1)) == 'J' {
                (Pos(pos.0, pos.1 + 1), Direction::Left)
            } else {
                (Pos(pos.0, pos.1 + 1), Direction::Right)
            }
        }
        Direction::Left => {
            if get_char(&field, &Pos(pos.0 - 1, pos.1)) == '-' {
                (Pos(pos.0 - 1, pos.1), Direction::Left)
            } else if get_char(&field, &Pos(pos.0 - 1, pos.1)) == 'F' {
                (Pos(pos.0 - 1, pos.1), Direction::Down)
            } else {
                (Pos(pos.0 - 1, pos.1), Direction::Up)
            }
        }
        Direction::Right => {
            if get_char(&field, &Pos(pos.0 + 1, pos.1)) == '-' {
                (Pos(pos.0 + 1, pos.1), Direction::Right)
            } else if get_char(&field, &Pos(pos.0 + 1, pos.1)) == 'J' {
                (Pos(pos.0 + 1, pos.1), Direction::Up)
            } else {
                (Pos(pos.0 + 1, pos.1), Direction::Down)
            }
        }
    }
}

pub fn solve1(lines: Vec<String>) -> i64 {
    let field = parse_input(&lines);
    let mut pos = find_start_field(&field);
    let mut steps = 0i64;
    let mut dir = find_start_direction(&field, &pos);

    while !(get_char(&field, &pos) == 'S' && steps > 0) {
        (pos, dir) = do_step(&field, &pos, &dir);
        steps += 1;
    }

    steps / 2
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use super::{find_start_direction, find_start_field, parse_input, solve1, Direction, Pos};

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day10_test", solve1, 8);
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day10", solve1, 6909);
        assert!(result)
    }

    #[test]
    fn test_parse_input() {
        let input = vec![
            String::from("7-F7-"),
            String::from(".FJ|7"),
            String::from("SJLL7"),
            String::from("|F--J"),
            String::from("LJ.LJ"),
        ];
        let field = parse_input(&input);
        assert_eq!(field.0.len(), 5);
        assert_eq!(field.0[0].len(), 5);
        assert_eq!(field.0[0][0], '7');
        assert_eq!(field.0[2][0], 'S');
        assert_eq!(field.0[1][0], '.');
        assert_eq!(field.0[3][3], '-');
    }

    #[test]
    fn test_find_start_field() {
        let input = vec![
            String::from("7-F7-"),
            String::from(".FJ|7"),
            String::from("SJLL7"),
            String::from("|F--J"),
            String::from("LJ.LJ"),
        ];
        let field = parse_input(&input);
        let pos = find_start_field(&field);
        assert_eq!(pos, Pos(0, 2))
    }

    #[test]
    fn test_find_start_direction() {
        let input = vec![
            String::from("7-F7-"),
            String::from(".FJ|7"),
            String::from("SJLL7"),
            String::from("|F--J"),
            String::from("LJ.LJ"),
        ];
        let field = parse_input(&input);
        let pos = find_start_field(&field);
        let dir = find_start_direction(&field, &pos);
        assert_eq!(dir, Direction::Down)
    }
}
