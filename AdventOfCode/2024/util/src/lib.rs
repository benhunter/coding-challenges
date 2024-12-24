use core::panic;
use std::{cmp::Ordering, fmt::Display, str::FromStr};

use crate::Direction::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Coord {
        Coord { x, y }
    }

    pub fn go_bound(&self, direction: &Direction, bound: &Coord) -> Option<Coord> {
        match direction {
            Up if self.y > 0 => Some(Coord { x: self.x, y: self.y - 1 }),
            Down if self.y < bound.y - 1 => Some(Coord { x: self.x, y: self.y + 1 }),
            Left if self.x > 0 => Some(Coord { x: self.x - 1, y: self.y }),
            Right if self.x < bound.x - 1 => Some(Coord { x: self.x + 1, y: self.y }),
            _ => None,
        }
    }

    pub fn go(&self, direction: Direction) -> Coord {
        let (dx, dy) = match direction {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        };
        Coord::new(self.x + dx, self.y + dy)
    }

    pub fn neighbors(&self, bound: &Coord) -> Vec<Coord> {
        Direction::iter().filter_map(|direction| self.go_bound(&direction, bound)).collect()
    }

    pub fn add(&self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl FromStr for Coord {
    fn from_str(s: &str) -> Result<Coord, ParseError> {
        let mut tokens = s
            .split(',')
            .map(|a| a.parse::<i64>().unwrap());
        let c = Coord {
            x: tokens.next().expect("i64 expected"),
            y: tokens.next().expect("i64 expected")
        };
        Ok(c)
    }

    type Err = ParseError;
}

//impl PartialEq for Coord {
//    fn eq(&self, other: &Self) -> bool {
//        todo!()
//    }
//}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
        //let x_cmp = self.x.partial_cmp(&other.x).unwrap();
        //let y_cmp = self.y.partial_cmp(&other.y).unwrap();
        ////println!("comparing {:?} to {:?}. result {:?} and {:?}", self, other, x_cmp, y_cmp);
        //
        //if x_cmp == y_cmp {
        //    Some(x_cmp)
        //} else if (x_cmp == Ordering::Less || x_cmp == Ordering::Equal) && (y_cmp == Ordering::Less || y_cmp == Ordering::Equal) {
        //    Some(Ordering::Less)
        //} else {
        //    Some(Ordering::Greater)
        //}
    }
}

//impl PartialEq for Coord {
//    fn eq(&self, other: &Self) -> bool {
//        self.x == other.x && self.y == other.y
//    }
//}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x == other.x && self.y == other.y {
            Ordering::Equal
        } else {
            self.partial_cmp(other).unwrap();
            panic!("I don't trust this. see partial_cmp()")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn iter() -> core::array::IntoIter<Direction, 4> {
        [
            Up,
            Right,
            Down,
            Left,
        ].into_iter()
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    pub fn left(&self) -> Direction {
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    pub fn right(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

impl From<Direction> for i8 {
    fn from(value: Direction) -> Self {
        match value {
            Up => 0,
            Right => 1,
            Down => 2,
            Left => 3,
        }
    }
}

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Up => '^',
            Right => '>',
            Down => 'v',
            Left => '<',
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = match self {
            Up => "Up",
            Right => "Right",
            Down => "Down",
            Left => "Left",
        };
        write!(f, "{}", d)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Distance {
    Value(i64),
    Infinity,
}

impl Distance {
    pub fn add_i64(self, rhs: i64) -> Self {
        match self {
            Distance::Value(v) => Distance::Value(v + rhs),
            Distance::Infinity => Distance::Infinity,
        }
    }
}
//impl PartialEq for Distance {
//    fn eq(&self, other: &Self) -> bool {
//        let eq = (*self == Distance::Infinity && *other == Distance::Infinity);
//        let s = if let Distance::Value(s) = self {
//            s
//        } else {
//            panic!("not a Distance::Value")
//        };
//    }
//}

//impl Ord for Distance {
//    fn cmp(&self, other: &Self) -> Ordering {
//        if *self == Distance::Infinity && *other == Distance::Infinity {
//            Ordering::Equal
//        } else {
//            match self {
//                Distance::Value(s) => {
//                    match other {
//                        Distance::Value(o) => {
//                            s.cmp(o)
//                        },
//                        Distance::Infinity => Ordering::Less
//                    }
//                },
//                Distance::Infinity => Ordering::Greater
//            }
//        }
//    }
//} 

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl FromStr for Vector {
    fn from_str(s: &str) -> Result<Vector, ParseError> {
        let mut tokens = s
            .split(',')
            .map(|a| a.parse().unwrap());
        let v = Vector {
            x: tokens.next().expect("i64 expected"),
            y: tokens.next().expect("i64 expected")
        };
        Ok(v)
    }

    type Err = ParseError;
}

impl Vector {
    pub fn new(x: i64, y: i64) -> Vector {
        Vector { x, y }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidCharacter(char),
    BadInput,
}

impl From<ParseError> for String {
    fn from(error: ParseError) -> Self {
        match error {
            ParseError::InvalidCharacter(c) => format!("Invalid character: {}", c),
            ParseError::BadInput => "Bad input".to_string(),
        }
    }
}

pub fn parse_grid_chars(input: &str) -> Result<Vec<Vec<char>>, ParseError> {
    parse_grid(input, Ok)
}

pub fn parse_grid<T>(input: &str, parser: impl Fn(char) -> Result<T, ParseError>) -> Result<Vec<Vec<T>>, ParseError> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(&parser)
                .collect
                    ::<Result<Vec<T>, ParseError>>()
        })
        .collect()
}

// How to get the (x, y) Coord of a specific tile/cell in a grid:
//fn find_in_grid(matcher: fn(i64)-> bool ) -> Option<Coord> {
//    self.grid
//        .iter()
//        .enumerate()
//        .map(|(yi, y)| {
//            y
//           .iter()
//           .enumerate()
//           .map(move |(xi, x)| (xi, yi, x))
//        })
//        .flatten()
//        .filter(matcher)
//}

