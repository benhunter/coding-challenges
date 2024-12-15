use std::str::FromStr;

use crate::Direction::*;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }

    pub fn from_str(s: String) -> Coord {
        //Coord { x:0 , y:0 }
        todo!()
    }

    pub fn go(&self, direction: &Direction, bound: &Coord) -> Option<Coord> {
        match direction {
            Up if self.y > 0 => Some(Coord { x: self.x, y: self.y - 1 }),
            Down if self.y < bound.y - 1 => Some(Coord { x: self.x, y: self.y + 1 }),
            Left if self.x > 0 => Some(Coord { x: self.x - 1, y: self.y }),
            Right if self.x < bound.x - 1 => Some(Coord { x: self.x + 1, y: self.y }),
            _ => None,
        }
    }

    pub fn neighbors(&self, bound: &Coord) -> Vec<Coord> {
        Direction::iter().filter_map(|direction| self.go(&direction, bound)).collect()
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
            .map(|a| a.parse::<usize>().unwrap());
        let c = Coord {
            x: tokens.next().expect("usize expected"),
            y: tokens.next().expect("usize expected")
        };
        Ok(c)
    }

    type Err = ParseError;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn iter() -> core::array::IntoIter<Direction, 4> {
        [
            Down,
            Right,
            Left,
            Up,
        ].into_iter()
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Distance {
    Infinity,
    Value(usize),
}

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
