use util::Coord;

pub fn solve_part1(input: &str, steps: usize) -> i32 {
    let garden = parse(input);

    let current = &garden.start;
    let nexts = current.neighbors(&garden.bound);
    todo!()
}

pub fn solve_part2(input: &str) -> i32 {
    0
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
        let actual = solve_part1(input, steps);
        let solution = 16;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input, 64);
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
