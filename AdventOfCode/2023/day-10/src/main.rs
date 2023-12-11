fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result.1);

    let result = solve_part2(input);
    println!("✅ part2: {}", result.1);
}

fn solve_part1(input: &str) -> (Field, i32) {
    let mut field = parse(input);
    let mut steps = 0;
    loop {
        let step = field.step();
        let a_last = field.a.as_mut().unwrap().last();
        let b_last = field.b.as_mut().unwrap().last();


        match step {
            Ok(_) => steps += 1,
            Err(_) => break,
        }

        // when a and b are on the same pipe, loop is complete
        if a_last.coord == b_last.coord {
            print!("{} ", steps);
            break;
        }
    }
    (field, steps)
}

fn solve_part2(input: &str) -> (Field, i32) {
    let part1 = solve_part1(input);
    let mut field = part1.0;
    println!("field:\n{}", field.visualize_distances(true));

    // replace S with correct pipe
    field.pipes[field.start.coord.0][field.start.coord.1] = field.calculate_start_pipe();
    print!("DEBUG new start pipe: {}", field.pipes[field.start.coord.0][field.start.coord.1]);
    println!(" at coord: {:?}", field.start.coord);

    for (i, line) in field.pipes.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if field.distances[i][j] == -1 {
                // include junk
                // if *char == '.' {
                field.in_out[i][j] = field.calculate_in_out((i, j));
            }
        }
    }

    println!("field:\n{}", field.visualize_in_out(true));

    let count_in = field.in_out.iter().map(|line| line.iter().filter(|c| **c == 'I').count()).sum::<usize>() as i32;

    (field, count_in)
}

#[derive(Debug, PartialEq, Clone)]
struct Field {
    pipes: Vec<Vec<char>>,
    start: Pipe,
    distances: Vec<Vec<i32>>,
    a: Option<Pipe>,
    b: Option<Pipe>,
    in_out: Vec<Vec<char>>,
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

