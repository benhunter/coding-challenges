use std::{io, str::FromStr};

use util::{parse_grid_chars, Coord, ParseError};

pub fn solve_part1(input: &str) -> Result<i64, String> {
    let mut warehouse = input.parse::<Warehouse>()?;
    warehouse.do_moves();
    Ok(warehouse.solve())
}

pub fn solve_part2(input: &str) -> Result<i64, String> {
    todo!()
}

pub fn simulate_large_example() -> Result<(), String> {
    let input = include_str!("../test-large.txt");
    let mut warehouse = input.parse::<Warehouse>().unwrap();
    let moves = warehouse.moves.clone();

    println!("{}", warehouse.visualize());
    for (i, m) in moves.chars().enumerate() {
        warehouse.move_robot(m);
        println!("{} Move {}:", i, m);
        println!("{}", warehouse.visualize());
        let mut buf = String::new();
        io::stdin().read_line(&mut buf);
    }

    Ok(())
}


#[derive(Debug, PartialEq, Clone, Default)]
struct Warehouse {
    tiles: Vec<Vec<char>>,
    moves: String,
    robot: Coord,
}

impl Warehouse {
    fn move_robot(&mut self, c: char) {
        // println!("Move {}:", c);
        let boxes_before = self.tiles.iter().flatten().filter(|&c| *c == 'O').count();
        let visualize_before = self.visualize();
        match c {
            '^' => self.up(),
            'v' => self.down(),
            '<' => self.left(),
            '>' => self.right(),
            _ => panic!("Invalid move: {}", c),
        }
        let boxes_after = self.tiles.iter().flatten().filter(|&c| *c == 'O').count();
        if boxes_after < boxes_before {
            println!("Box disappeared!");
            let visualize_after = self.visualize();
            println!("Before:");
            println!("{}", visualize_before);
            println!("After:");
            println!("Move {}:", c);
            println!("{}", visualize_after);
            panic!("Box disappeared!");
        }
    }

    fn up(&mut self) {
        if self.robot.y == 1 {
            return;
        } else if self.tiles[self.robot.y as usize - 1][self.robot.x as usize] == '#' {
            return;
        } else if self.tiles[self.robot.y as usize - 1][self.robot.x as usize] == 'O' {
            // Attempt to move all boxes in the same direction
            // Look for the first open tile (.) and move the box there.
            // Then update robot position
            // println!("Robot on tile: {} at ({}, {})", self.tiles[self.robot.y as usize][self.robot.x as usize], self.robot.y, self.robot.x);
            // println!("for y in {}..0", self.robot.y - 1);
            for y in (0..self.robot.y).rev() {
                // println!("Up Checking tile: {}", self.tiles[y as usize][self.robot.x as usize]);
                if self.tiles[y as usize][self.robot.x as usize] == '#' {
                    return;
                } else if self.tiles[y as usize][self.robot.x as usize] == '.' {
                    self.tiles[y as usize][self.robot.x as usize] = 'O';
                    // println!("Moved box to: {}, {}", y, self.robot.x);
                    // set previous tile to empty
                    break;
                }
            }
        } else {
            // println!("Current tile: {}", self.tiles[self.robot.y as usize][self.robot.x as usize]);
            // println!("Unexpected tile: {}", self.tiles[self.robot.y as usize - 1][self.robot.x as usize]);
        }
        let new_y = self.robot.y - 1;
        if new_y >= 0 {
            self.robot.y = new_y;
            self.tiles[self.robot.y as usize][self.robot.x as usize] = '.'; // set new position to empty
        }
    }

    fn down(&mut self) {
        if self.robot.y == self.tiles.len() as i64 - 2 {
            return;
        } else if self.tiles[self.robot.y as usize + 1][self.robot.x as usize] == '#' {
            return;
        } else if self.tiles[self.robot.y as usize + 1][self.robot.x as usize] == 'O' {
            // Attempt to move all boxes in the same direction
            // Look for the first open tile (.) and move the box there.
            // Then update robot position
            for y in self.robot.y + 1..self.tiles.len() as i64 {
                if self.tiles[y as usize][self.robot.x as usize] == '#' {
                    return;
                }
                if self.tiles[y as usize][self.robot.x as usize] == '.' {
                    self.tiles[y as usize][self.robot.x as usize] = 'O';
                    // set previous tile to empty
                    break;
                }
            }
        }
        let new_y = self.robot.y + 1;
        if new_y < self.tiles.len() as i64 {
            self.robot.y = new_y;
            self.tiles[self.robot.y as usize][self.robot.x as usize] = '.'; // set new position to empty
        }
    }

