use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use util::{Coord, ParseError, Vector};

pub fn solve_part1(input: &str) -> Result<i32, String> {
    let item = parse(input)?;
    println!("{}", item);
    todo!()
}

pub fn solve_part2(input: &str) -> Result<i32, String> {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Robot {
    start_pos: Coord,
    position: Coord,
    velocity: Vector,
}

impl FromStr for Robot {
    fn from_str(s: &str) -> Result<Robot, ParseError> {
        let mut tokens = s
            .split(' ')
            .map(|s| s
                .trim()
                .chars()
                .skip(2)
                .collect::<String>()
            );

        let p = tokens.next().unwrap().parse::<Coord>().unwrap();
        let v = tokens.next().unwrap().parse::<Vector>().unwrap();
        let r = Robot { start_pos: p, position: p, velocity: v};
        Ok(r)
    }

    type Err = ParseError;
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Ebhq {
    robots: Vec<Robot>,
}

impl Display for Ebhq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //for r in &self.robots  {
        //    write!(f, "{}", r);
        //}
        write!(f, "Item")
    }
}

fn parse(input: &str) -> Result<Ebhq, ParseError> {

    let robots = input
        .lines()
        .map(|line| {
            //Robot::from(line)
            line.parse::<Robot>().unwrap()
        })
        .collect();
    Ok(Ebhq { robots })
}

#[cfg(test)]
mod tests {
    use util::Vector;

    use super::*;

    #[test]
    fn test_parse_robot() -> Result<(), String> {
        let input = "p=0,4 v=3,-3";
        let actual = input.parse::<Robot>().unwrap();
        let expected = Robot {
            start_pos: Coord::new(0, 4),
            position: Coord::new(0, 4),
            velocity: Vector::new(3, -3),
        };
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = parse(input)?;
        let expected = Ebhq { ..Default::default() };
        assert_eq!(actual, expected);
        Ok(())
    }

    //#[test]
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
