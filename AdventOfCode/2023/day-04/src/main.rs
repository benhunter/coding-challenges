fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    let result = solve_part2(input);
    println!("✅ part2: {}", result);
}

fn parse(input: &str) -> Vec<Card> {
    input.split("\n").map(|line| {
        Card::parse(line)
    }).collect()
}

fn solve_part1(input: &str) -> i32 {
    parse(input)
        .iter()
        .inspect(|card| println!("{:?}", card))
        .map(|card| card.score() as i32)
        .inspect(|score| println!("{:?}", score))
        .sum()
}

#[derive(Debug, PartialEq, Clone)]
struct Card<'a> {
    winning: Vec<&'a str>,
    yours: Vec<&'a str>,
}

impl<'a> Card<'a> {
    fn parse(input: &str) -> Card {
        let numbers = input
            .split(": ").nth(1).unwrap().split(" | ").collect::<Vec<&str>>();
        let winning = numbers[0]
            .split(" ")
            .filter(|number| !number.is_empty())
            .collect::<Vec<&str>>();
        let yours = numbers[1]
            .split(" ")
            .filter(|number| !number.is_empty())
            .collect::<Vec<&str>>();
        Card { winning, yours }
    }

    pub(crate) fn is_winning(&self, number: &str) -> bool {
        self.winning.contains(&number)
    }

    pub(crate) fn count_winning(&self) -> usize {
        self.yours.iter().filter(|number| self.is_winning(number)).count()
    }

    pub(crate) fn score(&self) -> usize {
        if self.count_winning() == 0 {
            return 0;
        } else {
            2_usize.pow(self.count_winning() as u32 - 1)
        }
    }
}

fn solve_part2(input: &str) -> i32 {
    let cardss = parse(input);
    let mut cards = cardss.iter().map(|card| (card, 1)).collect::<Vec<(&Card, usize)>>();

    for index in 0..cards.len() {
        for quantity in 0..cards[index].1 {
            // println!("{} {}", index, quantity+1)
            let count = cards[index].0.count_winning();
            for extra in (index + 1)..(index + count + 1) {
                // println!("index: {} extra: {}", index, extra);
                cards[extra].1 += 1;
                // println!("{:?}", cards[extra]);
            }
        }
    }

    println!("{:?}", cards.iter().map(|card| card.1).collect::<Vec<usize>>());
    cards
        .iter()
        .map(|card| card.1 as i32)
        // .collect::<Vec<usize>>()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let output = Card { winning: vec!["41", "48", "83", "86", "17"], yours: vec!["83", "86", "6", "31", "17", "9", "48", "53"] };
        assert_eq!(Card::parse(input), output);
    }

    #[test]
    fn test_parse_card_score_0() {
        // let expected_card = Card { winning: vec!["31", "18", "13", "56", "72"], yours: vec!["74", "77", "10", "23", "35", "67", "36", "11"] };
        let input = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let card = Card::parse(input);
        let score = card.score();

        let expected = 0;
        assert_eq!(expected, score);
    }


    #[test]
    fn test_parse() {
        let input = include_str!("../test1.txt");
        let actual = parse(input).len();
        let expected = 6;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_winning() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual = Card::parse(input).is_winning("41");
        let expected = true;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_winning() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual = Card::parse(input).count_winning();
        let expected = 4;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_card_score() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let actual = Card::parse(input).score();
        let expected = 8;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test1.txt");
        let solution = 13;
        assert_eq!(solve_part1(input), solution);
    }

    #[test]
    fn test_solve2() {
        let input = include_str!("../test1.txt");
        let solution = 30;
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