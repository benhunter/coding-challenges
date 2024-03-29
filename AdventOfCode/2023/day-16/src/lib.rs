use crate::Direction::{Down, Left, Right, Up};

pub fn solve_part1(input: &str) -> usize {
    let mut contraption = parse(input);
    let start = Node { coord: Coord { x: 0, y: 0 }, direction: Right };
    contraption.calc_energized(start);
    contraption.count_engergized()
}


pub fn solve_part2(input: &str) -> i32 {
    let mut contraption = parse(input);
    let mut max = 0;

    for x in 0..contraption.layout[0].len() {
        contraption.reset();
        let y = 0;
        let start = Node { coord: Coord { x, y }, direction: Down };
        contraption.calc_energized(start);
        max = max.max(contraption.count_engergized());

        contraption.reset();
        let y = contraption.layout.len() - 1;
        let start = Node { coord: Coord { x, y }, direction: Up };
        contraption.calc_energized(start);
        max = max.max(contraption.count_engergized());
    }

    for y in 0..contraption.layout.len() - 1 {
        contraption.reset();
        let x = 0;
        let start = Node { coord: Coord { x, y }, direction: Right };
        contraption.calc_energized(start);
        max = max.max(contraption.count_engergized());

        contraption.reset();
        let x = contraption.layout[0].len() - 1;
        let start = Node { coord: Coord { x, y }, direction: Left };
        contraption.calc_energized(start);
        max = max.max(contraption.count_engergized());
    }

    max as i32
}

#[derive(Debug, Clone)]
struct Contraption {
    layout: Vec<Vec<char>>,
    visited: Vec<Vec<[bool; 4]>>, // Up, Down, Left, Right
}

impl Contraption {
    pub fn new(layout: Vec<Vec<char>>) -> Self {
        let visited = vec![vec![[false; 4]; layout[0].len()]; layout.len()];
        Self {
            layout,
            visited,
        }
    }

    pub fn reset(&mut self) {
        self.visited = vec![vec![[false; 4]; self.layout[0].len()]; self.layout.len()]; // TODO store x,y dimensions
    }

    fn go_direction(&self, direction: &Direction, x: usize, y: usize) -> Option<Node> {
        match direction {
            Up if y > 0 => Some(Node::new(x, y - 1, Up)),
            Down if y < self.layout.len() - 1 => Some(Node::new(x, y + 1, Down)),
            Left if x > 0 => Some(Node::new(x - 1, y, Left)),
            Right if x < self.layout[0].len() - 1 => Some(Node::new(x + 1, y, Right)),
            _ => None,
        }
    }

    fn mark_visited(&mut self, direction: &Direction, x: usize, y: usize) {
        self.visited[y][x][*direction as usize] = true
    }

    fn is_visited(&self, direction: &Direction, x: usize, y: usize) -> bool {
        self.visited[y][x][*direction as usize]
    }

    pub fn calc_energized(&mut self, start: Node) {
        let mut node_queue: Vec<Node> = vec![start];

        while let Some(node) = node_queue.pop() {
            let last = node;
            let x = last.coord.x;
            let y = last.coord.y;

            if self.is_visited(&last.direction, x, y) {
                continue;
            } else {
                self.mark_visited(&last.direction, x, y);
            }

            let next: Option<Node> = match self.layout[y][x] {
                '.' => self.go_direction(&last.direction, x, y),

                '|' => match last.direction {
                    Up | Down => self.go_direction(&last.direction, x, y),
                    Left | Right => {
                        let up = self.go_direction(&Up, x, y);
                        if let Some(up) = up {
                            node_queue.push(up);
                        }
                        self.go_direction(&Down, x, y)
                    }
                }

                '-' => match last.direction {
                    Left | Right => self.go_direction(&last.direction, x, y),
                    Up | Down => {
                        let left = self.go_direction(&Left, x, y);
                        if let Some(left) = left {
                            node_queue.push(left);
                        }
                        self.go_direction(&Right, x, y)
                    }
                }

                '/' => match last.direction {
                    Right => self.go_direction(&Up, x, y),
                    Up => self.go_direction(&Right, x, y),
                    Left => self.go_direction(&Down, x, y),
                    Down => self.go_direction(&Left, x, y),
                }

                '\\' => match last.direction {
                    Right => self.go_direction(&Down, x, y),
                    Down => self.go_direction(&Right, x, y),
                    Up => self.go_direction(&Left, x, y),
                    Left => self.go_direction(&Up, x, y),
                }

                _ => { panic!("should not happen") }
            };

            if let Some(next) = next {
                node_queue.push(next);
            }
        }
    }

    pub fn count_engergized(&self) -> usize {
        let count: usize = self.visited.iter().map(|line| {
            line.iter()
                .filter(|coord| coord.contains(&true))
                .count()
        }).sum();
        count
    }

    #[allow(dead_code)]
    pub fn print_energized(&self) {
        for y in 0..self.visited.len() - 1 {
            for x in 0..self.visited[0].len() - 1 {
                if self.visited[y][x].contains(&true) {
                    print!("#");
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


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
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
    use crate::{parse, solve_part1, solve_part2};

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

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input);
        let solution = 6994;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        let actual = solve_part2(input);
        let solution = 51;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input);
        let solution = 7488;
        assert_eq!(actual, solution);
    }
}
