use util::{Coord, Direction};

pub fn solve_part1(input: (&str, usize)) -> usize {
    let steps = input.1;
    let input = input.0;
    let garden = parse(input);

    let mut touching = vec![vec![false; garden.bound.x]; garden.bound.y];
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
                    match current.go(&direction, &garden.bound) {
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

    visualize_touching(&garden, &touching);

    count_touching(&touching)
}

pub fn solve_part2(input: (&str, usize)) -> usize {
    let steps = input.1;
    println!("steps: {}", steps);
    let input = input.0;
    let garden = parse(input);
    let steps_rem = steps.rem_euclid(garden.bound.x);
    let part1 = solve_part1((input, steps_rem));

    let infini_bound = steps * garden.bound.x;
    dbg!(infini_bound);

    let zero_offset = Coord { x: garden.bound.x, y: garden.bound.y };
    let rows: Vec<Vec<Plot>> = garden.plots.iter().map(|line| line.repeat(3)).collect::<Vec<Vec<Plot>>>();
    let mut plots_3x3 = rows.clone();
    plots_3x3.extend(rows.clone());
    plots_3x3.extend(rows.clone());
    // dbg!(plots_3x3.clone());

    let start_3x3 = zero_offset.add(garden.start);
    dbg!(start_3x3.clone());
    let bound_3x3 = Coord { x: plots_3x3[0].len(), y: plots_3x3.len() };
    dbg!(bound_3x3.clone());
    let garden_3x3 = Garden { plots: plots_3x3, start: start_3x3, bound: bound_3x3 };

    let mut touching = vec![vec![false; garden_3x3.bound.x]; garden_3x3.bound.y];
    touching[garden_3x3.start.y][garden_3x3.start.x] = true;

    for i in 0..garden_3x3.bound.x + 3 {
        let mut next_touching = touching.clone();

        for y in 0..garden_3x3.bound.y {
            for x in 0..garden_3x3.bound.x {
                if !touching[y][x] { continue; }
                let current = Coord::new(x, y);
                next_touching[y][x] = false;

                for direction in Direction::iter() {
                    match current.go(&direction, &garden_3x3.bound) {
                        None => { continue; }
                        Some(next) => {
                            match garden_3x3.plots[next.y][next.x] {
                                Plot::Rock => continue,
                                _ => {
                                    next_touching[next.y][next.x] = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        // println!("line 47, step: {}, next_touching", i);
        // visualize_touching(&garden, &next_touching);
        touching = next_touching.clone();
        let count = count_touching(&touching);
        println!("step: {}, touching: {}", i, count);
        // visualize_touching(&garden_3x3, &touching);

        // break;
    }

    // visualize_touching(&garden, &touching);

    let count = count_touching(&touching);
    println!("{}", count);

    // plot_3x3:
    // step: 0, touching: 2
    // step: 1, touching: 4
    // step: 2, touching: 6
    // step: 3, touching: 9
    // step: 4, touching: 13
    // step: 5, touching: 16
    // step: 6, touching: 22
    // step: 7, touching: 30
    // step: 8, touching: 41
    // step: 9, touching: 50
    // step: 10, touching: 63
    // step: 11, touching: 74
    // step: 12, touching: 89
    // step: 13, touching: 99
    // step: 14, touching: 115
    // step: 15, touching: 129
    // step: 16, touching: 145
    // step: 17, touching: 165
    // step: 18, touching: 192
    // step: 19, touching: 216
    // step: 20, touching: 234
    // step: 21, touching: 254
    // step: 22, touching: 272
    // step: 23, touching: 288
    // step: 24, touching: 303
    // step: 25, touching: 319
    // step: 26, touching: 329
    // step: 27, touching: 339
    // step: 28, touching: 348
    // step: 29, touching: 354
    // step: 30, touching: 359
    // step: 31, touching: 361
    // step: 32, touching: 361
    // step: 33, touching: 365
    // step: 34, touching: 363
    // step: 35, touching: 366

    // test eventually alternates:
    // 39
    // 42

    let stable1 = 39;
    let multiplier = steps / garden.bound.x;
    let output = stable1 * multiplier + part1;

    println!("stable1: {}, steps: {}, garden.bound.x: {}, multiplier: {}, steps_rem: {}, part1: {}, part2: {}", stable1, steps, garden.bound.x, multiplier, steps_rem, part1, output);

    // input eventually alternates:
    // 7719
    // 7734

    output
}

fn visualize_touching(garden: &Garden, distances: &[Vec<bool>]) {
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

fn count_touching(touching: &[Vec<bool>]) -> usize {
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

#[derive(Debug, PartialEq, Clone, Copy)]
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


    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        let steps = 6;
        let actual = solve_part2((input, steps));
        let solution = 16;
        assert_eq!(actual, solution);
    }

    #[test]
    fn test_part2_1() {
        let input = include_str!("../test.txt");
        let steps = 10;
        let actual = solve_part2((input, steps));
        let solution = 50;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_part2_2() {
        let input = include_str!("../test.txt");
        let steps = 50;
        let actual = solve_part2((input, steps));
        let solution = 1594;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_part2_3() {
        let input = include_str!("../test.txt");
        let steps = 100;
        let actual = solve_part2((input, steps));
        let solution = 6536;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_part2_4() {
        let input = include_str!("../test.txt");
        let steps = 500;
        let actual = solve_part2((input, steps));
        let solution = 167004;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_part2_5() {
        let input = include_str!("../test.txt");
        let steps = 1000;
        let actual = solve_part2((input, steps));
        let solution = 668697;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_part2_6() {
        let input = include_str!("../test.txt");
        let steps = 5000;
        let actual = solve_part2((input, steps));
        let solution = 16733044;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let steps = 26501365;
        let actual = solve_part2((input, steps));
        let solution = 0;
        assert_eq!(actual, solution);
    }
}