    fn left(&mut self) {
        // is left a wall? '#' or out of bounds?
        if self.robot.x == 1 {
            return;
        } else if self.tiles[self.robot.y as usize][self.robot.x as usize - 1] == '#' {
            return;
        } else if self.tiles[self.robot.y as usize][self.robot.x as usize - 1] == 'O' {
            // Attempt to move all boxes in the same direction
            // Look for the first open tile (.) and move the box there.
            // Then update robot position
            for x in (0..self.robot.x).rev() {
                if self.tiles[self.robot.y as usize][x as usize] == '#' {
                    return;
                }
                if self.tiles[self.robot.y as usize][x as usize] == '.' {
                    self.tiles[self.robot.y as usize][x as usize] = 'O';
                    // set previous tile to empty
                    break;
                }
            }
        }
        let new_x = self.robot.x - 1;
        if new_x >= 0 {
            self.robot.x = new_x;
            self.tiles[self.robot.y as usize][self.robot.x as usize] = '.'; // set new position to empty
        }
    }

    fn right(&mut self) {
        if self.robot.x == self.tiles[0].len() as i64 - 2 {
            return;
        } else if self.tiles[self.robot.y as usize][self.robot.x as usize + 1] == '#' {
            return;
        } if self.tiles[self.robot.y as usize][self.robot.x as usize + 1] == 'O' {
            // Attempt to move all boxes in the same direction
            // Look for the first open tile (.) and move the box there.
            // Then update robot position
            for x in self.robot.x + 2..self.tiles[0].len() as i64 {
                if self.tiles[self.robot.y as usize][x as usize] == '#' {
                    return;
                }
                if self.tiles[self.robot.y as usize][x as usize] == '.' {
                    self.tiles[self.robot.y as usize][x as usize] = 'O';
                    // set previous tile to empty
                    break;
                }
            }
        }
        let new_x = self.robot.x + 1;
        if new_x < self.tiles[0].len() as i64 {
            self.robot.x = new_x;
            self.tiles[self.robot.y as usize][self.robot.x as usize] = '.'; // set new position to empty
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

    fn do_moves(&mut self) {
        self.moves.clone().chars().for_each(|c| {
            self.move_robot(c);
        });
    }

    fn solve(&self) -> i64 {
        self.tiles.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().map(|(x, c)| {
                if *c == 'O' {
                    (100 * y + x) as i64
                } else {
                    0
                }
            }).sum::<i64>()
        }).sum()
    }
}

impl FromStr for Warehouse {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split("\n\n").collect();
        let mut tiles = parse_grid_chars(tokens[0])?;
        let moves = tokens[1].split('\n').map(|s| s.trim()).collect();
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
    fn test_example_steps() -> Result<(), String> {
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
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}\n", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('>');
        let actual = warehouse.visualize();
        let expected = r"########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}\n", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('>');
        let actual = warehouse.visualize();
        let expected = r"########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}\n", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('v');
        let actual = warehouse.visualize();
        let expected = r"########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('v');
        let actual = warehouse.visualize();
        let expected = r"########
#....OO#
##..@..#
#...O..#
#.#.O..#
#...O..#
#...O..#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('<');
        let actual = warehouse.visualize();
        let expected = r"########
#....OO#
##.@...#
#...O..#
#.#.O..#
#...O..#
#...O..#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('v');
        let actual = warehouse.visualize();
        let expected = r"########
#....OO#
##.....#
#..@O..#
#.#.O..#
#...O..#
#...O..#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('>');
        let actual = warehouse.visualize();
        let expected = r"########
#....OO#
##.....#
#...@O.#
#.#.O..#
#...O..#
#...O..#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('>');
        let actual = warehouse.visualize();
        let expected = r"########
#....OO#
##.....#
#....@O#
#.#.O..#
#...O..#
#...O..#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('v');
        let actual = warehouse.visualize();
        let expected = r"########
#....OO#
##.....#
#.....O#
#.#.O@.#
#...O..#
#...O..#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('<');
        let actual = warehouse.visualize();
        let expected = r"########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}", expected);
        assert_eq!(actual, expected);

        warehouse.move_robot('<');
        let actual = warehouse.visualize();
        let expected = r"########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}", expected);
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_large_example() -> Result<(), String> {
        let input = include_str!("../test-large.txt");
        let mut warehouse = input.parse::<Warehouse>().unwrap();

        warehouse.do_moves();
        let actual = warehouse.visualize();
        let expected = r"##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########";
        println!("actual:");
        println!("{}\n", actual);
        println!("expected:");
        println!("{}\n", expected);
        assert_eq!(actual, expected);

        let actual = warehouse.solve();
        let expected = 10092;
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

    #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input)?;
        let solution = 1514353;
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
