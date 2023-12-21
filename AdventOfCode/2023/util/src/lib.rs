use crate::Direction::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
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
