fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    let result = solve_part2(input);
    println!("✅ part2: {}", result);
}

#[derive(Debug, PartialEq, Clone)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug, Clone)]
struct Game {
    id: i32,
    record: Cubes,
}

fn parse_record(record: &str) -> Cubes {
    let mut record = record.split(", ");
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    while let Some(part) = record.next() {
        let mut part = part.split(" ");
        let num = part.next().unwrap().parse::<i32>().unwrap();
        let color = part.next().unwrap();
        match color {
            "red" => red += num,
            "green" => green += num,
            "blue" => blue += num,
            _ => panic!("😅"),
        }
    }
    Cubes {red, green, blue}
}

fn combine_records(input: &str) -> Cubes {
    input.split("; ")
        .map(|record| parse_record(record))
        .fold(Cubes {red: 0, green: 0, blue: 0}, |acc, record| {
            Cubes {
                red: acc.red.max(record.red),
                green: acc.green.max(record.green),
                blue: acc.blue.max(record.blue),
            }
        })
}

fn solve_part1(input: &str) -> i32 {
    let bag = Cubes {red: 12, green: 13, blue: 14 };
    input.lines()
        .map(|games| {
            let mut games = games.split(": ");
            let id = games.next().unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            let record = combine_records(games.next().unwrap());
            Game {id, record}
        })
        .filter(|game| {
            game.record.red <= bag.red && game.record.green <= bag.green && game.record.blue <= bag.blue
        })
        .map(|game| game.id)
        .sum()
}

fn solve_part2(input: &str) -> i32 {
    input.lines()
        .map(|games| {
            let games = games
                .split(": ")
                .nth(1).unwrap();
            power(combine_records(games))
        })
        .sum()
}

fn power(input: Cubes) -> i32 {
    input.blue * input.green * input.red
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test1.txt");
        assert_eq!(solve_part1(input), 8);
    }

    #[test]
    fn test_parse_record() {
        let input= "3 blue, 4 red";
        assert_eq!(parse_record(input), Cubes {red: 4, green: 0, blue: 3});

    }

    #[test]
    fn test_combine_records() {
        let input = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Cubes {red: 4, green: 2, blue: 6};
        assert_eq!(combine_records(input), expected);
    }

    #[test]
    fn test_power() {
        let input = Cubes {red: 4, green: 2, blue: 6};
        let expected = 48;
        assert_eq!(power(input), expected);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test1.txt");
        assert_eq!(solve_part2(input), 2286);
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(solve_part1(input), 2593);
    }

    #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(solve_part2(input), 54699);
    }
}
