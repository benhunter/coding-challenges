use util::{parse_grid_chars, ParseError};

pub fn solve_part1(input: &str) -> Result<i64, String> {
    let secrets = parse(input)?;
    let mut solutions = vec![];
    for secret in secrets {
        println!("{}", secret);
        let mut next = secret;
        for _ in 0..2000 {
            next = solve_next(next);
        }
        println!("{}", next);
        solutions.push(next);
    }
    Ok(solutions.iter().sum())
}

pub fn solve_part2(input: &str) -> Result<i64, String> {
    todo!()
}

fn parse(input: &str) -> Result<Vec<i64>, ParseError> {
    let secrets = input.lines().map(|l| l.parse::<i64>().expect("gib")).collect();
    Ok(secrets)
}

fn solve_next(value: i64) -> i64 {
    let result = ((value * 64) ^ value) % 16777216;
    let result = ((result / 32) ^ result) % 16777216;
    let result = ((result * 2048) ^ result) % 16777216;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = parse(input)?.len();
        let expected = 4;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_solve() -> Result<(), String> {
        assert!(42 ^ 15 == 37);

        let actual = solve_next(123);
        let expected = 15887950;
        assert_eq!(actual, expected);

        let actual = solve_next(expected);
        let expected = 16495136;
        assert_eq!(actual, expected);

        let actual = solve_next(expected);
        let expected = 527345;
        assert_eq!(actual, expected);

        let actual = solve_next(expected);
        let expected = 704524;
        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<(), String> {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input)?;
        let solution = 37327623;
        assert_eq!(actual, solution);
        Ok(())
    }

     #[test]
    fn test_solve_part1() -> Result<(), String> {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input)?;
        let solution = 13234715490;
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
