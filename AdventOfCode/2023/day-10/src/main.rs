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

        println!("{:?}", field.visualize_distances());

        if steps > 10 {
            break;
        }
    }
    0
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
            // find a pipe pointing to the start
            self.a = Some(self.next_pipes(&self.start, None, None));
            self.b = Some(self.next_pipes(&self.start, None, Some(self.a.clone().unwrap().coord)));

            // set distance to 1
            let coord = self.a.clone().unwrap().coord;
            self.distances[coord.0][coord.1] = 1;
            let coord = self.b.clone().unwrap().coord;
            self.distances[coord.0][coord.1] = 1;
            return Ok(());
        }

        self.a = Option::from(self.find_next_pipe(self.a.clone().unwrap()));

        let mut last_a = self.a.clone().unwrap();
        let mut distance = 1;
        while last_a.next.is_some() {
            last_a = *last_a.next.unwrap();
            distance += 1;
        }
        self.distances[last_a.coord.0][last_a.coord.1] = distance;

        println!("{}", self.visualize_distances());

        dbg!(&self.b);
        self.b = Option::from(self.find_next_pipe(self.b.clone().unwrap()));
        dbg!(&self.b);

        let mut last_b = self.b.clone().unwrap();
        let mut distance = 1;
        while last_b.next.is_some() {
            last_b = *last_b.next.unwrap();
            distance += 1;
        }
        self.distances[last_b.coord.0][last_b.coord.1] = distance;

        println!("{}", self.visualize_distances());
        Ok(())
    }

    fn find_next_pipe(&self, mut pipe: Pipe) -> Pipe {
        let mut current: &mut Pipe = &mut pipe;
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        // TODO ignore previous coord
        let next = self.next_pipes(&current, None, Some(current.coord));
        dbg!(&next);
        current.next = Some(Box::new(next));
        pipe
    }

    pub(crate) fn next_pipes(&self, from: &Pipe, ignore: Option<Direction>, ignore_coord: Option<(usize, usize)>) -> Pipe {
        // up - check for |, 7, F
        let mut coords: Vec<(usize, usize)> = Vec::new();
        if !ignore.as_ref().is_some_and(|d| *d == Direction::Up)
            && matches!(self.pipes[from.coord.0-1][from.coord.1], '|'|'7'|'F') {
            coords.push((from.coord.0 - 1, from.coord.1));
        }

        // right - check for -, J, 7
        if !ignore.as_ref().is_some_and(|d| *d == Direction::Right)
            && matches!(self.pipes[from.coord.0][from.coord.1+1], '-'|'J'|'7') {
            println!("right");
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
        dbg!(ignore_coord);
        coords.retain(|c| ignore_coord.is_none() || *c != ignore_coord.unwrap());
        let mut pipes: Vec<Pipe> = coords.iter().map(|c| Pipe { coord: *c, next: None }).collect();
        pipes.remove(0)
    }

    pub(crate) fn distance(&self, coord: (usize, usize)) -> Option<i32> {
        let dist = self.distances[coord.0][coord.1];
        Some(dist)
    }

    pub(crate) fn visualize_distances(&self) -> String {
        let mut result = String::new();
        for row in self.distances.iter() {
            for col in row.iter() {
                if *col == -1 {
                    result.push_str(" .");
                } else {
                    result.push_str(&format!("{:2}", col));
                }
                result.push_str(" ");
            }
            result.push_str("\n");
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
        for j in 0..pipe.len() {
            if pipe[j] == 'S' {
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
        field.step().expect("TODO: panic message");
        assert_eq!(field.distance((1, 2)).unwrap(), 1);
        assert_eq!(field.distance((2, 1)).unwrap(), 1);

        let actual = field.visualize_distances();
        println!("{}", actual);
        let expected = " .  .  .  .  . \n \
            .  0  1  .  . \n \
            .  1  .  .  . \n \
            .  .  .  .  . \n \
            .  .  .  .  . \n";
        assert_eq!(actual, expected);

        field.step().expect("TODO: panic message");
        assert_eq!(field.distance((1, 3)).unwrap(), 2);
        assert_eq!(field.distance((3, 1)).unwrap(), 2);

        field.step().expect("TODO: panic message");
        assert_eq!(field.distance((2, 3)).unwrap(), 3);
        assert_eq!(field.distance((3, 2)).unwrap(), 3);

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
