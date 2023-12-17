use std::panic::panic_any;
use crate::Direction::*;

fn main() {
    // let input = include_str!("../input.txt");
    let input = include_str!("../test.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    // let result = solve_part2(input);
    // println!("✅ part2: {}", result);
}

#[derive(Debug)]
struct Contraption {
    layout: Vec<Vec<char>>,
    paths: Vec<Path>,
}

impl Contraption {
    fn new(layout: Vec<Vec<char>>) -> Self {
        Self {
            layout,
            paths: Vec::new(),
        }
    }

    fn go_direction(&self, direction: &Direction, x: usize, y: usize) -> Option<Node> {
        match direction {
            Up => {
                if y > 0 {
                    Some(Node::new(x, y - 1, Up))
                } else {
                    None
                }
            }
            Down => {
                if y < self.layout.len() - 1 {
                    Some(Node::new(x, y + 1, Down))
                } else {
                    None
                }
            }
            Left => {
                if x > 0 {
                    Some(Node::new(x - 1, y, Left))
                } else {
                    None
                }
            }
            Right => {
                if x < self.layout[0].len() - 1 {
                    Some(Node::new(x + 1, y, Right))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    route: Vec<Node>,
}

#[derive(Debug, Clone)]
struct Node {
    coord: Coord,
    direction: Direction,
}

impl Node {
    fn new(x: usize, y: usize, direction: Direction) -> Node {
        Node {
            coord: Coord { x, y },
            direction,
        }
    }
}

#[derive(Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn solve_part1(input: &str) -> i32 {
    let contraption = parse(input);
    let mut partial_paths: Vec<Path> = Vec::new();
    partial_paths.push(Path {
        route: vec![Node { coord: Coord { x: 0, y: 0 }, direction: Right }]
    });
    let mut completed_paths: Vec<Path> = Vec::new();

    while !partial_paths.is_empty() {
        if partial_paths.len() > 10 {
            panic!("too many paths");
        }
        // dbg!(partial_paths.clone().len());
        let mut path = partial_paths.pop().unwrap();
        let last = path.route.iter().last().unwrap();
        // dbg!(last.clone());
        let x = last.coord.x;
        let y = last.coord.y;

        println!("x: {}, y: {}, char: {}, direction: {:?}, paths: {}", x, y, contraption.layout[y][x], last.direction, partial_paths.len());
        match contraption.layout[y][x] {
            '.' => {
                let next = contraption.go_direction(&last.direction, x, y);
                if next.is_some() {
                    // dbg!(next.clone());
                    path.route.push(next.unwrap());
                    partial_paths.push(path);
                } else {
                    println!("completed path");
                    completed_paths.push(path);
                }
            }

            '|' => {
                match last.direction {
                    Up | Down => {
                        // continue like '.'
                        let next = contraption.go_direction(&last.direction, x, y);
                        if next.is_some() {
                            path.route.push(next.unwrap());
                            partial_paths.push(path);
                        } else {
                            println!("completed path");
                            completed_paths.push(path);
                        }
                    }
                    Left | Right => {
                        println!("split |");
                        let up = contraption.go_direction(&Up, x, y);
                        if up.is_some() {
                            let mut up_path = path.clone();
                            up_path.route.push(up.unwrap());
                            partial_paths.push(up_path);
                        }
                        let down = contraption.go_direction(&Down, x, y);
                        if down.is_some() {
                            path.route.push(down.unwrap());
                            partial_paths.push(path);
                        }
                    }
                }
            }

            '-' => {
                match last.direction {
                    Left | Right => {
                        // continue like '.'
                        let next = contraption.go_direction(&last.direction, x, y);
                        if next.is_some() {
                            path.route.push(next.unwrap());
                            partial_paths.push(path);
                        } else {
                            println!("completed path");
                            completed_paths.push(path);
                        }
                    }
                    Up | Down => {
                        println!("split -");
                        let left = contraption.go_direction(&Left, x, y);
                        if left.is_some() {
                            let mut up_path = path.clone();
                            up_path.route.push(left.unwrap());
                            partial_paths.push(up_path);
                        }
                        let right = contraption.go_direction(&Right, x, y);
                        if right.is_some() {
                            path.route.push(right.unwrap());
                            partial_paths.push(path);
                        }
                    }
                }
            }

            '/' => {
                let next = match last.direction {
                    Right => contraption.go_direction(&Up, x, y),
                    Up => contraption.go_direction(&Right, x, y),
                    Left => contraption.go_direction(&Down, x, y),
                    Down => contraption.go_direction(&Left, x, y),
                };
                if next.is_some() {
                    path.route.push(next.unwrap());
                    partial_paths.push(path);
                } else {
                    println!("completed path");
                    completed_paths.push(path);
                }
            }

            '\\' => {
                let next = match last.direction {
                    Right => contraption.go_direction(&Down, x, y),
                    Down => contraption.go_direction(&Right, x, y),
                    Up => contraption.go_direction(&Left, x, y),
                    Left => contraption.go_direction(&Up, x, y),
                };
                if next.is_some() {
                    path.route.push(next.unwrap());
                    partial_paths.push(path);
                } else {
                    println!("completed path");
                    completed_paths.push(path);
                }
            }

            _ => {
                dbg!(y, x, contraption.layout[y][x]);
                todo!()
            }
        }

        // partial_paths.pop()
    }

    todo!()
}


fn solve_part2(input: &str) -> i32 {
    0
}

fn parse(input: &str) -> Contraption {
    let layout = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    Contraption::new(layout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../test.txt");
        let actual = parse(input);
        assert_eq!(actual.layout.len(), 10);
        assert_eq!(actual.layout[0].len(), 10);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input);
        let solution = 46;
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
