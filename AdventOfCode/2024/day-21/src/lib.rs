use std::{fmt::Display, str::FromStr};
use util::{Coord, Direction, ParseError};

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
        let keypad1 = self.robot_1.pointed_at_button();
        let keypad2 = self.robot_2.pointed_at_button();
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

    fn distance_r1_to(&self, npad_button: char) -> usize {
        let r2_pad_current_coord = self.robot_2.pad_position.coord();
        let r1_pad_current_coord = self.robot_1.pad_position.coord();
        let r1_dest_coord = self.robot_1.coord_of(npad_button);
        println!("distance_r1_to curr={}, dest={}", r1_pad_current_coord, r1_dest_coord);

        let d = r1_pad_current_coord.distance(r1_dest_coord);
        println!("r1 curr to dest dist={}", d);

        let path = self.robot_1.path_to(npad_button);
        println!("r1 curr to dest path={}", path);

        path.len()
    }

    fn distance_numpad_to(&self, npad_button: char) -> usize {
        let pad_current_coord = self.robot_1.pad_position.coord();
        let dest_coord = self.robot_1.coord_of(npad_button);
        //println!("distance_numpad_to curr={}, dest={}", pad_current_coord, dest_coord);
        let d = pad_current_coord.distance(dest_coord);
        //println!("dist={}", d);
        d as usize
    }

    fn distance_r2_to(&self, npad_button: char) -> usize {
        let mut r2_path = String::new();
        let r1_path = self.robot_1.path_to(npad_button);
        r1_path.chars().for_each(|r1_btn| {
            r2_path.push_str(self.robot_2.path_to(r1_btn).as_str());
            println!("r1_btn={}, r2_path={}", r1_btn, r2_path);
        });
        todo!()
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

impl PadPosition {
    fn coord(&self) -> Coord {
        match self {
            PadPosition::NumPad(c) => *c,
            PadPosition::DirectionPad(c) => *c,
        }
    }
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

    fn pointed_at_button(&self) -> char {
        let pad = self.pad();
        let c = match self.pad_position {
            PadPosition::NumPad(c) => c,
            PadPosition::DirectionPad(c) => c,
        };

        pad[c.y as usize][c.x as usize]
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

    fn coord_of(&self, button: char) -> Coord {
        let vals: (usize, usize, char) = *self
            .pad()
            .iter()
            .enumerate()
            .map(|(yi, y)| {
                y
                    .iter()
                    .enumerate()
                    .map(move |(xi, x)| (xi, yi, *x))
        })
            .flatten()
            .filter(|(_xi, _yi, x)| *x == button )
            //.inspect(|x| println!("{:?}", x))
            .collect::<Vec<(usize, usize, char)>>()
            .first()
            .unwrap();

        Coord::new(vals.0 as i64, vals.1 as i64)
    }

    fn pad(&self) -> Vec<Vec<char>> {
        match self.pad_position {
            PadPosition::DirectionPad(_) => {
                vec!(vec!(' ', '^', 'A'), vec!('<', 'v', '>'))
            },
            PadPosition::NumPad(_) => {
                vec!(vec!('7', '8', '9'), vec!('4', '5', '6'), vec!('1', '2', '3'), vec!(' ', '0', 'A'))
            }
        }
    }

    fn path_to(&self, button: char) -> String {
        let self_coord = self.pad_position.coord();
        let button_coord = self.coord_of(button);
        let mut diff = self_coord - button_coord;
        println!("Robot::path_to() self={:?}, button={}, button coord={}, diff={}", self, button, button_coord, diff);
        let mut path = String::new();

        while diff.x != 0 {
            if diff.x > 0 { // go left
                path.push('<');
                diff.x -= 1;
            } else {
                path.push('>');
                diff.x += 1;
            }
        }

        while diff.y != 0 {
            if diff.y > 0 {
                path.push('^'); // (0, 0) is origin
                diff.y -=1;
            } else {
                path.push('v');
                diff.y += 1;
            }
        }
        path.push('A');
        path
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

        //conundrum.press("v");
        //let actual = conundrum.state();
        //let solution = "1 position: A\n2 position: v";
        //assert_eq!(actual, solution);

        Ok(())
    }

    #[test]
    fn test_distance() -> Result<(), String> {
        let conundrum: Conundrum = "".parse()?;

        let actual = conundrum.distance_numpad_to('0');
        let expected = 1;
        assert_eq!(expected, actual);

        let actual = conundrum.distance_r1_to('0');
        let expected = 2;
        assert_eq!(expected, actual);

        let actual = conundrum.distance_r2_to('0');
        let expected = 8;
        assert_eq!(expected, actual);
        Ok(())
    }

    // #[test]
    //fn test_part1() -> Result<(), String> {
    //    let input = include_str!("../test1.txt");
    //    let actual = solve_part1(input)?;
    //    let solution = 0;
    //    assert_eq!(actual, solution);
    //    Ok(())
    //}

    // #[test]
    //fn test_solve_part1() -> Result<(), String> {
    //    let input = include_str!("../input.txt");
    //    let actual = solve_part1(input)?;
    //    let solution = 0;
    //    assert_eq!(actual, solution);
    //    Ok(())
    //}

    // #[test]
    //fn test_part2() -> Result<(), String> {
    //    let input = include_str!("../test1.txt");
    //    let actual = solve_part2(input)?;
    //    let solution = 0;
    //    assert_eq!(actual, solution);
    //    Ok(())
    //}

    // #[test]
    //fn test_solve_part2() -> Result<(), String> {
    //    let input = include_str!("../input.txt");
    //    let actual = solve_part2(input)?;
    //    let solution = 0;
    //    assert_eq!(actual, solution);
    //    Ok(())
    //}
}
