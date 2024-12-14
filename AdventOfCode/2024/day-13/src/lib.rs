#![feature(f128)]

use util::ParseError;
use std::fmt;
use std::str::FromStr;
use std::ops::Mul;
use std::ops::Add;

const PART1_MAX_PRESSES: u64 = 100;
const PART2_MAX_PRESSES: u64 = 10000000000000;
const PRECISION: f64 = 0.01;

pub fn solve_part1(input: &str) -> Result<u64, String> {
    let machines = parse(input, 0)?;
    //let solution: u64 = machines.iter().map(|m| m.solve()).sum();
    //Ok(solution)

    //println!("[DEBUG solve_part1()] machines len={}", machines.len());
    let solutions = machines
        .iter()
        .enumerate()
        .map(|(i, m)| {
            //println!("[DEBUG solve_part1()] solving machine: i={}, m={}", i, m);
            //let s = m.solve(PART1_MAX_PRESSES);
            let s = m.solve();
            //if s == 0 {
            //    println!("no prize: {}, {}", i, m);
            //} else {
            //    println!("prize: {}, {}", i, m);
            //}
            s
        })
        .sum();
    Ok(solutions)
}

pub fn solve_part2(input: &str) -> Result<u64, String> {
    let machines = parse(input, PART2_MAX_PRESSES)?;
    //let solution: u64 = machines.iter().map(|m| m.solve()).sum();
    //Ok(solution)

    let solutions = machines
        .iter()
        .enumerate()
        .map(|(i, m)| {
            //println!("[DEBUG solve_part1()] solving machine: i={}, m={}", i, m);
            let s = m.solve();
            //if s == 0 {
            //    println!("no prize: {}, {}", i, m);
            //} else {
            //    println!("prize: {}, {}", i, m);
            //}
            s
        })
        .sum();
    Ok(solutions)
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct Point {
    x: u64,
    y: u64,
}

impl FromStr for Point {
    fn from_str(s: &str) -> Result<Point, ParseError> {
        let tokens = s
            .split(':')
            .map(|s| s.trim())
            .nth(1)
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>();

        //println!("{:?}", tokens);

        fn parse(tokens: &str) -> u64 {
            tokens
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        }

        let x = parse(tokens[0]);
        let y = parse(tokens[1]);

        Ok(Point {x, y})
    }

    type Err = ParseError;
}

impl Mul<u64> for Point {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Point { x: self.x * rhs,
            y: self.y * rhs }
    }
}

impl Add<u64> for Point {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Point { 
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct Machine {
    btn_a: Point,
    btn_b: Point,
    prize: Point,
}

impl FromStr for Machine {
    fn from_str(s: &str) -> Result<Machine, ParseError> {
        let mut lines = s.lines();
        let btn_a = lines.next().unwrap().parse().unwrap();
        let btn_b = lines.next().unwrap().parse().unwrap();
        let prize = lines.next().unwrap().parse().unwrap();
        let m = Machine { btn_a, btn_b, prize };
        Ok(m)
    }

    type Err = ParseError;
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Machine: A={} B={} Prize={}", self.btn_a, self.btn_b, self.prize)
    }
}

impl Machine {
    /**
     * Button A is 3 tokens. B is 1 token. Find all solutions!
     */
    fn solve1(self, max_presses: u64) -> u64 {
        println!("solving: {}", self);
        let mut a = 0; // a presses
        let mut b = 0; // b presses

        let mut position = self.update_position(a, b);

        let mut loops = 0;
        let mut solutions: Vec<(u64, u64)> = vec![];

        while b <= max_presses && position.x <= self.prize.x && position.y <= self.prize.y {
            //if position.x < self.prize.x && position.y < self.prize.y {
                a = 0;
                let mut inner_loops = 0;
                while a <= max_presses && position.x <= self.prize.x && position.y <= self.prize.y {
                    if position == self.prize {
                        println!("prize at a={}, b={}", a, b);
                        solutions.push((a, b));
                    }
                    a += 1;
                    position = self.update_position(a, b);
                    inner_loops += 1;
                    if inner_loops % 100000000 == 0 {
                        println!("inner_loops={}", inner_loops);
                    }
                }
            //}

            a = 0;
            b += 1;
            position = self.update_position(a, b);
            loops += 1;
            if loops % 10 == 0 {
                println!("loops={}", loops);
            }
        }
        println!("solutions={:?}", solutions);
        if solutions.len() > 1 {
            println!("Found multiple solutions");
            todo!()
        }
        let solution = match solutions.len() {
            0 => &(0, 0),
            _ => {
                solutions
                    .iter()
                    .min_by(|l, r| {
                        println!("[DEBUG Machine::solve()] min_by l={:?}, r={:?}", l, r);
                        l.0.cmp(&r.0)
                    })
                    .unwrap()
            }
        };

        solution.0 * 3 + solution.1
    }

    fn solve(self) -> u64 {
        let a = ((self.prize.x as f64/self.btn_a.x as f64) - (((self.prize.y as f64 - self.prize.x as f64 * self.btn_a.y as f64 / self.btn_a.x as f64) / (self.btn_b.y as f64 - self.btn_b.x as f64 * self.btn_a.y as f64 / self.btn_a.x as f64)) * self.btn_b.x as f64 / self.btn_a.x as f64));
        let b = ((self.prize.y as f64 - self.prize.x as f64 * self.btn_a.y as f64 / self.btn_a.x as f64) / (self.btn_b.y as f64 - self.btn_b.x as f64 * self.btn_a.y as f64 / self.btn_a.x as f64));
        //println!("solution a={:?}, b={:?}", a, b);

        if (a.round()-a).abs() > PRECISION || (b.round() - b).abs() > PRECISION {
            return 0
        }
        let a = a.round() as u64;
        let b = b.round() as u64;
        a * 3 + b
    }

