use crate::Direction::*;
use crate::Distance::{Infinity, Value};

pub fn solve_part1(input: &str) -> usize {
    let mut city = parse(input);
    let start = Coord { x: 0, y: 0 };
    let bound = Coord { x: city.layout[0].len(), y: city.layout.len() };
    let destination = Coord { x: bound.x - 1, y: bound.y - 1 };

    // let mut paths_queue = vec![vec![start]];
    // let mut paths_queue = vec![Path(vec![start])];

    let mut distances = vec![vec![Infinity; bound.x]; bound.y];
    distances[start.y][start.x] = Value(0);
    let mut visited = vec![vec![false; bound.x]; bound.y];
    let mut previous: Vec<Vec<(Option<Coord>, Option<Direction>)>> = vec![vec![(None, None); bound.x]; bound.y];

    loop {
        // if visited.iter().all(|blocks| !blocks.contains(&false)) {
        //     break;
        // }

        let mut current = None;
        for y in 0..bound.y {
            for x in 0..bound.x {
                if !visited[y][x] && distances[y][x] != Infinity {
                    current = Some(Coord::new(x, y));
                    break;
                }
            }
            if current.is_some() {
                break;
            }
        }
        if current.is_none() {
            break;
        }

        let current = current.unwrap();
        println!("current: {:?}", current);

        for direction in Direction::iter() {
            if let Some(next) = current.go(&direction, &bound) {
                println!("current: {:?}, next: {:?}, direction: {:?}", current, next, direction);

                if current == Coord::new(4, 0) {
                    visualize_previous(&previous, &city);
                    visualize_distances(&distances);
                    let debug_path = build_path(&start, bound, &current, &previous);
                    visualize_path(&debug_path, &city);
                    println!("🚫🐞");
                }
                // if current == Coord::new(4, 1) && direction == Up {
                //     println!("debug");
                // }

                // TODO only consider next if the line is 3 or less
                // track path, check last 3 for same line
                // whatever direction we are going, look opposite
                let prev1 = &previous[current.y][current.x];
                if prev1.0.is_some() {
                    if direction == *prev1.1.as_ref().unwrap() {
                        let prev2 = &previous[prev1.0.unwrap().y][prev1.0.unwrap().x];
                        if prev2.0.is_some() {
                            if prev1.1.as_ref().unwrap() == prev2.1.as_ref().unwrap() {
                                let prev3 = &previous[prev2.0.unwrap().y][prev1.0.unwrap().x];
                                if prev3.0.is_some() {
                                    if prev2.1.as_ref().unwrap() == prev3.1.as_ref().unwrap() {
                                        println!("can't go more than 3 in the same direction");
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }

                // consider unvisited neighbors
                // if !visited[next.y][next.x] { // TODO bug here - current: Coord { x: 4, y: 1 }, next: Coord { x: 4, y: 0 }, direction: Up
                match distances[current.y][current.x] {
                    Value(current_value) => {
                        let new_dist = current_value + city.layout[next.y][next.x] as usize;

                        match distances[next.y][next.x] {
                            Infinity => {
                                distances[next.y][next.x] = Value(new_dist);
                                println!("set x: {}, y: {}, distance: {}", next.x, next.y, new_dist);
                                previous[next.y][next.x] = (Some(current), Some(direction));
                            }

                            Value(dist) => {
                                if new_dist < dist {
                                    distances[next.y][next.x] = Value(new_dist);
                                    println!("updated x: {}, y: {}, distance: {}", next.x, next.y, new_dist);
                                    previous[next.y][next.x] = (Some(current), Some(direction));
                                }
                            }
                        }
                    }

                    Infinity => {
                        panic!("must have current_value")
                    }
                }
                // }
            }
        }

        visited[current.y][current.x] = true;
        println!("set visited x: {}, y: {}", current.x, current.y);
    }

    let best_previous = build_path(&start, bound, &destination, &mut previous);

    visualize_distances(&distances);
    visualize_path(&best_previous, &city);
    visualize_best_previous(&best_previous, &city);
    visualize_previous(&previous, &city);
    let Value(d) = distances[bound.y - 1][bound.x - 1] else { panic!("impossible 🧐") };
    d
}

fn build_path<'a>(to: &Coord, bound: Coord, from: &Coord, previous: &'a Vec<Vec<(Option<Coord>, Option<Direction>)>>) -> Vec<Vec<&'a (Option<Coord>, Option<Direction>)>> {
    let mut path: Vec<Vec<&(Option<Coord>, Option<Direction>)>> = vec![vec![&(None, None); bound.x]; bound.y];
    let mut current = *from;
    while current != *to {
        path[current.y][current.x] = &previous[current.y][current.x];
        let previous_coord = &previous[current.y][current.x].0.clone();
        current = previous_coord.unwrap();
    }
    path
}

fn visualize_distances(distances: &Vec<Vec<Distance>>) {
    for y in 0..distances.len() {
        for x in 0..distances[0].len() {
            match distances[y][x] {
                Infinity => print!(". "),
                Value(v) => {
                    print!("{:>3} ", v);
                }
            }
        }
        println!();
    }
    println!();
}

fn visualize_path(previous: &Vec<Vec<&(Option<Coord>, Option<Direction>)>>, city: &City) {
    for y in 0..previous.len() {
        for x in 0..previous[0].len() {
            match &previous[y][x].1 {
                Some(direction) => match direction {
                    Up => print!("^"),
                    Down => print!("v"),
                    Left => print!("<"),
                    Right => print!(">"),
                }
                None => {
                    // print!(".")
                    print!("{}", city.layout[y][x]);
                }
            }
        }
        println!();
    }
    println!();
}

fn visualize_best_previous(previous: &Vec<Vec<&(Option<Coord>, Option<Direction>)>>, city: &City) {
    for y in 0..previous.len() {
        for x in 0..previous[0].len() {
            match &previous[y][x].0 {
                Some(coord) => print!("({:2},{:2})", coord.x, coord.y),
                None => print!("       "),
            }
        }
        println!();
    }
    println!();
}

fn visualize_previous(previous: &Vec<Vec<(Option<Coord>, Option<Direction>)>>, city: &City) {
    for y in 0..previous.len() {
        for x in 0..previous[0].len() {
            match &previous[y][x].0 {
                Some(coord) => print!("({:2},{:2})", coord.x, coord.y),
                None => print!("       "),
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug, Clone, PartialEq)]
enum Distance {
    Infinity,
    Value(usize),
}

pub fn solve_part2(input: &str) -> i32 {
    0
}

#[derive(Debug, PartialEq, Clone)]
struct City {
    layout: Vec<Vec<u8>>,
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

#[derive(Debug, Clone, PartialEq, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub(crate) fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }

    pub(crate) fn go(&self, direction: &Direction, bound: &Coord) -> Option<Coord> {
        match direction {
            Up if self.y > 0 => Some(Coord { x: self.x, y: self.y - 1 }),
            Down if self.y < bound.y - 1 => Some(Coord { x: self.x, y: self.y + 1 }),
            Left if self.x > 0 => Some(Coord { x: self.x - 1, y: self.y }),
            Right if self.x < bound.x - 1 => Some(Coord { x: self.x + 1, y: self.y }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    pub(crate) fn opposite(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

fn parse(input: &str) -> City {
    let layout = input
        .lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect())
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
        assert_eq!(actual.layout[0][0], 2);
    }

    #[test]
    fn test_go() {
        let input = "11\n11";
        let city = parse(input);
        // let path = Path(vec![Coord::new(1, 0)]);
        let go = Coord::new(0, 0).go(&Left, &Coord::new(2, 2));
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
