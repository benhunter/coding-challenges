#[derive(Debug, PartialEq, Clone)]
struct Item {
    attribute: i32,
}

pub fn solve_part1(input: &str) -> i32 {
    0
}

pub fn solve_part2(input: &str) -> i32 {
    0
}

fn parse(input: &str) -> Item {
    Item {attribute: 0}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../test.txt");
        let actual = parse(input);
        let expected = Item {attribute: 0};
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let actual = solve_part1(input);
        let solution = 0;
        assert_eq!(actual, solution);
    }

    // #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let actual = solve_part1(input);
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