    fn update_position(&self, a: u64, b: u64) -> Point {
        self.btn_a * a + self.btn_b * b
    }

}

fn parse(input: &str, add: u64) -> Result<Vec<Machine>, ParseError> {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    //println!("{:?}", sections);
    let machines = sections.iter()
        .map(|section| {
            let mut lines = section.lines();
            let btn_a = lines.next().unwrap().parse::<Point>().unwrap();
            let btn_b = lines.next().unwrap().parse::<Point>().unwrap();
            let prize = lines.next().unwrap().parse::<Point>().unwrap();
            Machine { btn_a, btn_b, prize: prize + add }
        })
        .collect();
    Ok(machines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() -> Result<(), String> {
        let input = "Button A: X+94, Y+34";
        let actual = input.parse::<Point>().unwrap();
        let expected = Point {x: 94, y: 34};
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse_machine() -> Result<(), String> {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        //println!("{}", input);
        let actual = input.parse::<Machine>().unwrap();
        let btn_a = Point { x: 94, y: 34 };
        let btn_b = Point { x: 22, y: 67 };
        let prize = Point { x: 8400, y: 5400 };
        let expected = Machine { btn_a, btn_b, prize };
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        //println!("{}", input);
        let actual = parse(input, 0)?.len() > 0;
        let expected = true;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_point_mul_u64() -> Result<(), String> {
        let p = Point { x: 1, y: 2 };
        let x = 3;
        let px = p * x;
        let expected = Point { x: 3, y: 6 };
        assert_eq!(px, expected);
        Ok(())
    }

    #[test]
    fn test_point_add() -> Result<(), String> {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: 2 };
        let a_plus_b = Point { x: 2, y: 4 };
        assert_eq!(a + b, a_plus_b);
        assert_eq!(a * 0 + b, b);
        assert_eq!(a + b * 0, a);
        Ok(())
    }

    #[test]
    fn test_solve_machine() -> Result<(), String> {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        //println!("{}", input);
        let machine = input.parse::<Machine>().unwrap();
        let actual = machine.solve();
        let expected = 280;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_solve_machines_no_solution() -> Result<(), String> {
        // input.txt machine 0
        let input = r"Button A: X+46, Y+68
Button B: X+34, Y+14
Prize: X=11306, Y=10856";
        //println!("{}", input);
        let machine = input.parse::<Machine>().unwrap();
        let actual = machine.solve();
        let expected = 0;
        assert_eq!(actual, expected);

        // Example 2
        let input = r"Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
        //println!("{}", input);
        let machine = input.parse::<Machine>().unwrap();
        let actual = machine.solve();
        let expected = 0;
        assert_eq!(actual, expected);

        // Example 4
        let input = r"Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        //println!("{}", input);
        let machine = input.parse::<Machine>().unwrap();
        let actual = machine.solve();
        let expected = 0;
        assert_eq!(actual, expected);

        // Example 1 part 2
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        let machine = parse(input, PART2_MAX_PRESSES)?.into_iter().next().unwrap();
        let actual = machine.solve();
        let expected = 0;
        assert_eq!(actual, expected);

        // Example 3 part 2
        let input = r"Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450";
        let machine = parse(input, PART2_MAX_PRESSES)?.into_iter().next().unwrap();
        let actual = machine.solve();
        let expected = 0;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_solve_machines_part2_big_solution() -> Result<(), String> {
        // Example 2
        let input = r"Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
        let machine = parse(input, PART2_MAX_PRESSES)?.into_iter().next().unwrap();
        let actual = machine.solve();
        let expected = 459236326669;
        assert_eq!(actual, expected);

        // Example 4
        let input = r"Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let machine = parse(input, PART2_MAX_PRESSES)?.into_iter().next().unwrap();
        let actual = machine.solve();
        let expected = 416082282239;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test.txt");
        //println!("{}", input);
        let actual = solve_part1(input)?;
        let solution = 480;
        assert_eq!(actual, solution);
        Ok(())
    }

    #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input1.txt");
        let actual = solve_part1(input)?;
        let expected = 30973;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_solvers_part1_and_2_same() -> Result<(), String> {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        let machine = input.parse::<Machine>().unwrap();
        let solver1 = machine.solve1(PART1_MAX_PRESSES);
        let solver2 = machine.solve();
        assert_eq!(solver1, solver2);
        Ok(())
    }

     #[test]
    fn test_part2() -> Result<(), String> {
        let input = include_str!("../test.txt");
        //let machine = input.parse::<Machine>().unwrap();
        let actual = solve_part2(input)?;
        let solution = 875318608908; // machines 2 and 4 have solutions, 1 and 3 are not
        assert_eq!(actual, solution);
        Ok(())
    }

     #[test]
    fn test_solve_part2() -> Result<(), String> {
        let input = include_str!("../input1.txt");
        let actual = solve_part2(input)?;
        let solution = 95688837203288; // 161516197152513 too high
                          // 48295572837373
                          // 1545093008499 too low
                          // 95688837203288
        assert_eq!(actual, solution);
        Ok(())
    }
}