    pub(crate) fn visualize_without_junk(&self, with_pipes: bool) -> String {
        let mut result = String::new();
        for (i, row) in self.distances.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == -1 {
                    result.push_str(".");
                } else {
                    result.push_str(&format!("{}", self.pipes[i][j]));
                }
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

    fn calculate_start_pipe(&self) -> char {
        let a_rel_start = self.a.as_ref().unwrap().relative_to(self.start.coord);
        let b_rel_start = self.b.as_ref().unwrap().relative_to(self.start.coord);

        println!("DEBUG a_rel_start: {:?} b_rel_start: {:?}", a_rel_start, b_rel_start);

        match a_rel_start {
            Direction::North => {
                match b_rel_start {
                    Direction::East => 'L',
                    Direction::South => '|',
                    Direction::West => 'J',
                    _ => panic!("a_rel_start: {:?} b_rel_start: {:?}", a_rel_start, b_rel_start),
                }
            }

            Direction::East => {
                match b_rel_start {
                    Direction::North => 'L',
                    Direction::South => 'F',
                    Direction::West => '-',
                    _ => panic!("a_rel_start: {:?} b_rel_start: {:?}", a_rel_start, b_rel_start),
                }
            }

            Direction::South => {
                match b_rel_start {
                    Direction::North => '|',
                    Direction::East => 'F',
                    Direction::West => '7',
                    _ => panic!("a_rel_start: {:?} b_rel_start: {:?}", a_rel_start, b_rel_start),
                }
            }
            Direction::West => {
                match b_rel_start {
                    Direction::North => 'J',
                    Direction::East => '-',
                    Direction::South => '7',
                    _ => panic!("a_rel_start: {:?} b_rel_start: {:?}", a_rel_start, b_rel_start),
                }
            }
        }
    }

    fn calculate_in_out(&self, coord: (usize, usize)) -> char {
        let mut debug = false;
        // if coord == (4, 8) {
        if coord == (8, 12) {
            debug = true;
        }
        // out:
        // out if touching border
        // out if touching another out

        // out if count loops up is even number
        let mut count = 0;
        let mut pipe_count = 0;
        let mut touch_stack: Vec<char> = Vec::new();

        for i in (0..coord.0).rev() {
            // ignore junk
            if self.distances[i][coord.1] == -1 {
                continue;
            }
            // except for '|'
            if debug {
                // println!("DEBUG {} {}", i, coord.1);
                print!("DEBUG {}", self.pipes[i][coord.1]);
            }

            match self.pipes[i][coord.1] {
                'S' => panic!("S should not be in the middle of the field"),
                '|' => pipe_count += 1,
                'J' => {
                    touch_stack.push('J');
                }
                'L' => {
                    touch_stack.push('L');
                }
                '7' => {
                    if touch_stack.last().is_some() && *touch_stack.last().unwrap() == 'J' {
                        touch_stack.pop();
                    } else if touch_stack.last().is_some() && *touch_stack.last().unwrap() == 'L' {
                        touch_stack.pop();
                        count += 1;
                    } else {
                        panic!("match 7: {:?}", touch_stack);
                        touch_stack.push('7');
                    }
                }
                'F' => {
                    if touch_stack.last().is_some() && *touch_stack.last().unwrap() == 'J' {
                        touch_stack.pop();
                        count += 1;
                    } else if touch_stack.last().is_some() && *touch_stack.last().unwrap() == 'L' {
                        touch_stack.pop();
                    } else {
                        panic!("match F: {:?}", touch_stack);
                        touch_stack.push('F');
                    }
                }
                _ => {
                    if debug {
                        panic!("match _: {}", self.pipes[i][coord.1]);
                    }
                    count += 1
                }
            }

            if debug {
                print!(" stack: {:?}", touch_stack);
                println!(" count: {}", count);
            }
        }

        // if pipe_count > 0 {
        //     count -= 1; // TODO does this underflow?
        // }
        if count % 2 == 0 {
            return 'O';
        }
        'I'
    }

    pub(crate) fn visualize_in_out(&self, with_pipes: bool) -> String {
        let mut result = String::new();
        for (i, row) in self.in_out.iter().enumerate() {
            for col in row.iter() {
                result.push_str(&format!("{}", col));
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
    pub(crate) fn relative_to(&self, other: (usize, usize)) -> Direction {
        if self.coord.0 < other.0 {
            Direction::North
        } else if self.coord.0 > other.0 {
            Direction::South
        } else {
            // match (self.coord.1 - other.1) as i32 {
            //     0 => panic!("relative_to: same coord"),
            //     1 => Direction::East,
            //     -1 => Direction::West,
            //     _ => panic!("relative_to: same coord"),
            // }
            if self.coord.1 < other.1 {
                Direction::West
            } else if self.coord.1 > other.1 {
                Direction::East
            } else {
                panic!("relative_to: same coord");
            }
        }
    }
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
    Field { pipes: pipes.clone(), start, distances, a: None, b: None, in_out: pipes }
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

        let actual = solve_part1(input).1;
        let solution = 4;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input).1;
        let solution = 8;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input).1;
        let solution = 6842;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test2.txt");
        let actual = solve_part2(input);
        let field = actual.0;
        println!("field:\n{}", field.visualize_distances(true));
        println!("field:\n{}", field.visualize_in_out(true));

        let actual_in_out = field.visualize_in_out(false);
        let expected_in_out = "OOOOOOOOOOO
OS-------7O
O|F-----7|O
O||OOOOO||O
O||OOOOO||O
O|L-7OF-J|O
O|II|O|II|O
OL--JOL--JO
OOOOOOOOOOO
";
        assert_eq!(actual_in_out, expected_in_out);

        let solution = 4;
        assert_eq!(actual.1, solution);
    }

    #[test]
    fn test_part2_test3() {
        let input = include_str!("../test3.txt");
        let actual = solve_part2(input);
        let field = actual.0;
        println!("field distances:\n{}", field.visualize_distances(true));
        println!("field in_out:\n{}", field.visualize_in_out(true));

        let actual_in_out = field.visualize_in_out(false);
        let expected_in_out = "OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO\n";

        println!("actual_in_out:\n{}", actual_in_out);
        println!("expected_in_out:\n{}", expected_in_out);
        assert_eq!(actual_in_out, expected_in_out);

        let solution = 8;
        assert_eq!(actual.1, solution);
    }

    #[test]
    fn test_part2_test4() {
        let input = include_str!("../test4.txt");
        let actual = solve_part2(input);
        let field = actual.0;
        println!("field:\n{}", field.visualize_distances(true));
        println!("field:\n{}", field.visualize_in_out(true));

        let actual_in_out = field.visualize_in_out(false);
        let expected_in_out = "OOOOOOOOOO
OS------7O
O|F----7|O
O||OOOO||O
O||OOOO||O
O|L-7F-J|O
O|II||II|O
OL--JL--JO
OOOOOOOOOO\n";

        assert_eq!(actual_in_out, expected_in_out);

        let solution = 4;
        assert_eq!(actual.1, solution);
    }

    #[test]
    fn test_part2_test5() {
        let input = include_str!("../test5.txt");
        let actual = solve_part2(input);
        let field = actual.0;
        println!("field:\n{}", field.visualize_distances(true));
        println!("field without junk:\n{}", field.visualize_without_junk(true));
        println!("field in_out:\n{}", field.visualize_in_out(true));

        let actual_in_out = field.visualize_in_out(false);
        let expected_in_out = "OF7FSF7F7F7F7F7F---7
O|LJ||||||||||||F--J
OL-7LJLJ||||||LJL-7O
F--JF--7||LJLJIF7FJO
L---JF-JLJIIIIFJLJOO
OOOF-JF---7IIIL7OOOO
OOFJF7L7F-JF7IIL---7
OOL-JL7||F7|L7F-7F7|
OOOOOFJ|||||FJL7||LJ
OOOOOL-JLJLJL--JLJOO\n";

        assert_eq!(actual_in_out, expected_in_out);

        let solution = 10;
        assert_eq!(actual.1, solution);
    }

    #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input);
        let actual = actual.1;
        let solution = 393;
        assert_eq!(actual, solution);
    }
}
