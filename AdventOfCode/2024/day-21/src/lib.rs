use std::{fmt::{format, Display}, str::{Chars, FromStr}};
use tracing::{debug, field::debug, info, warn};
use tracing_subscriber::field::debug;
use util::{Coord, Direction, ParseError};

pub fn solve_part1(input: &str) -> Result<i64, String> {
    let conundrum: Conundrum = input.parse()?;
    todo!();
    //Ok(conundrum.solve_part1() as i64)
}

pub fn solve_part2(_input: &str) -> Result<i64, String> {
    todo!()
}

#[derive(Debug, PartialEq, Clone)]
struct Conundrum {
    codes: Vec<String>,
    robot_3: Robot,
}

impl FromStr for Conundrum {
    fn from_str(s: &str) -> Result<Conundrum, ParseError> {
        let lines = s.lines().map(str::to_string).collect();
        let robot_1 = Robot { next: Box::new(RobotOrNumpad::Numpad) };
        let robot_2 = Robot { next: Box::new(RobotOrNumpad::Robot(robot_1)) };
        let robot_3 = Robot { next: Box::new(RobotOrNumpad::Robot(robot_2)) };
        Ok(Conundrum { codes: lines, robot_3 })
    }

    type Err = ParseError;
}

#[derive(Debug, PartialEq, Clone)]
enum Pad {
    DirectionPad,
    NumPad,
}

impl Pad {
    fn pad(self) -> Vec<Vec<char>> {
        match self {
            Pad::DirectionPad => {
                vec!(vec!(' ', '^', 'A'), vec!('<', 'v', '>'))
            },
            Pad::NumPad => {
                vec!(vec!('7', '8', '9'), vec!('4', '5', '6'), vec!('1', '2', '3'), vec!(' ', '0', 'A'))
            }
        }
    }
}


impl Default for Pad {
    fn default() -> Self {
        Pad::NumPad
    }
}

impl Display for Pad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum RobotOrNumpad {
    Robot(Robot),
    Numpad,
}

#[derive(Debug, PartialEq, Clone)]
struct Robot {
    // All robots have a DirectionPad
    next: Box<RobotOrNumpad>,
}

impl Robot {
    fn new(next: RobotOrNumpad) -> Self {
        Robot { next: Box::new(next) }
    }

