use std::str::FromStr;
use util::{parse_grid_chars, Coord, ParseError};

pub fn solve_part1(input: &str) -> Result<i64, String> {
    let topo_map: TopoMap = input.parse()?;
    println!("{:?}", topo_map);

    let mut candidate_trails: Vec<Vec<Coord>> = vec![topo_map.trailheads];
    println!("{:?}", candidate_trails);

    while candidate_trails.len() > 0 {
        let candidate = candidate_trails.pop();
        println!("{:?}", candidate);
    }
    todo!()
}

pub fn solve_part2(input: &str) -> Result<i64, String> {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Default)]
struct TopoMap {
    topo: Vec<Vec<char>>,
    trailheads: Vec<Coord>,
}

impl FromStr for TopoMap {
    fn from_str(s: &str) -> Result<TopoMap, ParseError> {
        let lines = parse_grid_chars(s)?;
        let mut trailheads: Vec<Coord> = vec![];

        lines.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, column)| {
                if *column == '0' {
                    trailheads.push(Coord::new(x.try_into().unwrap(), y.try_into().unwrap()));
                }
            });
        });
        Ok(TopoMap { topo: lines, trailheads })
    }

    type Err = ParseError;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual: TopoMap = input.parse()?;
        let expected_trailheads = vec![Coord::new(0, 0)];
        assert_eq!(actual.trailheads, expected_trailheads);
        Ok(())
    }

    #[test]
    fn test_parse2() -> Result<(), String> {
        let input = include_str!("../test2.txt");
        let actual: TopoMap = input.parse()?;
        let expected_trailheads_len = 9;
        assert_eq!(actual.trailheads.len(), expected_trailheads_len);
        Ok(())
    }

    // #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_part2() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part2(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }

    // #[test]
    fn test_solve_part2() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input)?;
        let solution = 0;
        assert_eq!(actual, solution);
        Ok(())
    }
}
