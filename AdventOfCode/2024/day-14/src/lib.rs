use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use util::{Coord, ParseError, Vector};

pub fn solve_part1(input: &str) -> Result<i64, String> {
    let max = Coord::new(101, 103);
    let mut ebhq = Ebhq::new(input, max);
    //println!("{}", ebhq.visualize());
    ebhq.step(100)?;
    let solution = ebhq.solve();
    //println!("{}", ebhq.visualize());
    //println!("{}", solution);
    Ok(solution)
}

pub fn solve_part2(input: &str) -> Result<i64, String> {
    let max = Coord::new(101, 103);
    let mut ebhq = Ebhq::new(input, max);
    //println!("{}", ebhq.visualize());
    let mut steps = 0;

    loop {
        ebhq.step(1)?;
        steps += 1;
        match ebhq.scan() {
            Some(c) => {
                println!("{}", ebhq.visualize());
                //println!("{}", steps);
                break
            }
            None => ()
        }
        if steps > 10000 {
            return Err("no christmas tree".to_string())
        }
        //let mut buf = String::new();
        //io::stdin().read_line(&mut buf);
    }
    Ok(steps)
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Robot {
    start_pos: Coord,
    position: Coord,
    velocity: Vector,
}

impl Robot {
    fn step(&mut self, steps: i64,  max_tile: Coord) -> Result<(), String> {
        self.position.x = ( self.position.x + self.velocity.x * steps ) % max_tile.x;
        self.position.x = if self.position.x < 0 {
            max_tile.x + self.position.x
        } else {
            self.position.x
        };

        self.position.y = ( self.position.y + self.velocity.y * steps ) % max_tile.y;
        self.position.y = if self.position.y < 0 {
            max_tile.y + self.position.y
        } else {
            self.position.y
        };

        Ok(())
    }
}

impl FromStr for Robot {
    fn from_str(s: &str) -> Result<Robot, ParseError> {
        let mut tokens = s
            .split(' ')
            .map(|s| s
                .trim()
                .chars()
                .skip(2)
                .collect::<String>()
            );

        let p = tokens.next().unwrap().parse::<Coord>().unwrap();
        let v = tokens.next().unwrap().parse::<Vector>().unwrap();
        let r = Robot { start_pos: p, position: p, velocity: v};
        Ok(r)
    }

    type Err = ParseError;
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Ebhq {
    robots: Vec<Robot>,
    max: Coord,
}

impl Ebhq {
    fn new(input: &str, max: Coord) -> Self {
        let robots = Ebhq::parse(input).unwrap();
        Ebhq { robots, max }
    }

    fn parse(input: &str) -> Result<Vec<Robot>, ParseError> {
        let robots = input
            .lines()
            .map(|line| {
                line.parse::<Robot>().unwrap()
            })
            .collect();
        Ok(robots)
    }

    fn visualize(&self) -> String {
        let mut grid: Vec<Vec<i64>> = vec![vec![0; self.max.x as usize]; self.max.y as usize];
        self.robots.iter().for_each(|r| {
            grid[r.position.y as usize][r.position.x as usize] += 1;
        });
        let s = String::new();

        grid
            .iter()
            .map(|line| {
                line
                    .iter()
                    .map(|tile| {
                        match tile {
                            0 => ".".to_string(),
                            _ => tile.to_string(),
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn step(&mut self, steps: i64) -> Result<(), String> {
        self.robots
            .iter_mut()
            .for_each(|r| {
                r.step(steps, self.max);
                //println!("stepped robot: {:?}", r);
            });
        Ok(())
    }

    fn quadrants(&self) -> Vec<i64> {
        let split = Coord::new(self.max.x/2, self.max.y/2);
        //println!("split={:?}", split);
        let bounds = [
            (Coord::new(0, 0), Coord::new(split.x - 1, split.y - 1)),  // nw
            (Coord::new(split.x + 1, 0), Coord::new(self.max.x - 1, split.y - 1)), // ne
            (Coord::new(0, split.y + 1), Coord::new(split.x - 1, self.max.y - 1)),  // sw
            (Coord::new(split.x + 1, split.y + 1), Coord::new(self.max.x - 1, self.max.y - 1)), // se
        ];

        fn robot_in_bound(r: &Robot, northwest: Coord, southeast: Coord) -> bool {
            //northwest <= r.position && r.position <= southeast
            let nw = northwest <= r.position;
            let se = r.position <= southeast;
            //println!("robot_in_bound: r.position={:?}, nw={:?}, se={:?}", r.position, nw, se);
            nw && se
        }

        bounds.iter().map(|b|{
            self.robots
                .iter()
                .map(|r| {
                    let v = match robot_in_bound(r, b.0, b.1) {
                        true => 1,
                        false => 0,
                    };
                    //if v == 1 {
                    //    println!("found r={:?}", r);
                    //}
                    v
                })
                .sum()
        }).collect()
    }

    fn solve(&self) -> i64 {
        let quadrants: Vec<i64> = self.quadrants();
        //println!("{:?}", quadrants);
        quadrants.into_iter().reduce(|acc, element| acc * element).unwrap()
    }

    fn scan(&self) -> Option<Coord> {
        let mut grid: Vec<Vec<i64>> = vec![vec![0; self.max.x as usize]; self.max.y as usize];
        self.robots.iter().for_each(|r| {
            grid[r.position.y as usize][r.position.x as usize] += 1;
        });
        let pattern = [[0,1,0],[1,1,1],[1,1,1],[1,1,1]];

        for x in 0..self.max.x - pattern[0].len() as i64 {
            for y in 0..self.max.y - pattern.len() as i64 {
                let mut next_grid_loc = false;
                for px in 0..pattern[0].len() {
                    for py in 0..pattern.len() {
                        if grid[py + y as usize][px + x as usize] != pattern[py][px] {
                            next_grid_loc = true;
                            break
                        }
                    }
                    if next_grid_loc {
                        break
                    }
                }

                if !next_grid_loc {
                    return Some(Coord::new(x, y))
                }
            }
        }

        None
    }
}

impl Display for Ebhq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //for r in &self.robots  {
        //    write!(f, "{}", r);
        //}
        write!(f, "Ebhq: robots={:?}, max={:?}", self.robots, self.max)
    }
}

#[cfg(test)]
mod tests {
    use util::Vector;

    use super::*;

    #[test]
    fn test_robot_parse() -> Result<(), String> {
        let input = "p=0,4 v=3,-3";
        let actual = input.parse::<Robot>().unwrap();
        let expected = Robot {
            start_pos: Coord::new(0, 4),
            position: Coord::new(0, 4),
            velocity: Vector::new(3, -3),
        };
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_robot_step() -> Result<(), String> {
        let input = "p=0,4 v=3,-3";
        let mut r = input.parse::<Robot>().unwrap();
        let max_tile = Coord::new(7, 11);

        r.step(1, max_tile)?;

        let expected = Robot {
            start_pos: Coord::new(0, 4),
            position: Coord::new(3, 1),
            velocity: Vector::new(3, -3),
        };
        assert_eq!(r, expected);

        r.step(1, max_tile)?;

        let expected = Robot {
            start_pos: Coord::new(0, 4),
            position: Coord::new(6, 9),
            velocity: Vector::new(3, -3),
        };
        assert_eq!(r, expected);
        Ok(())
    }

    #[test]
    fn test_ebhq_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = Ebhq::new(input, Coord::new(0, 0));
        //let expected = Ebhq { ..Default::default() };
        //assert_eq!(actual, expected);
        assert!(actual.robots.len() == 12);

        let r12: Robot = r"p=9,5 v=-3,-3".parse().unwrap();
        assert_eq!(actual.robots[11], r12);
        Ok(())
    }

    #[test]
    fn test_ebhq_visualize() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let ebhq = Ebhq::new(input, Coord::new(11, 7));

        let actual_tiles = ebhq.visualize();
        let expected_tiles = r"1.12.......
...........
...........
......11.11
1.1........
.........1.
.......1...".to_string();
        assert_eq!(actual_tiles, expected_tiles);
        Ok(())
    }

    #[test]
    fn test_ebhq_step() -> Result<(), String> {
        let input = r"p=2,4 v=2,-3";
        let mut ebhq = Ebhq::new(input, Coord::new(11, 7));

        let actual_tiles = ebhq.visualize();
        let expected_tiles = r"...........
...........
...........
...........
..1........
...........
...........".to_string();
        assert_eq!(actual_tiles, expected_tiles);

        println!("initial {}", ebhq);
        ebhq.step(1)?;
        println!("step 1: {}", ebhq);
        let actual_tiles = ebhq.visualize();
        let expected_tiles = r"...........
....1......
...........
...........
...........
...........
...........".to_string();
        assert_eq!(actual_tiles, expected_tiles);

        ebhq.step(1)?;
        println!("step 2: {}", ebhq);
        let actual_tiles = ebhq.visualize();
        let expected_tiles = r"...........
...........
...........
...........
...........
......1....
...........".to_string();
        assert_eq!(actual_tiles, expected_tiles);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let mut ebhq = Ebhq::new(input, Coord::new(11, 7));

        ebhq.step(100)?;
        let actual_tiles = ebhq.visualize();
        let expected_tiles = r"......2..1.
...........
1..........
.11........
.....1.....
...12......
.1....1....".to_string();
        assert_eq!(actual_tiles, expected_tiles);

        //let quadrants: Vec<i64> = ebhq.quadrants();
        //let expected = vec![1, 3, 4, 1];
        //assert_eq!(quadrants, expected);
        //
        //let actual = quadrants.into_iter().reduce(|acc, element| acc * element).unwrap();
        let actual = ebhq.solve();
        let solution = 12;
        assert_eq!(actual, solution);
        Ok(())
    }

    #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let max = Coord::new(101, 103);
        let mut ebhq = Ebhq::new(input, max);
        ebhq.step(100)?;
        let actual = ebhq.solve();
        let solution = 211773366;
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

     #[test]
    fn test_solve_part2() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input)?;
        let solution = 7344;
        assert_eq!(actual, solution);
        Ok(())
    }
}
