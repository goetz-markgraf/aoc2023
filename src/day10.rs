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

    fn at_in_loop(&self, pos: &Pos) -> char {
        let Pos(x, y) = pos;
        if self.0[*y][*x].1 {
            self.0[*y][*x].0
        } else {
            ' '
        }
    }

    fn at(&self, pos: &Pos) -> char {
        let Pos(x, y) = pos;
        self.0[*y][*x].0
    }

    fn set_at(&mut self, pos: &Pos, in_loop: bool) {
        let Pos(x, y) = pos;
        self.0[*y][*x].1 = in_loop
    }

    fn do_step(&mut self, pos: &Pos, dir: &Direction) -> (Pos, Direction) {
        self.set_at(pos, true);
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

    fn is_inside(&self, pos: &Pos) -> bool {
        // from the current position to upwards. for every 7, J or - in the loop toggle the state between inside and outside
        let mut inside = false;
        if self.at_in_loop(pos) != ' ' {
            return false;
        }
        let x = pos.0;
        let mut y = pos.1;
        while y > 0 {
            y = y - 1;

            let char = self.at_in_loop(&Pos(x, y));
            if "7J-".contains(char) {
                inside = !inside
            }
        }

        inside
    }

    fn count_all_inside(&self) -> usize {
        let mut count = 0usize;
        for x in 0..self.0[0].len() {
            for y in 0..self.0.len() {
                if self.is_inside(&Pos(x, y)) {
                    count += 1
                }
            }
        }
        count
    }

    fn follow_pipe(&mut self) -> usize {
        let mut pos = self.find_start_field();
        let mut steps = 0;
        let mut dir = self.find_start_direction(&pos);

        while self.at(&pos) != 'S' || steps == 0 {
            (pos, dir) = self.do_step(&pos, &dir);
            steps += 1;
        }

        steps / 2
    }
}

pub fn solve1(lines: Vec<String>) -> usize {
    let mut field = Field::parse_input(&lines);
    field.follow_pipe()
}

pub fn solve2(lines: Vec<String>) -> usize {
    let mut field = Field::parse_input(&lines);
    field.follow_pipe();
    field.count_all_inside()
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use super::{solve1, solve2, Direction, Field, Pos};

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
    fn test_solve2_test() {
        let result = solution_lines("day10_test", solve2, 1);
        assert!(result)
    }

    #[test]
    fn test_solve2() {
        let result = solution_lines("day10", solve2, 461);
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
    fn test_is_inside() {
        let input = vec![
            String::from("7-F7-"),
            String::from(".FJ|7"),
            String::from("SJLL7"),
            String::from("|F--J"),
            String::from("LJ.LJ"),
        ];
        let mut field = Field::parse_input(&input);
        field.follow_pipe();
        let inside = field.is_inside(&Pos(1, 4));
        assert!(!inside)
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
