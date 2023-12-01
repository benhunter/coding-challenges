use std::io::prelude::*;

fn main() {
    let input = include_str!("../part1.txt");
    let result = solve_part1(input);
    println!("part1: {}", result);

    let result = solve_part2(input);
    println!("part2: {}", result);
}

fn solve_part1(input: &str) -> i32 {
    input.lines()
        .map( |line| {
            let first_digit = line.chars()
                .find( |c| c.is_digit(10) ).unwrap().to_digit(10).unwrap() as i32 * 10;
            let last_digit = line.chars()
                .rev()
                .find( |c| c.is_digit(10) ).unwrap().to_digit(10).unwrap() as i32;
            first_digit + last_digit
        })
        .sum::<i32>()
}

fn solve_part2(input: &str) -> i32 {
    input.lines()
        .map( |line| {
            let first_digit = first_digit(line) * 10;
            let last_digit = last_digit(line);
            first_digit + last_digit
        })
        .sum::<i32>()
}

fn first_digit(line: &str) -> i32 {
    for i in 0..line.len() {
        let s = &line[i..];
        if s.chars().next().unwrap().is_digit(10) {
            return s.chars().next().unwrap().to_digit(10).unwrap() as i32;
        }
        let nums = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for j in 1..10 {
            let s2 = format!("{}", j);
            if s.starts_with(nums[j]) {
                return j as i32;
            }
        }
    }
    panic!("no digit found");
}

fn last_digit(line: &str) -> i32 {
    for i in (0..line.len()).rev() {
        let slice = &line[i..];
        if slice.chars().next().unwrap().is_digit(10) {
            return slice.chars().next().unwrap().to_digit(10).unwrap() as i32;
        }
        let nums = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for j in 1..10 {
            let s2 = format!("{}", j);
            if slice.starts_with(nums[j]) {
                return j as i32;
            }
        }
    }
    panic!("no digit found");
}

fn string_to_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input = include_str!("../test1.txt");
        assert_eq!(solve_part1(input), 142);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test2.txt");
        assert_eq!(solve_part2(input), 281);
    }

    #[test]
    fn test_first_digit() {
        assert_eq!(first_digit("one"), 1);
        assert_eq!(first_digit("one2"), 1);
        assert_eq!(first_digit("one2three"), 1);
    }

    #[test]
    fn test_last_digit() {
        assert_eq!(last_digit("one"), 1);
        assert_eq!(last_digit("2one"), 1);
        assert_eq!(last_digit("three2one"), 1);
        assert_eq!(last_digit("three2on"), 2);
    }
}
