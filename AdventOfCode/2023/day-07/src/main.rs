fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    // let result = solve_part2(input);
    // println!("✅ part2: {}", result);
}

#[derive(Debug, PartialEq, Clone)]
struct Item {
    attribute: i32,
}

fn parse(input: &str) -> Item {
    Item {attribute: 0}
}

fn solve_part1(input: &str) -> i32 {
    0
}

fn solve_part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input= "simple input";
        let output = Item {attribute: 0};
        assert_eq!(parse(input), output);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let solution = 0;
        assert_eq!(solve_part1(input), solution);
    }

    // #[test]
    fn test_solve2() {
        let input = include_str!("../test.txt");
        let solution = 0;
        assert_eq!(solve_part2(input), solution);
    }

    // #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let solution = 0;
        assert_eq!(solve_part1(input), solution);
    }

    // #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let solution = 0;
        assert_eq!(solve_part2(input), solution);
    }
}