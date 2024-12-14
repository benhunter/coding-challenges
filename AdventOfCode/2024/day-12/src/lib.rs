use util::{parse_grid_chars, ParseError};

pub fn solve_part1(input: &str) -> Result<i32, String> {
    //let garden = parse(input)?;
    todo!()
}

pub fn solve_part2(input: &str) -> Result<i32, String> {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Default)]
struct Garden {
    plots: Vec<Vec<char>>,
    regions: Vec<Vec<Option<char>>>,
}

impl Garden {
    // constructor:
    fn new(input: &str) -> Garden {
        Garden {
            plots: parse_grid_chars(input).unwrap(),
            regions: vec![vec![None; self.plots[0].len()]; self.plots.len()];
        }

    }

    fn parse(input: &str) -> Result<Garden, ParseError> {
        let plots = parse_grid_chars(input)?;
        Ok(Garden { plots })
    }

    fn regions(mut self) -> Vec<String> {
        let mut x = 0;
        let mut y = 0;
        let current_region_id = self.plots[0][0];
        self.regions[y][x] = Some(current_region_id);
        while self.regions.iter().flatten().any(|r| r.is_none() ) {
            self.regions[y][x] = Some(' ');
            x = (x + 1) % self.plots[0].len();
            if x == 0 {
                y += 1
            }

        }

        //self.visualize_regions();
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = Garden::parse(input)?;
        Ok(())
    }

    #[test]
    fn test_garden_regions() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let garden = Garden::parse(input)?;
        let actual = garden.regions();
        let expected = vec!["A", "B", "C", "D", "E"];
        assert_eq!(actual, expected);
        Ok(())
    }

    //#[test]
    //fn test_part1() -> Result<(), String> {
    //    let input = include_str!("../test.txt");
    //    let actual = solve_part1(input)?;
    //    let solution = 0;
    //    assert_eq!(actual, solution);
    //    Ok(())
    //}

    // #[test]
    //fn test_solve_part1() -> Result<(), String> {
    //    let input = include_str!("../input.txt");
    //    let actual = solve_part1(input)?;
    //    let solution = 0;
    //    assert_eq!(actual, solution);
    //    Ok(())
    //}

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