    fn coord_of(&self, button: char) -> Coord {
        //debug!("Robot::coord_of() button={:?}", button);
        let pad = match *self.next {
            RobotOrNumpad::Numpad => Pad::NumPad,
            RobotOrNumpad::Robot(_) => Pad::DirectionPad,
        };
        let vals: (usize, usize, char) = *pad 
            .pad()
            .iter()
            //.inspect(|v| debug!("Robot::coord_of() v={:?}", v))
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

    fn local_paths(&self, from_button: char, to_button: char) -> Vec<String> {
        /*
         * Return multiple strings for valid paths
        */
        debug!("Robot::local_paths() from={}, to={}", from_button, to_button);

        let from_coord = self.coord_of(from_button);
        let to_coord = self.coord_of(to_button);
        let start_diff = from_coord - to_coord;
        debug!("Robot::local_paths() self={:?}, from_button={}, to_button={}", self, from_button, to_button);
        let mut partial_paths: Vec<(String, Coord, Coord)> = vec![(String::new(), start_diff.clone(), from_coord.clone())];
        let mut complete_paths: Vec<String> = vec![];

        // if numpad, avoid (0,3)
        // if dirpad, avoid (0,0)
        let avoid = Coord::new(0, 0);
        //match self.pad {
        //    Pad::NumPad(_) => Coord::new(0, 3),
        //    Pad::DirectionPad(_) => Coord::new(0, 0),
        //};

        let mut loops = 0;
        let mut loops2 = 0;
        let MAX_LOOPS = 10;

        while partial_paths.len() > 0 {
            loops += 1;
            if loops > MAX_LOOPS {
                break;
            }

            let path_diff_coord = partial_paths.pop().unwrap();
            //println!("[Robot::path()] partial_paths={:?} after pop", partial_paths);
            let mut path = path_diff_coord.0;
            let mut diff = path_diff_coord.1;
            let mut curr = path_diff_coord.2;

            if diff.x != 0 && diff.y != 0 {
                // two paths
                // go x and add path
                let mut path_x = path.clone();
                let mut diff_x = diff.clone();
                let mut curr_x = curr.clone();

                if diff_x.x > 0 {
                    //println!("[Robot::path()] diff_x.x > 0");
                    if (curr_x.x - 1) != avoid.x || curr_x.y != avoid.y {
                        //println!("[Robot::path()] push <");
                        path_x.push('<');
                        diff_x.x -= 1;
                        curr_x.x -= 1;
                        partial_paths.push((path_x, diff_x, curr_x));
                    }
                } else if diff_x.x < 0 {
                    //println!("[Robot::path()] diff_x.x < 0");
                    if (curr_x.x + 1) != avoid.x || curr_x.y != avoid.y {
                        path_x.push('>');
                        diff_x.x += 1;
                        curr_x.x += 1;
                        partial_paths.push((path_x, diff_x, curr_x));
                    }
                }

                // go y and add path
                let mut path_y = path.clone();
                let mut diff_y = diff.clone();
                let mut curr_y = curr.clone();

                if diff_y.y > 0 {
                    //println!("[Robot::path()] diff_y.y > 0");
                    if (curr_y.y - 1) != avoid.y || curr_y.x != avoid.x {
                        path_y.push('^');
                        diff_y.y -= 1;
                        curr_y.y -= 1;
                        partial_paths.push((path_y, diff_y, curr_y));
                    }
                } else if diff_y.y < 0 {
                    //println!("[Robot::path()] diff_y.y < 0");
                    if (curr_y.y + 1) != avoid.y || curr_y.x != avoid.x {
                        path_y.push('v');
                        diff_y.y += 1;
                        curr_y.y += 1;
                        partial_paths.push((path_y, diff_y, curr_y));
                    } 
                }

                //println!("[Robot::path()] partial_paths={:?}", partial_paths);

            } else {

                while diff.x != 0 || diff.y != 0 {
                    loops2 += 1;
                    if loops2 > MAX_LOOPS {
                        panic!("too many loops");
                    }

                    //println!("[Robot::path()] self={:?}, from={}, to={}, path={}, diff={}, curr={}, avoid={}", self, from_coord, to_coord, path, diff, curr, avoid);
                    if diff.x > 0 {
                        //println!("[Robot::path()] diff.x > 0");
                        if (curr.x - 1) != avoid.x || curr.y != avoid.y {
                            //println!("[Robot::path()] push <");
                            path.push('<');
                            diff.x -= 1;
                            curr.x -= 1;
                        }
                    } else if diff.x < 0 {
                        //println!("[Robot::path()] diff.x < 0");
                        if (curr.x + 1) != avoid.x || curr.y != avoid.y {
                            path.push('>');
                            diff.x += 1;
                            curr.x += 1;
                        }
                    }

                    if diff.y > 0 {
                        //println!("[Robot::path()] diff.y > 0");
                        if (curr.y - 1) != avoid.y || curr.x != avoid.x {
                            path.push('^');
                            diff.y -= 1;
                            curr.y -= 1;
                        }
                    } else if diff.y < 0 {
                        //println!("[Robot::path()] diff.y < 0");
                        if (curr.y + 1) != avoid.y || curr.x != avoid.x {
                            path.push('v');
                            diff.y += 1;
                            curr.y += 1;
                        }
                    }

                }

                path.push('A');
                complete_paths.push(path.clone());

            }

        }
        complete_paths

    }

    fn paths(self, from_button: char, to_button: char) -> Vec<String> {
        /*
        * Recursive solution. Base case is RobotOrNumpad::NumPad
        */

        let full_paths = match *self.next {
            RobotOrNumpad::Numpad => self.local_paths(from_button, to_button),
            RobotOrNumpad::Robot(ref r) => {

                let mut r_paths = r.clone().paths(from_button, to_button);
                debug!("r_paths={:?}", r_paths);

                let mut window_paths: Vec<Vec<String>> = vec![];

                debug!("starting r_paths.iter()");
                r_paths.iter_mut()
                    .inspect(|r| debug!("r={:?}", r))
                    .for_each(|r_path| {
                        r_path.insert(0, 'A');
                        debug!("r_path={:?}", r_path);
                        r_path.as_bytes()
                            .windows(2)
                            //.inspect(|w| debug!("w={:?}", w))
                            .for_each(|r_path_pair| {
                                debug!("r_path_pair={:?}", r_path_pair);
                                let lp = self.local_paths(r_path_pair[0].into(), r_path_pair[1].into());
                                debug!("lp={:?}", lp);
                                window_paths.push(lp);
                            })

                    });

                debug!("Building full_paths. r_paths={:?}, local_paths={:?}", r_paths, window_paths);

                build_full_paths_from_window_paths(window_paths)
            },
        };

        debug!("full_paths={:?}", full_paths);
        full_paths
    }
}

fn build_full_paths_from_window_paths(window_paths: Vec<Vec<String>>) -> Vec<String> {
    let mut full_paths: Vec<String> = vec!["".to_string()];
    for window_path in window_paths {
        let mut new_full_paths: Vec<String> = vec![];
        for full_path in full_paths {
            for wp in window_path.clone() {
                new_full_paths.push(format!("{}{}", full_path, wp));
            }
        }
        full_paths = new_full_paths;
    }

    full_paths
}

#[derive(Debug)]
struct Path {
    numpad: String,
    r1: String,
    r2: String,
    r3: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

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
    #[allow(non_snake_case)]
    fn test_r1_path_A0() -> Result<(), String> {
        let r1 = Robot { next: Box::new(RobotOrNumpad::Numpad) }; // Robot 1 points to a NumPad.

        let actual = r1.paths('A', '0');
        debug!("{:?}", actual);
        let expected_len = 1;
        assert_eq!(expected_len, actual.len());

        let actual_answer = actual.iter().next().unwrap();
        let expected_answer = "<A";
        assert_eq!(expected_answer, actual_answer);

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_r2_path_A0() -> Result<(), String> {
        let r1 = Robot { next: Box::new(RobotOrNumpad::Numpad) };
        let r2 = Robot { next: Box::new(RobotOrNumpad::Robot(r1)) };
        debug!("{:?}", r2);

        let actual = r2.paths('A', '0');
        debug!("{:?}", actual);
        let expected_len = 4;
        assert_eq!(expected_len, actual.len());

        let expected_len_of_shortest = 8;
        let actual_len_of_shortest = actual.iter().map(|a| a.len()).min().unwrap();
        assert_eq!(expected_len_of_shortest, actual_len_of_shortest);

        let actual_answer = actual.iter().next().unwrap();
        let expected_answer = "v<<A>^>A";
        assert_eq!(expected_answer, actual_answer);

        Ok(())
    }

    #[test]
    fn test_build_full_path() -> Result<(), String> {
        let window_paths = vec![vec!["A".to_string(), "B".to_string()], vec!["C".to_string(), "D".to_string()]];
        let actual = build_full_paths_from_window_paths(window_paths);
        let expected = vec!["AC".to_string(), "AD".to_string(), "BC".to_string(), "BD".to_string()];
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_build_full_path_from_3_windows() -> Result<(), String> {
        let window_paths = vec![vec!["A".to_string(), "B".to_string()], vec!["C".to_string(), "D".to_string()], vec!["E".to_string(), "F".to_string()]];
        let actual = build_full_paths_from_window_paths(window_paths);
        let expected = vec![
            "ACE".to_string(), "ACF".to_string(), "ADE".to_string(), "ADF".to_string(), 
            "BCE".to_string(), "BCF".to_string(), "BDE".to_string(), "BDF".to_string(),
        ];
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_r3_path_A0() -> Result<(), String> {

        /*
2025-01-27T20:43:05.752679Z DEBUG day_21: Building full_paths. r_paths=["Av<<A>^>A", "Av<<A>>^A", "A<v<A>^>A", "A<v<A>>^A"], local_paths=[["v<A", "<vA"], ["<A"], ["A"], [">^>A", ">>^A"], ["vA"], ["^<A", "<^A"], ["v>A", ">vA"], ["^A"], ["v<A", "<vA"], ["<A"], ["A"], [">^>A", ">>^A"], ["vA"], ["A"], ["^<A", "<^A"], [">A"], ["v<<A", "<v<A"], [">A"], ["<A"], [">^>A", ">>^A"], ["vA"], ["^<A", "<^A"], ["v>A", ">vA"], ["^A"], ["v<<A", "<v<A"], [">A"], ["<A"], [">^>A", ">>^A"], ["vA"], ["A"], ["^<A", "<^A"], [">A"]]

r_paths=["Av<<A>^>A", "Av<<A>>^A", "A<v<A>^>A", "A<v<A>>^A"]
local_paths=
[
["v<A", "<vA"],
["<A"],
["A"],
[">^>A", ">>^A"],
["vA"],
["^<A", "<^A"],
["v>A", ">vA"],
["^A"],
["v<A", "<vA"],
["<A"],
["A"],
[">^>A", ">>^A"],
["vA"],
["A"],
["^<A", "<^A"],
[">A"], ["v<<A", "<v<A"], [">A"], ["<A"], [">^>A", ">>^A"], ["vA"], ["^<A", "<^A"], ["v>A", ">vA"], ["^A"], ["v<<A", "<v<A"], [">A"], ["<A"], [">^>A", ">>^A"], ["vA"], ["A"], ["^<A", "<^A"], [">A"]]
        */

        let r1 = Robot { next: Box::new(RobotOrNumpad::Numpad) };
        let r2 = Robot { next: Box::new(RobotOrNumpad::Robot(r1)) };
        let r3 = Robot { next: Box::new(RobotOrNumpad::Robot(r2)) };

        let actual = r3.paths('A', '0');
        //debug!("{:?}", actual);
        //let expected_len = 0;
        //assert_eq!(expected_len, actual.len());

        let expected_shortest_path = "AAAAA";
        let actual_shortest_path = actual.iter().min_by_key(|a| a.len()).unwrap();
        assert_eq!(expected_shortest_path, actual_shortest_path);

        let actual_answer = actual.iter().next().unwrap();
        let expected_answer = "v<<A>^>A";
        assert_eq!(expected_answer, actual_answer);

        Ok(())
    }
}
