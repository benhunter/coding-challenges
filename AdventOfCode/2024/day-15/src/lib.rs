use std::str::FromStr;

use util::{parse_grid_chars, Coord, ParseError};

pub fn solve_part1(input: &str) -> Result<i64, String> {
    let warehouse = input.parse::<Warehouse>();
    todo!()
}

pub fn solve_part2(input: &str) -> Result<i64, String> {
    todo!()
}

// #[derive(Debug, PartialEq, Clone, Default)]
// struct Robot {
//     position: Coord,
//     start_position: Coord,
// }

#[derive(Debug, PartialEq, Clone, Default)]
struct Warehouse {
    tiles: Vec<Vec<char>>,
    moves: String,
    robot: Coord,
}

impl Warehouse {
    fn move_robot(&mut self, c: char) {
        // println!("Move {}:", c);
        match c {
            '^' => self.up(),
            'v' => self.down(),
            '<' => self.left(),
            '>' => self.right(),
            _ => panic!("Invalid move: {}", c),
        }
        // self.visualize();
    }

    fn up(&mut self) {
        if self.robot.y == 0 {
            return;
        } else if self.tiles[self.robot.y as usize - 1][self.robot.x as usize] == '#' {
            return;
        }
        let new_y = self.robot.y - 1;
        if new_y >= 0 {
            self.robot.y = new_y;
        }
    }

    fn down(&mut self) {
        if self.robot.y == self.tiles.len() as i64 {
            return;
        } else if self.tiles[self.robot.y as usize + 1][self.robot.x as usize] == '#' {
            return;
        }
        let new_y = self.robot.y + 1;
        if new_y < self.tiles.len() as i64 {
            self.robot.y = new_y;
        }
    }

    fn left(&mut self) {
        // is left a wall? '#' or out of bounds?
        if self.robot.x == 0 {
            return;
        } else if self.tiles[self.robot.y as usize][self.robot.x as usize - 1] == '#' {
            return;
        }
        let new_x = self.robot.x - 1;
        if new_x >= 0 {
            self.robot.x = new_x;
        }
    }

    fn right(&mut self) {
        if self.robot.x == self.tiles[0].len() as i64 {
            return;
        } else if self.tiles[self.robot.y as usize][self.robot.x as usize + 1] == '#' {
            return;
        }
        let new_x = self.robot.x + 1;
        if new_x < self.tiles[0].len() as i64 {
            self.robot.x = new_x;
        }
    }

    fn visualize(&self) -> String {
        // mark robot position as '@'
        let mut s = self.tiles.iter().enumerate().map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| {
                    if x == self.robot.x as usize && y == self.robot.y as usize {
                        '@'
                    } else {
                        *c
                    }
                })
                .collect::<String>()
        }).collect::<Vec<String>>().join("\n");
        s
    }
}

impl FromStr for Warehouse {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split("\n\n").collect();
        let mut tiles = parse_grid_chars(tokens[0])?;
        let moves = tokens[1].trim().to_string();
        let mut robot = Coord::new(0, 0);
        for (r, row) in tiles.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if *col == '@' {
                    robot = Coord::new(c as i64, r as i64);
                }
            }
        }
        tiles[robot.y as usize][robot.x as usize] = '.';
        Ok(Warehouse { tiles, moves, robot })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let warehouse = input.parse::<Warehouse>().unwrap();

        let expected_rows = 8;
        assert_eq!(warehouse.tiles.len(), expected_rows);
        let expected_cols = 8;
        assert_eq!(warehouse.tiles[0].len(), expected_cols);
        let expected_moves = "<^^>>>vv<v>>v<<";
        assert_eq!(warehouse.moves, expected_moves);
        let expected_robot = Coord::new(2, 2);
        assert_eq!(warehouse.robot, expected_robot);
        Ok(())
    }

    #[test]
    fn test_example() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let mut warehouse = input.parse::<Warehouse>().unwrap();

        warehouse.move_robot('<');

        let expected_robot = Coord::new(2, 2);
        assert_eq!(warehouse.robot, expected_robot);

        let actual = warehouse.visualize();
        let expected = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        assert_eq!(actual, expected);

        warehouse.move_robot('^');
        let actual = warehouse.visualize();
        let expected = r"########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        assert_eq!(actual, expected);

        warehouse.move_robot('^');
        let actual = warehouse.visualize();
        let expected = r"########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        assert_eq!(actual, expected);

        warehouse.move_robot('>');
        let actual = warehouse.visualize();
        let expected = r"########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        assert_eq!(actual, expected);

        Ok(())
    }

    // #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let warehouse = input.parse::<Warehouse>().unwrap();
        // let expected = Warehouse { ..Default::default() };
        // assert_eq!(warehouse, expected);
        Ok(())
    }

    // #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_part2() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part2(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_solve_part2() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }
}
