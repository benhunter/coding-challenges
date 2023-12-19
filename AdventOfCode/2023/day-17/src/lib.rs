use crate::Direction::*;

pub fn solve_part1(input: &str) -> i32 {
    let mut city = parse(input);
    let start = Coord { x: 0, y: 0 };
    let bound = Coord { x: city.layout[0].len(), y: city.layout.len() };
    let destination = Coord { x: bound.x - 1, y: bound.y - 1 };

    // let mut paths_queue = vec![vec![start]];
    let mut paths_queue = vec![Path(vec![start])];

    while let Some(mut current_path) = paths_queue.pop() {
        println!("paths_queue len: {}", paths_queue.len());
        let last_block = current_path.0.last().unwrap();

        if last_block == &destination {
            println!("path arrived at destination");
            city.visualize(&current_path);
            // dbg!(current_path);
            continue
        }

        // path will never cross itself (or loop)
        for direction in Direction::iter() {
            let mut path = current_path.clone();
            // go 1,2,3 distance in all direction
            // dbg!(direction.clone());

            if let Some(next) = last_block.go(&direction, &bound, &path.0) {
                path.0.push(next);
                paths_queue.push(path.clone());
            }
            // dbg!(paths_queue);
        }
    }
    // dbg!(paths_queue);

    todo!()
}

pub fn solve_part2(input: &str) -> i32 {
    0
}

#[derive(Debug, PartialEq, Clone)]
struct City {
    layout: Vec<Vec<char>>,
}

impl City {
    pub(crate) fn visualize(&self, path: &Path) {
        println!("path len: {}", path.0.len());
        for y in 0..self.layout.len() {
            for x in 0..self.layout[0].len() {
                if path.0.contains(&Coord::new(x, y)) {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct Path(Vec<Coord>);

#[derive(Debug, Clone, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub(crate) fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }

    pub(crate) fn go(&self, direction: &Direction, bound: &Coord, path: &Vec<Coord>) -> Option<Coord> {
        let next = match direction {
            Up if self.y > 0 => Some(Coord { x: self.x, y: self.y - 1 }),
            Down if self.y < bound.y - 1 => Some(Coord { x: self.x, y: self.y + 1 }),
            Left if self.x > 0 => Some(Coord { x: self.x - 1, y: self.y }),
            Right if self.x < bound.x - 1 => Some(Coord { x: self.x + 1, y: self.y }),
            _ => None,
        };

        if let Some(next) = next {
            return if path.contains(&next) {
                None
            } else {
                Some(next)
            };
        }
        None
    }
}

#[derive(Debug)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn iter() -> core::array::IntoIter<Direction, 4> {
        [
            Direction::Down,
            Direction::Right,
            Direction::Left,
            Direction::Up,
        ].into_iter()
    }
}

fn parse(input: &str) -> City {
    let layout = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    City { layout }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../test.txt");
        let actual = parse(input);
        assert_eq!(actual.layout.len(), 13);
        assert_eq!(actual.layout[0].len(), 13);
    }

    #[test]
    fn test_go() {
        let input = "11\n11";
        let city = parse(input);
        let path = Path(vec![Coord::new(1, 0)]);
        let go = Coord::new(0, 0).go(&Right, &Coord::new(2, 2), &path.0);
        assert_eq!(go, None);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input);
        let solution = 102;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input);
        let solution = 0;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        let actual = solve_part2(input);
        let solution = 0;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input);
        let solution = 0;
        assert_eq!(actual, solution);
    }
}

