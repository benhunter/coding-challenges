use util::{Coord, Direction, Distance};
use util::Distance::{Infinity, Value};

pub fn solve_part1(input: (&str, usize)) -> usize {
    let steps = input.1;
    let input = input.0;
    let garden = parse(input);

    let mut touching = vec![vec![false; garden.bound.x]; garden.bound.y];
    let mut next_touching = vec![vec![false; garden.bound.x]; garden.bound.y];
    touching[garden.start.y][garden.start.x] = true;


    for i in 0..steps {
        // println!("line 15, step: {}, touching", i);
        // visualize_touching(&garden, &touching);
        let mut next_touching = touching.clone();

        for y in 0..garden.bound.y {
            for x in 0..garden.bound.x {
                if !touching[y][x] { continue; }
                let current = Coord::new(x, y);
                next_touching[y][x] = false;

                // println!("line 25, step: {}, next_touching", i);
                // visualize_touching(&garden, &next_touching);

                for direction in Direction::iter() {
                    let next = match current.go(&direction, &garden.bound) {
                        None => { continue; }
                        Some(next) => {
                            match garden.plots[next.y][next.x] {
                                Plot::Rock => continue,
                                // Plot::Start => panic!("🧐"),
                                _ => {
                                    next_touching[next.y][next.x] = true;

                                    // println!("line 38, step: {}, next_touching", i);
                                    // visualize_touching(&garden, &next_touching);
                                }
                            }
                        }
                    };
                }
            }
        }

        // println!("line 47, step: {}, next_touching", i);
        // visualize_touching(&garden, &next_touching);
        touching = next_touching.clone();
        let count = count_touching(&touching);
        println!("step: {}, touching: {}", i, count);
        // visualize_touching(&garden, &touching);
    }

    // visualize_touching(&garden, &touching);

    count_touching(&touching)
}

pub fn solve_part2(input: &str) -> i32 {
    0
}

fn visualize_touching(garden: &Garden, distances: &Vec<Vec<bool>>) {
    for y in 0..garden.bound.y {
        for x in 0..garden.bound.x {
            let c = match garden.plots[y][x] {
                Plot::Start => "S",
                Plot::Garden => {
                    match distances[y][x] {
                        false => ".",
                        true => "O",
                    }
                }
                Plot::Rock => "#",
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn count_touching(touching: &Vec<Vec<bool>>) -> usize {
    touching.iter().map(|y| {
        y.iter().filter(|c| **c).count()
    }).sum()
}

#[derive(Debug)]
struct Garden {
    plots: Vec<Vec<Plot>>,
    start: Coord,
    bound: Coord,
}

#[derive(Debug, PartialEq)]
enum Plot {
    Start,
    Garden,
    Rock,
}

fn parse(input: &str) -> Garden {
    let mut start = Coord::new(0, 0);
    let plots: Vec<Vec<Plot>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| match c {
            '.' => Plot::Garden,
            '#' => Plot::Rock,
            'S' => {
                start = Coord::new(x, y);
                Plot::Start
            }
            _ => panic!("🧐"),
        }).collect()
    }).collect();
    let bound = Coord::new(plots.len(), plots[0].len());
    Garden { plots, start, bound }
}

#[cfg(test)]
mod tests {
    use util::Coord;
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../test.txt");
        let garden = parse(input);
        let expected_start = Coord::new(5, 5);
        assert_eq!(garden.start, expected_start);
        assert_eq!(garden.plots[expected_start.y][expected_start.x], Plot::Start);
        assert_eq!(garden.plots[5][6], Plot::Rock);
        assert_eq!(garden.plots[5][4], Plot::Garden);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let steps = 6;
        let actual = solve_part1((input, steps));
        let solution = 16;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let actual = solve_part1((input, 64));
        let solution = 3820;
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
