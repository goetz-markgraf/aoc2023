#[derive(Debug, PartialEq)]
struct Field(Vec<Vec<(char, bool)>>);

#[derive(Debug, PartialEq)]
struct Pos(usize, usize);

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Field {
    fn parse_input(input: &Vec<String>) -> Field {
        Field(
            input
                .iter()
                .map(|l| l.chars().map(|c| (c, false)).collect())
                .collect(),
        )
    }

    fn find_start_field(&self) -> Pos {
        for (y, row) in self.0.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let p = Pos(x, y);
                if (*c).0 == 'S' {
                    return p;
                }
            }
        }
        panic!("No start found");
    }

    fn at(&self, pos: &Pos) -> char {
        let Pos(x, y) = pos;
        self.0[*y][*x].0
    }

    fn set_at(&mut self, pos: &Pos, in_loop: bool) {
        let Pos(x, y) = pos;
        self.0[*y][*x].1 = in_loop
    }

    fn do_step(&self, pos: &Pos, dir: &Direction) -> (Pos, Direction) {
        match dir {
            Direction::Up => {
                if self.at(&Pos(pos.0, pos.1 - 1)) == '|' {
                    (Pos(pos.0, pos.1 - 1), Direction::Up)
                } else if self.at(&Pos(pos.0, pos.1 - 1)) == 'F' {
                    (Pos(pos.0, pos.1 - 1), Direction::Right)
                } else {
                    (Pos(pos.0, pos.1 - 1), Direction::Left)
                }
            }
            Direction::Down => {
                if self.at(&Pos(pos.0, pos.1 + 1)) == '|' {
                    (Pos(pos.0, pos.1 + 1), Direction::Down)
                } else if self.at(&Pos(pos.0, pos.1 + 1)) == 'J' {
                    (Pos(pos.0, pos.1 + 1), Direction::Left)
                } else {
                    (Pos(pos.0, pos.1 + 1), Direction::Right)
                }
            }
            Direction::Left => {
                if self.at(&Pos(pos.0 - 1, pos.1)) == '-' {
                    (Pos(pos.0 - 1, pos.1), Direction::Left)
                } else if self.at(&Pos(pos.0 - 1, pos.1)) == 'F' {
                    (Pos(pos.0 - 1, pos.1), Direction::Down)
                } else {
                    (Pos(pos.0 - 1, pos.1), Direction::Up)
                }
            }
            Direction::Right => {
                if self.at(&Pos(pos.0 + 1, pos.1)) == '-' {
                    (Pos(pos.0 + 1, pos.1), Direction::Right)
                } else if self.at(&Pos(pos.0 + 1, pos.1)) == 'J' {
                    (Pos(pos.0 + 1, pos.1), Direction::Up)
                } else {
                    (Pos(pos.0 + 1, pos.1), Direction::Down)
                }
            }
        }
    }
    fn find_start_direction(&self, pos: &Pos) -> Direction {
        let Pos(x, y) = pos;
        if y > &0 && "7|F".contains(self.0[y - 1][*x].0) {
            Direction::Up
        } else if y < &(self.0.len() - 1) && "L|J".contains(self.0[y + 1][*x].0) {
            Direction::Down
        } else if x > &0 && "L-F".contains(self.0[*y][x - 1].0) {
            Direction::Left
        } else if x < &(self.0[*y].len() - 1) && "J-7".contains(self.0[*y][x + 1].0) {
            Direction::Right
        } else {
            panic!("No start direction found");
        }
    }
}

pub fn solve1(lines: Vec<String>) -> i64 {
    let field = Field::parse_input(&lines);
    let mut pos = field.find_start_field();
    let mut steps = 0i64;
    let mut dir = field.find_start_direction(&pos);

    while field.at(&pos) != 'S' || steps == 0 {
        (pos, dir) = field.do_step(&pos, &dir);
        steps += 1;
    }

    steps / 2
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use super::{solve1, Direction, Field, Pos};

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
        let field = Field::parse_input(&input);
        assert_eq!(field.0.len(), 5);
        assert_eq!(field.0[0].len(), 5);
        assert_eq!(field.0[0][0].0, '7');
        assert_eq!(field.0[2][0].0, 'S');
        assert_eq!(field.0[1][0].0, '.');
        assert_eq!(field.0[3][3].0, '-');
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
        let field = Field::parse_input(&input);
        let pos = field.find_start_field();
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
        let field = Field::parse_input(&input);
        let pos = field.find_start_field();
        let dir = field.find_start_direction(&pos);
        assert_eq!(dir, Direction::Down)
    }
}
