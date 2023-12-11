fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    // let result = solve_part2(input);
    // println!("✅ part2: {}", result);
}

fn solve_part1(input: &str) -> i32 {
    let mut field = parse(input);
    let mut steps = 0;
    loop {
        println!("step: {}", steps);
        let step = field.step();
        // dbg!(&field);


        // when a and b are on the same pipe, return the distance
        let a_last = field.a.clone().unwrap().last();
        let b_last = field.b.clone().unwrap().last();
        println!("a: {:?}, b: {:?}", a_last, b_last);
        if a_last.coord == b_last.coord {
            return steps;
        }


        match step {
            Ok(_) => steps += 1,
            Err(_) => break,
        }

        if steps > 10 {
            break;
        }
    }
    0
}

fn solve_part2(input: &str) -> i32 {
    0
}

#[derive(Debug, PartialEq, Clone)]
struct Field {
    pipes: Vec<Vec<char>>,
    start: Pipe,
    distances: Vec<Vec<i32>>,
    a: Option<Pipe>,
    b: Option<Pipe>,
}

impl Field {
    pub(crate) fn step(&mut self) -> Result<(), &'static str> {
        if self.a.is_none() && self.b.is_none() {
            // find a pipe pointing to the start
            let pipes = self.next_pipes(&self.start, None, None);
            self.a = Some(pipes[0].clone());
            self.b = Some(pipes[1].clone());

            // set distance to 1
            let coord = self.a.clone().unwrap().coord;
            self.distances[coord.0][coord.1] = 1;
            let coord = self.b.clone().unwrap().coord;
            self.distances[coord.0][coord.1] = 1;
            // dbg!(&self.a);
            // dbg!(&self.b);
            return Ok(());
        }

        let result_a = self.get_next_pipe(&self.a);
        let result_b = self.get_next_pipe(&self.b);
        if result_a.is_none() && result_b.is_none() {
            // no more pipes
            return Err("no more pipes");
        }
        Ok(())
    }

    fn get_next_pipe(&self, pipe: &Option<Pipe>) -> Option<Pipe> {
        let mut current = pipe.clone().unwrap();
        while current.next.is_some() {
            current = *current.next.unwrap();
        }
        // dbg!(&curent);
        let next = self.next_pipes(&current, None, Some(current.coord));
        if next.is_empty() {
            return None
        }
        Some(next[0].clone())

        // TODO
        // self.distances[current.next.as_ref().unwrap().coord.0][current.next.as_ref().unwrap().coord.1] = self.distance(current.coord).unwrap() + 1;
    }

    pub(crate) fn next_pipes(&self, from: &Pipe, ignore: Option<Direction>, ignore_coord: Option<(usize, usize)>) -> Vec<Pipe> {
        // dbg!(&from, &ignore, &ignore_coord);
        // if self.pipes[self.start.coord.0][self.start.coord.1]
        // up - check for |, 7, F
        let mut coords: Vec<(usize, usize)> = Vec::new();
        if !ignore.as_ref().is_some_and(|d| *d == Direction::Up)
            && matches!(self.pipes[from.coord.0-1][from.coord.1], '|'|'7'|'F') {
            coords.push((from.coord.0 - 1, from.coord.1));
        }

        // right - check for -, J, 7
        if !ignore.as_ref().is_some_and(|d| *d == Direction::Right)
            && matches!(self.pipes[from.coord.0][from.coord.1+1], '-'|'J'|'7') {
            coords.push((from.coord.0, from.coord.1 + 1));
        }

        // down - check for |, L, J
        if !ignore.as_ref().is_some_and(|d| *d == Direction::Down)
            && matches!(self.pipes[from.coord.0+1][from.coord.1], '|'|'L'|'J') {
            coords.push((from.coord.0 + 1, from.coord.1));
        }

        // left - check for -, L, F
        if !ignore.as_ref().is_some_and(|d| *d == Direction::Left)
            && matches!(self.pipes[from.coord.0][from.coord.1-1], '-'|'L'|'F') {
            coords.push((from.coord.0, from.coord.1 - 1));
        }

        // filter the ignored coord
        coords.retain(|c| ignore_coord.is_none() || *c != ignore_coord.unwrap());
        let pipes = coords.iter().map(|c| Pipe { coord: *c, next: None }).collect();
        // dbg!(&pipes);
        pipes
    }

    pub(crate) fn distance(&self, coord: (usize, usize)) -> Option<i32> {
        let dist = self.distances[coord.0][coord.1];
        if dist == -1 {
            None
        } else {
            Some(dist)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Pipe {
    coord: (usize, usize),
    next: Option<Box<Pipe>>,
}

impl Pipe {
    fn last(&self) -> Pipe {
        let mut current = self.clone();
        while current.next.is_some() {
            current = *current.next.unwrap();
        }
        current
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse(input: &str) -> Field {
    let pipes = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut start = Pipe { coord: (0, 0), next: None };
    for (i, pipe) in pipes.iter().enumerate() {
        for j in 0..pipes[i].len() {
            if pipes[i][j] == 'S' {
                start.coord = (i, j); // = Node{ coord: (i, j), a: None, b: None };
            }
        }
    }
    println!("{:?}", pipes[1][1].clone());
    let distances = vec![vec![-1; pipes[0].len()]; pipes.len()];
    Field { pipes, start, distances, a: None, b: None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../test.txt");
        let actual = parse(input).pipes.len();
        let expected = 5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_square_loop() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let mut field = parse(input);
        assert_eq!(field.start, Pipe { coord: (1, 1), next: None });
        field.step();
        assert_eq!(field.distance((2, 1)).unwrap(), 1);
        assert_eq!(field.distance((1, 2)).unwrap(), 1);
        field.step();
        assert_eq!(field.distance((3, 1)).unwrap(), 2);
        assert_eq!(field.distance((1, 3)).unwrap(), 2);
        field.step();
        assert_eq!(field.distance((3, 2)).unwrap(), 3);
        assert_eq!(field.distance((2, 3)).unwrap(), 3);

        // let actual = solve_part1(input);
        // let solution = 4;
        // assert_eq!(actual, solution);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input);
        let solution = 8;
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
