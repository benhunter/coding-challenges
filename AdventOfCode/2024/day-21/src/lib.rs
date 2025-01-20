use std::{fmt::Display, str::FromStr};
use util::{parse_grid_chars, Coord, Direction, ParseError};

pub fn solve_part1(input: &str) -> Result<i64, String> {
    let conundrum: Conundrum = input.parse()?;
    todo!()
}

pub fn solve_part2(input: &str) -> Result<i64, String> {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Conundrum {
    codes: Vec<String>,
    robot_1: Robot,
    robot_2: Robot
}

impl Conundrum {
    fn state(&self) -> String {
        let keypad1 = self.robot_1.button();
        let keypad2 = self.robot_2.button();
        format!("1 position: {}\n2 position: {}", keypad1, keypad2)
    }

    fn press(&mut self, commands: &str) -> () {
        println!("press commands={}", commands);
        commands.chars().for_each(|c| {
            match c {
                'A' => self.robot_2.press(), 
                // '^' => 
                '<' => self.robot_2.go(Direction::Left),
                // '>' =>
                // 'v' =>
                _ => todo!("{}", c)
            }
        });
    }
}

impl FromStr for Conundrum {
    fn from_str(s: &str) -> Result<Conundrum, ParseError> {
        let lines = s.lines().map(str::to_string).collect();
        let robot_2 = Robot {
            pad_position: PadPosition::DirectionPad(Coord::new(2, 0)),
        };
        Ok(Conundrum { codes: lines, robot_1: Robot::new(), robot_2 })
    }

    type Err = ParseError;
}

#[derive(Debug, PartialEq, Clone)]
enum PadPosition {
    DirectionPad(Coord),
    NumPad(Coord)
}

impl Default for PadPosition {
    fn default() -> Self {
        PadPosition::NumPad(Coord::new(2, 3))
    }
}

impl Display for PadPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Robot {
    pad_position: PadPosition,
}

impl Robot {
    fn new() -> Self {
        Robot { pad_position: PadPosition::default() }
    }

    fn button(&self) -> char {
        match self.pad_position {
            PadPosition::DirectionPad(c) => {
                let dpad = [[' ', '^', 'A'], ['<', 'v', '>']];
                dpad[c.y as usize][c.x as usize]
                // match c {
                //     Coord { x: 1, y: 0 } => "^",
                //     Coord { x: 2, y: 0 } => "A",
                //     Coord { x: 0, y: 1 } => "<",
                //     Coord { x: 1, y: 1 } => "v",
                //     Coord { x: 2, y: 1 } => ">",
                //     _ => todo!("{:?}", c)
                // }
            },
            PadPosition::NumPad(c) => {
                let npad = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3'], [' ', '0', 'A']];
                npad[c.y as usize][c.x as usize]
                // match c {
                //     Coord { x: 0, y: 0 } => "7",
                //     Coord { x: 1, y: 0 } => "8",
                //     Coord { x: 2, y: 0 } => "9",
                //     Coord { x: 0, y: 1 } => "4",
                //     Coord { x: 1, y: 1 } => "5",
                //     Coord { x: 2, y: 1 } => "6",
                //     Coord { x: 0, y: 2 } => "1",
                //     Coord { x: 1, y: 2 } => "2",
                //     Coord { x: 2, y: 2 } => "3",
                //     Coord { x: 2, y: 3 } => "A",
                //     Coord { x: 1, y: 3 } => "0",
                //     _ => todo!("{:?}", c)
                // }
            }
        }
    }

    fn go(&mut self, direction: Direction) {
        let unit = Coord::from(direction);
        println!("go: direcion={}, unit={:?}", direction, unit);
        self.pad_position = match self.pad_position {
            PadPosition::NumPad(coord) => {
                let updated_coord = coord + unit;
                println!("matched PadPosition::NumPad(). coord + unit = {:?}", updated_coord);
                PadPosition::NumPad(updated_coord)
            },
            PadPosition::DirectionPad(coord) => {
                println!("matched PadPosition::DirectionPad()");
                PadPosition::DirectionPad(coord + unit)
            },
        };
        println!("go: pad_position={}", self.pad_position);
        //let p = self.pad_position + Coord::from(direction);
    }

    fn press(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test1.txt");
        let actual: Conundrum = input.parse()?;

        let expected_codes_len = 5;
        assert_eq!(actual.codes.len(), expected_codes_len);

        let expected_code_0 = "029A";
        assert_eq!(actual.codes.first().expect("should have a code"), expected_code_0);
        Ok(())
    }

    #[test]
    fn test_start_state() -> Result<(), String> {
        let conundrum: Conundrum = "".parse()?;
        let actual = conundrum.state();
        let solution = "1 position: A\n2 position: A";
        assert_eq!(actual, solution);
        Ok(())
    }

    #[test]
    fn test_press() -> Result<(), String> {
        let mut conundrum: Conundrum = "".parse()?;
        conundrum.press("<");
        let actual = conundrum.state();
        let solution = "1 position: A\n2 position: ^";
        assert_eq!(actual, solution);

        conundrum.press("v");
        let actual = conundrum.state();
        let solution = "1 position: A\n2 position: v";
        assert_eq!(actual, solution);

        Ok(())
    }

    // #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test1.txt");
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
        let input = include_str!("../test1.txt");
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
