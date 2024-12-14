use util::ParseError;
use std::str::FromStr;
use std::ops::Mul;
use std::ops::Add;

pub fn solve_part1(input: &str) -> Result<u32, String> {
    let machines = parse(input)?;
    let solution: u32 = machines.iter().map(|m| m.solve()).sum();
    Ok(solution)
}

pub fn solve_part2(_input: &str) -> Result<i32, String> {
    //println!("{}", input);
    todo!()
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
struct Point {
    x: u32,
    y: u32,
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

        fn parse(tokens: &str) -> u32 {
            tokens
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        }

        let x = parse(tokens[0]);
        let y = parse(tokens[1]);

        Ok(Point {x, y})
    }

    type Err = ParseError;
}

impl Mul<u32> for Point {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Point { x: self.x * rhs,
            y: self.y * rhs }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y}
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

impl Machine {
    fn solve (self) -> u32 {
        let mut a = 0; // a presses
        let mut b = 0; // b presses

        b = self.prize.x / self.btn_a.x + 1;
        if b > 100 {
            b = 100;
        }

        let mut position = self.btn_a * a + self.btn_b * b;

        let mut loops = 0;
        let mut solved = false;

        while loops < 100_000 {
            position = self.btn_a * a + self.btn_b * b;
            //println!("inner presses a={}, b={} position=({}, {})", a, b, position.x, position.y);
            if position == self.prize {
                println!("[DEBUG Machine::solve()] Found prize in outer loop (dec b)");
                break
            }

            if position.x <= self.prize.x && position.y <= self.prize.y {
                let mut inner_loops = 0;
                while inner_loops <= 100 && position.x <= self.prize.x && position.y <= self.prize.y {
                    let position = self.btn_a * a + self.btn_b * b;
                    //println!("inner presses a={}, b={} position=({}, {})", a, b, position.x, position.y);
                    if position == self.prize {
                        solved = true;
                        println!("[DEBUG Machine::solve()] Found prize in inner loop (inc a)");
                        break
                    }
                    a += 1;
                    inner_loops += 1;
                }

                if solved {
                    println!("[DEBUG Machine::solve()] Breaking outer loop for inner loop");
                    break;
                }
                a = 0;
            }
            if b == 0 {
                return 0;
            }
            b -= 1;
            loops += 1;
        }
        println!("[DEBUG Machine::solve()] loops={} presses a={}, b={} position={:?}, prize={:?}, btn_a={:?}, btn_b={:?}", loops, a, b, position, self.prize, self.btn_a, self.btn_b);

        assert!(((self.btn_a * a) + (self.btn_b * b)) == self.prize);
        assert_eq!(position, self.prize);
        a * 3 + b
    }
}

fn parse(input: &str) -> Result<Vec<Machine>, ParseError> {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    //println!("{:?}", sections);
    let machines = sections.iter()
        .map(|section| {
            let mut lines = section.lines();
            let btn_a = lines.next().unwrap().parse::<Point>().unwrap();
            let btn_b = lines.next().unwrap().parse::<Point>().unwrap();
            let prize = lines.next().unwrap().parse::<Point>().unwrap();
            Machine { btn_a, btn_b, prize }
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
        let actual = parse(input)?.len() > 0;
        let expected = true;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_point_mul_u32() -> Result<(), String> {
        let p = Point { x: 1, y: 2 };
        let x = 3;
        let px = p * x;
        let expected = Point { x: 3, y: 6 };
        assert_eq!(px, expected);
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
        let solution = 28314; // too low
        assert!(solution > 28314);
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    //fn test_part2() -> Result<(), String> {
    //    let input = include_str!("../test.txt");
    //    let actual = solve_part2(input)?;
    //    let solution = 0;
    //    assert_eq!(actual, solution);
    //    Ok(())
    //}

    // #[test]
    //fn test_solve_part2() -> Result<(), String> {
    //    let input = include_str!("../input.txt");
    //    let actual = solve_part2(input)?;
    //    let solution = 0;
    //    assert_eq!(actual, solution);
    //    Ok(())
    //}
}
