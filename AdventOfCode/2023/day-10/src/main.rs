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
        let step = field.step();
        let a_last = field.a.as_mut().unwrap().last();
        let b_last = field.b.as_mut().unwrap().last();

        // when a and b are on the same pipe, loop is complete
        if a_last.coord == b_last.coord {
            print!("{} ", steps);
            return steps + 1;
        }

        match step {
            Ok(_) => steps += 1,
            Err(_) => break,
        }
        // println!("{}", field.visualize_distances(true));
    }
    panic!("No solution found. steps: {}", steps);
}

// fn solve_part2(input: &str) -> i32 {
//     0
// }

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
            self.a = Some(self.next_pipes_from_start(&self.start, None, None));
            self.b = Some(self.next_pipes_from_start(&self.start, None, Some(self.a.as_ref().unwrap().coord)));

            let coord = self.a.as_ref().unwrap().coord;
            self.distances[coord.0][coord.1] = 1;
            let coord = self.b.as_ref().unwrap().coord;
            self.distances[coord.0][coord.1] = 1;
            return Ok(());
        }

        let next_a = self.find_next_pipe(self.a.as_ref().unwrap());

        let mut last_a = self.a.as_mut().unwrap();
        let mut distance = 1;
        while last_a.next.is_some() {
            last_a = last_a.next.as_mut().unwrap();
            distance += 1;
        }
        last_a.next = Some(Box::new(next_a.clone()));
        self.distances[next_a.coord.0][next_a.coord.1] = distance + 1;

        let next_b = self.find_next_pipe(self.b.as_ref().unwrap());

        let mut last_b = self.b.as_mut().unwrap();
        let mut distance = 1;
        while last_b.next.is_some() {
            last_b = last_b.next.as_mut().unwrap();
            distance += 1;
        }
        last_b.next = Some(Box::new(next_b.clone()));
        self.distances[next_b.coord.0][next_b.coord.1] = distance + 1;

        Ok(())
    }

    fn find_next_pipe(&self, pipe: &Pipe) -> Pipe {
        let mut previous_coord = self.start.coord;
        let mut current: &Pipe = pipe;
        while current.next.is_some() {
            previous_coord = current.coord;
            current = current.next.as_ref().unwrap();
        }

        self.next_pipe(current, None, Some(previous_coord))
    }

    pub(crate) fn next_pipes_from_start(&self, from: &Pipe, ignore: Option<Direction>, ignore_coord: Option<(usize, usize)>) -> Pipe {
        let mut coords: Vec<Option<(usize, usize)>> = vec![
            self.get_north(from, &ignore),
            self.get_east(from, &ignore),
            self.get_south(from, &ignore),
            self.get_west(from, &ignore),
        ];

        coords.retain(|c| c.is_some());
        coords.retain(|c| ignore_coord.is_none() || ignore_coord.is_some_and(|ic| c.unwrap() != ic));

        Pipe { coord: coords[0].unwrap(), next: None }
    }

    pub(crate) fn next_pipe(&self, from: &Pipe, ignore: Option<Direction>, ignore_coord: Option<(usize, usize)>) -> Pipe {
        let mut directions: Vec<Direction> = Vec::new();
        match self.pipes[from.coord.0][from.coord.1] {
            '|' => {
                directions.push(Direction::North);
                directions.push(Direction::South);
            }
            '-' => {
                directions.push(Direction::West);
                directions.push(Direction::East);
            }
            'L' => {
                directions.push(Direction::North);
                directions.push(Direction::East);
            }
            'J' => {
                directions.push(Direction::North);
                directions.push(Direction::West);
            }
            '7' => {
                directions.push(Direction::South);
                directions.push(Direction::West);
            }
            'F' => {
                directions.push(Direction::South);
                directions.push(Direction::East);
            }
            _ => panic!("Unknown pipe: {}", self.pipes[from.coord.0][from.coord.1]),
        }

        let mut coords: Vec<(usize, usize)> = Vec::new();
        for dir in directions {
            let coord = match dir {
                Direction::North => self.get_north(from, &ignore),
                Direction::East => self.get_east(from, &ignore),
                Direction::South => self.get_south(from, &ignore),
                Direction::West => self.get_west(from, &ignore),
            };
            if let Some(coord) = coord {
                coords.push(coord);
            }
        }

        coords.retain(|c| ignore_coord.is_none() || *c != ignore_coord.unwrap());
        Pipe { coord: coords[0], next: None }
    }

    fn get_north(&self, from: &Pipe, ignore: &Option<Direction>) -> Option<(usize, usize)> {
        if !ignore.as_ref().is_some_and(|d| *d == Direction::North)
            && (from.coord.0 > 0)
            && matches!(self.pipes[from.coord.0-1][from.coord.1], '|'|'7'|'F') {
            return Some((from.coord.0 - 1, from.coord.1));
        }
        None
    }

    fn get_east(&self, from: &Pipe, ignore: &Option<Direction>) -> Option<(usize, usize)> {
        if !ignore.as_ref().is_some_and(|d| *d == Direction::East)
            && (from.coord.1 < self.pipes[from.coord.0].len() - 1)
            && matches!(self.pipes[from.coord.0][from.coord.1+1], '-'|'J'|'7') {
            return Some((from.coord.0, from.coord.1 + 1));
        }
        None
    }

    fn get_south(&self, from: &Pipe, ignore: &Option<Direction>) -> Option<(usize, usize)> {
        if !ignore.as_ref().is_some_and(|d| *d == Direction::South)
            && (from.coord.0 < self.pipes.len() - 1)
            && matches!(self.pipes[from.coord.0+1][from.coord.1], '|'|'L'|'J') {
            return Some((from.coord.0 + 1, from.coord.1));
        }
        None
    }

    fn get_west(&self, from: &Pipe, ignore: &Option<Direction>) -> Option<(usize, usize)> {
        if !ignore.as_ref().is_some_and(|d| *d == Direction::West)
            && (from.coord.1 > 0)
            && matches!(self.pipes[from.coord.0][from.coord.1-1], '-'|'L'|'F') {
            return Some((from.coord.0, from.coord.1 - 1));
        }
        None
    }

    #[allow(dead_code)]
    pub(crate) fn distance(&self, coord: (usize, usize)) -> Option<i32> {
        let dist = self.distances[coord.0][coord.1];
        Some(dist)
    }

    pub(crate) fn visualize_distances(&self, with_pipes: bool) -> String {
        let mut result = String::new();
        for (i, row) in self.distances.iter().enumerate() {
            for col in row.iter() {
                if *col == -1 {
                    result.push_str(" .");
                } else {
                    result.push_str(&format!("{:2}", col));
                }
                result.push(' ');
            }
            if with_pipes {
                // let line = self.pipes[i].iter().collect::<String>();
                result.push_str(
                    &format!("\t{}", self.pipes[i].iter().collect::<String>())
                );
            }
            result.push('\n');
        }
        result
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Pipe {
    coord: (usize, usize),
    next: Option<Box<Pipe>>,
}

impl Pipe {
    fn last(&self) -> &Pipe {
        let mut current = self;
        while current.next.is_some() {
            current = current.next.as_deref().unwrap();
        }
        current
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse(input: &str) -> Field {
    let pipes = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut start = Pipe { coord: (0, 0), next: None };
    for (i, pipe) in pipes.iter().enumerate() {
        for (j, char) in pipe.iter().enumerate() {
            if *char == 'S' {
                start.coord = (i, j); // = Node{ coord: (i, j), a: None, b: None };
            }
        }
    }
    let mut distances = vec![vec![-1; pipes[0].len()]; pipes.len()];
    for i in 0..distances.len() {
        for j in 0..distances[i].len() {
            if pipes[i][j] == 'S' {
                distances[i][j] = 0;
            }
        }
    }
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
        field.step().expect("Testing field.step() failed");
        assert_eq!(field.distance((1, 2)).unwrap(), 1);
        assert_eq!(field.distance((2, 1)).unwrap(), 1);

        let actual = field.visualize_distances(false);
        println!("{}", actual);
        let expected = " .  .  .  .  . \n \
            .  0  1  .  . \n \
            .  1  .  .  . \n \
            .  .  .  .  . \n \
            .  .  .  .  . \n";
        assert_eq!(actual, expected);

        field.step().expect("Testing field.step() failed");
        assert_eq!(field.distance((1, 3)).unwrap(), 2);
        assert_eq!(field.distance((3, 1)).unwrap(), 2);

        field.step().expect("Testing field.step() failed");
        assert_eq!(field.distance((2, 3)).unwrap(), 3);
        assert_eq!(field.distance((3, 2)).unwrap(), 3);

        field.step().expect("Testing field.step() failed");
        assert_eq!(field.distance((3, 3)).unwrap(), 4);

        // stop when a and b are on the same pipe
        // field.step().expect("Testing field.step() failed");
        let a = field.a.as_ref().unwrap();
        let a_last = a.last();
        let b = field.b.as_ref().unwrap();
        let b_last = b.last();
        assert_eq!(a_last.coord, b_last.coord);
        assert_eq!(field.distance((1, 3)).unwrap(), 2);
        assert_eq!(field.distance((3, 1)).unwrap(), 2);
        assert_eq!(field.distance((2, 3)).unwrap(), 3);
        assert_eq!(field.distance((3, 2)).unwrap(), 3);
        assert_eq!(field.distance((3, 3)).unwrap(), 4);

        let actual = solve_part1(input);
        let solution = 4;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input);
        let solution = 8;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input);
        let solution = 6842;
        assert_eq!(actual, solution);
    }

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("../test.txt");
    //     let actual = solve_part2(input);
    //     let solution = 0;
    //     assert_eq!(actual, solution);
    // }

    // #[test]
    // fn test_solve_part2() {
    //     let input = include_str!("../input.txt");
    //     let actual = solve_part2(input);
    //     let solution = 0;
    //     assert_eq!(actual, solution);
    // }
}
