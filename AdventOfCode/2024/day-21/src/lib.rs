use std::{fmt::Display, str::FromStr};
use util::{Coord, Direction, ParseError};

pub fn solve_part1(input: &str) -> Result<i64, String> {
    let conundrum: Conundrum = input.parse()?;
    Ok(conundrum.solve_part1() as i64)
}

pub fn solve_part2(input: &str) -> Result<i64, String> {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Conundrum {
    codes: Vec<String>,
    robot_1: Robot,
    robot_2: Robot,
    robot_3: Robot,
}

impl Conundrum {
    fn state(&self) -> String {
        let keypad1 = self.robot_1.pointed_at_button();
        let keypad2 = self.robot_2.pointed_at_button();
        format!("1 position: {}\n2 position: {}", keypad1, keypad2)
    }

    //fn press(&mut self, commands: &str) -> () {
    //    println!("press commands={}", commands);
    //    commands.chars().for_each(|c| {
    //        match c {
    //            'A' => self.robot_2.press(),
    //            // '^' =>
    //            '<' => self.robot_2.go(Direction::Left),
    //            // '>' =>
    //            // 'v' =>
    //            _ => todo!("{}", c)
    //        }
    //    });
    //}

    fn distance_numpad_to(&self, npad_button: char) -> usize {
        let pad_current_coord = self.robot_1.pad_position.coord();
        let dest_coord = self.robot_1.coord_of(npad_button);
        //println!("distance_numpad_to curr={}, dest={}", pad_current_coord, dest_coord);
        let d = pad_current_coord.distance(dest_coord);
        //println!("dist={}", d);
        d as usize
    }

    //fn distance_r1_to(&self, npad_button: char) -> usize {
    //    let r1_pad_current_coord = self.robot_1.pad_position.coord();
    //    let r1_dest_coord = self.robot_1.coord_of(npad_button);
    //    println!("[distance_r1_to] curr={}, dest={}", r1_pad_current_coord, r1_dest_coord);
    //
    //    let d = r1_pad_current_coord.distance(r1_dest_coord);
    //    println!("[distance_r1_to] r1 curr to dest dist={}", d);
    //
    //    let path = self.robot_1.path_to(npad_button);
    //    println!("[distance_r1_to] r1 curr to dest path={}", path);
    //
    //    path.len()
    //}

    fn distance_r2_to(&self, npad_button: char) -> usize {
        println!("[distance_r2_to] self={:?}, npad_button={}", self, npad_button);
        let mut r2_path = String::new();
        let r1_path = self.robot_1.path('A', npad_button, &self.robot_1);
        let mut prev_r1_btn = 'A'; // Start on 'A'.
        r1_path.chars().for_each(|r1_btn| {
            println!("[distance_r2_to] r1_btn={}, r2_path={}", r1_btn, r2_path);
            r2_path.push_str(self.robot_2.path(prev_r1_btn, r1_btn, &self.robot_2).as_str());
            prev_r1_btn = r1_btn;
        });
        r2_path.len()
    }

    fn distance_r3_to(&self, npad_button: char) -> usize {
        println!("[distance_r3_to] self={:?}, npad_button={}", self, npad_button);

        let r1_path = self.robot_1.path('A', npad_button, &self.robot_1);
        println!("[distance_r3_to] r1_path={}", r1_path);
        let mut prev_r1_btn = 'A'; // Start on 'A'.
        let mut r2_path = String::new();
        let mut prev_r2_btn = 'A'; // Start on 'A'.
        let mut r3_path = String::new();

        r1_path.chars().for_each(|r1_btn| {
            println!("[distance_r3_to] prev_r1_btn={} r1_btn={}", prev_r1_btn, r1_btn);
            r2_path = self.robot_2.path(prev_r1_btn, r1_btn, &self.robot_2);
            println!("[distance_r3_to] r2_path={}", r2_path);

            prev_r1_btn = r1_btn;

            r2_path.chars().for_each(|r2_btn| {
                println!("[distance_r3_to] prev_r2_btn={}, r2_btn={}", prev_r2_btn, r2_btn);
                r3_path.push_str(self.robot_3.path(prev_r2_btn, r2_btn, &self.robot_3).as_str());
                prev_r2_btn = r2_btn;
                println!("[distance_r3_to] r3_path={}", r3_path);
            })
        });

        println!("[distance_r3_to] r3_path={}, r3_path.len()={}", r3_path, r3_path.len());
        r3_path.len()
    }

    fn distance_r3(&self, from_npad_btn: char, to_npad_button: char) -> usize {
        //println!("[distance_r3] self={:?}, npad_button={}", self, to_npad_button);

        let r1_path = self.robot_1.path(from_npad_btn, to_npad_button, &self.robot_1);
        //println!("[distance_r3] r1_path={}", r1_path);
        let mut prev_r1_btn = 'A'; // Start on 'A'.
        let mut r2_path = String::new();
        let mut r2_full_path = String::new();
        let mut prev_r2_btn = 'A'; // Start on 'A'.
        let mut r3_path = String::new();

        r1_path.chars().for_each(|r1_btn| {
            //println!("[distance_r3] prev_r1_btn={} r1_btn={}", prev_r1_btn, r1_btn);
            r2_path = self.robot_2.path(prev_r1_btn, r1_btn, &self.robot_2);
            r2_full_path.push_str(&r2_path);
            //println!("[distance_r3] r2_path={}", r2_path);

            prev_r1_btn = r1_btn;

            r2_path.chars().for_each(|r2_btn| {
                //println!("[distance_r3] prev_r2_btn={}, r2_btn={}", prev_r2_btn, r2_btn);
                r3_path.push_str(self.robot_3.path(prev_r2_btn, r2_btn, &self.robot_3).as_str());
                prev_r2_btn = r2_btn;
                //println!("[distance_r3] r3_path={}", r3_path);
            })
        });

        println!("[distance_r3] {} to {}, r3_path={}, r3_path.len()={}, r2_full_path={}, r1_path={}", from_npad_btn, to_npad_button, r3_path, r3_path.len(), r2_full_path, r1_path);
        r3_path.len()
    }

    fn solve_part1(&self) -> usize {
        self.codes
            .iter()
            .map(|code| {
                let mut code = code.clone();
                let numeric = &code[..code.len() - 1];
                let numeric: usize = numeric.parse().expect("must be a number");
                code.insert(0, 'A');
                let sum = code
                    .as_bytes()
                    .windows(2)
                    .map(|w| {
                        let a = w[0].into();
                        let b = w[1].into();
                        self.distance_r3(a, b)
                    })
                        .sum::<usize>();
                    //.collect::<Vec<usize>>()
                println!("code={}, sum={}, numeric={}, sum*numeric={}", code, sum, numeric, sum*numeric);
                (sum * numeric) as usize
                //.collect::<Vec<usize>>()
            })
            //.inspect(|x| println!("{}", x))
            .sum()
    }
}

impl FromStr for Conundrum {
    fn from_str(s: &str) -> Result<Conundrum, ParseError> {
        let lines = s.lines().map(str::to_string).collect();
        let robot_2 = Robot {
            pad_position: PadPosition::DirectionPad(Coord::new(2, 0)),
        };
        let robot_3 = robot_2.clone();

        Ok(Conundrum { codes: lines, robot_1: Robot::new(), robot_2, robot_3 })
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

    //fn press(&self) {
    //    todo!()
    //}

    fn coord_of(&self, button: char) -> Coord {
        //println!("[coord_of] self={:?}, button={}", self, button);
        let vals: (usize, usize, char) = *self
            .pad()
            .iter()
            .enumerate()
            .flat_map(|(yi, y)| {
                y
                    .iter()
                    .enumerate()
                    .map(move |(xi, x)| (xi, yi, *x))
        })
            .filter(|(_xi, _yi, x)| *x == button )
            //.inspect(|x| println!("[coord_of] filtered {:?}", x))
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

    //fn path_to(&self, button: char) -> String {
    //    let self_coord = self.pad_position.coord();
    //    let button_coord = self.coord_of(button);
    //    let mut diff = self_coord - button_coord;
    //    //println!("[Robot::path_to()] self={:?}, button={}, button coord={}, diff={}", self, button, button_coord, diff);
    //    let mut path = String::new();
    //
    //    while diff.x != 0 {
    //        if diff.x > 0 { // go left
    //            path.push('<');
    //            diff.x -= 1;
    //        } else {
    //            path.push('>');
    //            diff.x += 1;
    //        }
    //    }
    //
    //    while diff.y != 0 {
    //        if diff.y > 0 {
    //            path.push('^'); // (0, 0) is origin
    //            diff.y -=1;
    //        } else {
    //            path.push('v');
    //            diff.y += 1;
    //        }
    //    }
    //    path.push('A');
    //    path
    //}

    fn path(&self, from_button: char, to_button: char, using: &Robot) -> String {
        let from_coord = using.coord_of(from_button);
        let to_coord = using.coord_of(to_button);
        let mut diff = from_coord - to_coord;
        //println!("[Robot::path()] self={:?}, from_button={}, to_button={}", self, from_button, to_button);
        let mut path = String::new();

        // TODO if numpad, avoid (0,3)
        // TODO if dirpad, avoid (0,0)
        let avoid = match self.pad_position {
            PadPosition::NumPad(_) => Coord::new(0, 3),
            PadPosition::DirectionPad(_) => Coord::new(0, 0),
        };
        let mut curr = from_coord.clone();

        while diff.x != 0 || diff.y != 0 {
            match self.pad_position {
                PadPosition::NumPad(c) => {
                    // if going down, go down first, then left
                    // if going up, go right first, then left

                    while diff.y != 0 {
                        if diff.y < 0 {
                            if curr.x == avoid.x && (curr.y + 1) == avoid.y {
                                println!("[Robot::path()] avoid={}, curr={} avoid going down", avoid, curr);
                                break;
                            } else {
                                path.push('v');
                                diff.y += 1;
                                curr.y += 1;
                            }
                        } else {
                            if curr.x == avoid.x && (curr.y - 1) == avoid.y {
                                break;
                            } else {
                                path.push('^'); // (0, 0) is origin
                                diff.y -=1;
                                curr.y -=1;
                            }
                        }
                    }

                    while diff.x != 0 {
                        if diff.x > 0 { // go left
                            if curr.y == avoid.y && (curr.x - 1) == avoid.x {
                                // TODO for dpad - better to go vv< than <v< for path from A to <
                                println!("[Robot::path()] avoid going left, avoid={}, curr={}, path={}", avoid, curr, path);
                                break;
                            } else {
                                path.push('<');
                                diff.x -= 1;
                                curr.x -= 1;
                            }
                            //if curr.y == avoid.y && curr.x == avoid.y {
                            //    panic!("from={}, to={}, diff={}, avoid={}", from_coord, to_coord, diff, avoid);
                            //}
                        } else {
                            path.push('>');
                            diff.x += 1;
                            curr.x += 1;
                        }
                    }

                }

                PadPosition::DirectionPad(c) => {
                    while diff.y != 0 {
                        if diff.y < 0 {
                            if curr.x == avoid.x && (curr.y + 1) == avoid.y {
                                println!("[Robot::path()] avoid={}, curr={} avoid going down", avoid, curr);
                                break;
                            } else {
                                path.push('v');
                                diff.y += 1;
                                curr.y += 1;
                            }
                        } else {
                            if curr.x == avoid.x && (curr.y - 1) == avoid.y {
                                break;
                            } else {
                                path.push('^'); // (0, 0) is origin
                                diff.y -=1;
                                curr.y -=1;
                            }
                        }
                    }

                    while diff.x != 0 {
                        if diff.x > 0 { // go left
                            if curr.y == avoid.y && (curr.x - 1) == avoid.x {
                                // TODO for dpad - better to go vv< than <v< for path from A to <
                                println!("[Robot::path()] avoid going left, avoid={}, curr={}, path={}", avoid, curr, path);
                                break;
                            } else {
                                path.push('<');
                                diff.x -= 1;
                                curr.x -= 1;
                            }
                            //if curr.y == avoid.y && curr.x == avoid.y {
                            //    panic!("from={}, to={}, diff={}, avoid={}", from_coord, to_coord, diff, avoid);
                            //}
                        } else {
                            path.push('>');
                            diff.x += 1;
                            curr.x += 1;
                        }
                    }


                }
            }

        }

        assert_eq!(to_coord, curr);

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

    //#[test]
    //fn test_press() -> Result<(), String> {
    //    let mut conundrum: Conundrum = "".parse()?;
    //    conundrum.press("<");
    //    let actual = conundrum.state();
    //    let solution = "1 position: A\n2 position: ^";
    //    assert_eq!(actual, solution);
    //
    //    //conundrum.press("v");
    //    //let actual = conundrum.state();
    //    //let solution = "1 position: A\n2 position: v";
    //    //assert_eq!(actual, solution);
    //
    //    Ok(())
    //}

    #[test]
    fn test_distance_numpad_to() -> Result<(), String> {
        let conundrum: Conundrum = "".parse()?;

        let actual = conundrum.distance_numpad_to('0');
        let expected = 1;
        assert_eq!(expected, actual);

        Ok(())
    }

    //#[test]
    //fn test_distance_r1_to() -> Result<(), String> {
    //    let conundrum: Conundrum = "".parse()?;
    //
    //    let actual = conundrum.distance_r1_to('0');
    //    let expected = 2;
    //    assert_eq!(expected, actual);
    //
    //    Ok(())
    //}

    #[test]
    fn test_distance_r2_to() -> Result<(), String> {
        let conundrum: Conundrum = "".parse()?;

        let actual = conundrum.distance_r2_to('0');
        let expected = 8;
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_distance_r3_to() -> Result<(), String> {
        let conundrum: Conundrum = "".parse()?;

        let actual = conundrum.distance_r3_to('0');
        let expected = 18;
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_distance_r3_from_A_to_0() -> Result<(), String> {
        let conundrum: Conundrum = "".parse()?;

        let actual = conundrum.distance_r3('A', '0');
        let expected = 18;
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_distance_r3_from_0_to_9() -> Result<(), String> {
        let conundrum: Conundrum = "".parse()?;

        let actual = conundrum.distance_r3('0', '9');
        let expected = 21; // TODO not verified
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_029A_by_steps() -> Result<(), String> {
        /*
        *   029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
        */
        let conundrum: Conundrum = "".parse()?;

        let actual: usize = vec![
            conundrum.distance_r3('A', '0'),
            conundrum.distance_r3('0', '2'),
            conundrum.distance_r3('2', '9'),
            conundrum.distance_r3('9', 'A'),
        ].iter().sum();
        let expected = 68;
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_029A_code() -> Result<(), String> {
        let conundrum: Conundrum = "029A".parse()?;

        let actual = conundrum.solve_part1();
        let expected = 68 * 29;
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_980A_code() -> Result<(), String> {
        /*
        *   980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
        */
        let conundrum: Conundrum = "980A".parse()?;

        let actual = conundrum.solve_part1();
        let expected = 60 * 980;
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_179A_code() -> Result<(), String> {
        /*
        * Expected:
        *   179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        *
        * Actual:
        * [Robot::path()] avoid going left, avoid=(0, 3), curr=(1, 3), path=<
        * [distance_r3] A to 1, r3_path=v<A<AA>>^AvA^<A>Av<A<A>>^AvAA^<A>A, r3_path.len()=34, r2_full_path=v<<A>^Av<A>>^A, r1_path=<^<A
        * [distance_r3] 1 to 7, r3_path=v<<A>>^AAvA^A, r3_path.len()=13, r2_full_path=<AA>A, r1_path=^^A
        * [distance_r3] 7 to 9, r3_path=v<A^>AA<A>A, r3_path.len()=11, r2_full_path=vAA^A, r1_path=>>A
        * [distance_r3] 9 to A, r3_path=v<A<A>>^AAA<Av>A^A, r3_path.len()=18, r2_full_path=v<AAA^>A, r1_path=vvvA
        * code=A179A, sum=76, numeric=179, sum*numeric=13604
        *
        * A1 v<A<AA>>^AvA^<A>Av<A<A>>^AvAA^<A>A
        * 17 v<<A>>^AAvA^A
        * 79 v<A^>AA<A>A
        * 9A v<A<A>>^AAA<Av>A^A
        */
        let conundrum: Conundrum = "179A".parse()?;

        let actual = conundrum.solve_part1();
        let expected = 68 * 179;
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_456A_code() -> Result<(), String> {
        /*
        *   456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
        */
        let conundrum: Conundrum = "456A".parse()?;

        let actual = conundrum.solve_part1();
        let expected = 64 * 456;
        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_379A_code() -> Result<(), String> {
        /*
        *   379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        *
        * [distance_r3] A to 3, r3_path=v<<A>>^AvA^A, r3_path.len()=12, r2_full_path=<A>A, r1_path=^A
        * [distance_r3] 3 to 7, r3_path=v<<A>>^AAv<A<A>>^AAvAA^<A>A, r3_path.len()=27, r2_full_path=<AAv<AA>>^A, r1_path=^^<<A
        * [distance_r3] 7 to 9, r3_path=v<A^>AA<A>A, r3_path.len()=11, r2_full_path=vAA^A, r1_path=>>A
        * [distance_r3] 9 to A, r3_path=v<A<A>>^AAA<Av>A^A, r3_path.len()=18, r2_full_path=v<AAA^>A, r1_path=vvvA
        * code=A379A, sum=68, numeric=379, sum*numeric=25772
        *
        * expected 379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        * actual      A3 v<<A>>^AvA^A
        *                     37 v<<A>>^AAv<A<A>>^AAvAA^<A>A  * <- TODO error here? 3 to 7
        *                                                79 v<A^>AA<A>A
        *                                                           9A v<A<A>>^AAA<Av>A^A
        *
        */
        let conundrum: Conundrum = "379A".parse()?;

        let actual = conundrum.solve_part1();
        let expected = 64 * 379;
        assert_eq!(expected, actual);

        Ok(())
    }

     #[test]
    fn test_part1() -> Result<(), String> {
        // TODO: path cannot cross the empty space on the keypad
        // https://www.reddit.com/r/adventofcode/comments/1hja685/2024_day_21_here_are_some_examples_and_hints_for/

        /* Expected
029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        */

        /* Actual
[distance_r3] A to 0, r3_path=<<vAA>A>^AvAA<^A>A, r3_path.len()=18, r2_full_path=<<vA>>^A, r1_path=<A
[distance_r3] 0 to 2, r3_path=<<vA>>^AvA^A, r3_path.len()=12, r2_full_path=<A>A, r1_path=^A
[distance_r3] 2 to 9, r3_path=<vA>^A<<vA>^A>AAvA^A, r3_path.len()=20, r2_full_path=vA<^AA>A, r1_path=>^^A
[distance_r3] 9 to A, r3_path=<<vA>A>^AAAvA<^A>A, r3_path.len()=18, r2_full_path=<vAAA>^A, r1_path=vvvA
code=A029A, sum=68, numeric=29, sum*numeric=1972
[distance_r3] A to 9, r3_path=<<vA>>^AAAvA^A, r3_path.len()=14, r2_full_path=<AAA>A, r1_path=^^^A
[distance_r3] 9 to 8, r3_path=<<vAA>A>^AvAA<^A>A, r3_path.len()=18, r2_full_path=<<vA>>^A, r1_path=<A
[distance_r3] 8 to 0, r3_path=<<vA>A>^AAAvA<^A>A, r3_path.len()=18, r2_full_path=<vAAA>^A, r1_path=vvvA
[distance_r3] 0 to A, r3_path=<vA>^A<A>A, r3_path.len()=10, r2_full_path=vA^A, r1_path=>A
code=A980A, sum=60, numeric=980, sum*numeric=58800
[distance_r3] A to 1, r3_path=<<vAA>A>^AAvA<^A>AvA^A, r3_path.len()=22, r2_full_path=<<vAA>^A>A, r1_path=<<^A
[distance_r3] 1 to 7, r3_path=<<vA>>^AAvA^A, r3_path.len()=13, r2_full_path=<AA>A, r1_path=^^A
[distance_r3] 7 to 9, r3_path=<vA>^AA<A>A, r3_path.len()=11, r2_full_path=vAA^A, r1_path=>>A
[distance_r3] 9 to A, r3_path=<<vA>A>^AAAvA<^A>A, r3_path.len()=18, r2_full_path=<vAAA>^A, r1_path=vvvA
code=A179A, sum=64, numeric=179, sum*numeric=11456
[distance_r3] A to 4, r3_path=<<vAA>A>^AAvA<^A>AAvA^A, r3_path.len()=23, r2_full_path=<<vAA>^AA>A, r1_path=<<^^A
[distance_r3] 4 to 5, r3_path=<vA>^A<A>A, r3_path.len()=10, r2_full_path=vA^A, r1_path=>A
[distance_r3] 5 to 6, r3_path=<vA>^A<A>A, r3_path.len()=10, r2_full_path=vA^A, r1_path=>A
[distance_r3] 6 to A, r3_path=<<vA>A>^AAvA<^A>A, r3_path.len()=17, r2_full_path=<vAA>^A, r1_path=vvA
code=A456A, sum=60, numeric=456, sum*numeric=27360
[distance_r3] A to 3, r3_path=<<vA>>^AvA^A, r3_path.len()=12, r2_full_path=<A>A, r1_path=^A
[distance_r3] 3 to 7, r3_path=<<vAA>A>^AAvA<^A>AAvA^A, r3_path.len()=23, r2_full_path=<<vAA>^AA>A, r1_path=<<^^A
[distance_r3] 7 to 9, r3_path=<vA>^AA<A>A, r3_path.len()=11, r2_full_path=vAA^A, r1_path=>>A
[distance_r3] 9 to A, r3_path=<<vA>A>^AAAvA<^A>A, r3_path.len()=18, r2_full_path=<vAAA>^A, r1_path=vvvA
code=A379A, sum=64, numeric=379, sum*numeric=24256
        */

        /*
expected:
179A: 
A1: <v<A>>^A<vA<A>>^AAvAA<^A>A r2: <Av<AA>>^A r1: ^<<A
17: <v<A>>^AAvA^A
79: <vA>^AA<A>A
9A: <v<A>A>^AAAvA<^A>A

actual:
A1: <<vAA>A>^AAvA<^A>AvA^A, r3_path.len()=22, r2_full_path=<<vAA>^A>A, r1_path=<<^A
pass 17: <<vA>>^AAvA^A, r3_path.len()=13, r2_full_path=<AA>A, r1_path=^^A
pass 79: <vA>^AA<A>A, r3_path.len()=11, r2_full_path=vAA^A, r1_path=>>A
pass 9A: <<vA>A>^AAAvA<^A>A, r3_path.len()=18, r2_full_path=<vAAA>^A, r1_path=vvvA
code=A179A, sum=64, numeric=179, sum*numeric=11456
        */

        let input = include_str!("../test1.txt");
        let actual = solve_part1(input)?;
        let expected = 126384;
        assert_eq!(expected, actual);

        Ok(())
    }

     #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input)?;
        let too_low = 187062;
        assert!(actual > too_low);
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

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
