fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    // let result = solve_part2(input);
    // println!("✅ part2: {}", result);
}

#[derive(Debug, PartialEq, Clone)]
struct Wire {
    path: Vec<Segment>,
}

struct Segment {
    direction: Direction,
    distance: u32,
}

impl Segment {
    fn new(direction: Direction, distance: u32) -> Self {
        Segment { direction, distance }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_wire(input: &str) -> Wire {
    Wire {
        path: input
            .split(',')
            .map(|x| Segment {
                direction: Direction::from_char(x.chars().next()),
                distance: 0,
            })
            .collect()
    }
}

fn solve_part1(input: &str) -> i32 {
    0
}

fn solve_part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::Direction::{Down, Left, Right, Up};
    use super::*;

    #[test]
    fn test_parse() {
        let input = "R8,U5,L5,D3";
        let output = Wire {
            path: vec!(
                Segment::new(Right, 8),
                Segment::new(Up, 5),
                Segment::new(Left, 5),
                Segment::new(Down, 3),
            )
        };
        assert_eq!(parse_wire(input), output);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test1.txt");
        let solution = 0;
        assert_eq!(solve_part1(input), solution);
    }

    // #[test]
    fn test_solve2() {
        let input = include_str!("../test1.txt");
        let solution = 0;
        assert_eq!(solve_part2(input), solution);
    }

    // #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let solution = 0;
        assert_eq!(solve_part1(input), solution);
    }

    // #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let solution = 0;
        assert_eq!(solve_part2(input), solution);
    }
}