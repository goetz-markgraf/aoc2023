// day2

fn parse_line(line: &str) -> Game {
    let game_line = line.split(": ").collect::<Vec<&str>>();
    let game_split = game_line[0].split(" ").collect::<Vec<&str>>();
    let game_no = game_split[1].parse::<i32>().expect(
        format!("Should be able to parse {} as i32", game_split[1]).as_str()
    );

    let mut blue = 0;
    let mut green = 0;
    let mut red = 0;

    let draws = game_line[1].split("; ").collect::<Vec<&str>>();
    for draw in draws {
        let draw_split = draw.split(", ").collect::<Vec<&str>>();
        for cube in draw_split {
            let cube_split = cube.split(" ").collect::<Vec<&str>>();
            let cube_count = cube_split[0].parse::<i32>().expect(
                format!("Should be able to parse {} as i32", cube_split[0]).as_str()
            );
            let cube_color = cube_split[1];
            match cube_color {
                "blue" => if cube_count > blue { blue = cube_count },
                "green" => if cube_count > green { green = cube_count },
                "red" => if cube_count > red { red = cube_count },
                _ => panic!("Unknown color {}", cube_color)
            }
        }
    }

    Game { game_no, blue, green, red }
}

pub fn solve1(input: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in input {
        let game = parse_line(&line);
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            sum += game.game_no
        }
    }
    sum
}

pub fn solve2(input: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in input {
        let game = parse_line(&line);
        let power = game.red * game.green * game.blue;
        sum += power
    }
    sum
}

#[derive(Debug, PartialEq)]
struct Game {
    game_no: i32,
    blue: i32,
    green: i32,
    red: i32,
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use crate::day2::Game;

    #[test]
    fn test_parse_line() {
        let test = super::parse_line("Game 1: 1 red, 2 blue; 3 green, 4 blue");
        assert_eq!(test, Game { game_no: 1, blue: 4, green: 3, red: 1 });
    }

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day2_test", super::solve1, 8);
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day2", super::solve1, 2101);
        assert!(result)
    }

    #[test]
    fn test_solve2_test() {
        let result = solution_lines("day2_test", super::solve2, 2286);
        assert!(result)
    }

    #[test]
    fn test_solve2() {
        let result = solution_lines("day2", super::solve2, 58269);
        assert!(result)
    }
}
