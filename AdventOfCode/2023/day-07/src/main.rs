fn main() {
    let input = include_str!("../input.txt");
    let result = solve_part1(input);
    println!("✅ part1: {}", result);

    let result = solve_part2(input);
    println!("✅ part2: {}", result);
}

fn solve_part1(input: &str) -> u32 {
    let mut game = parse(input);
    game.set_hand_types();
    let sorted = game.sort_hands();
    // dbg!(sorted.clone());
    sorted.clone().iter().for_each(|hand| {
        println!("{:?}", hand.clone());
    });
    sorted.iter().rev().enumerate().map(|(i, hand)| {
        hand.bid * (i as u32 + 1)
    }).sum::<u32>()
}

fn solve_part2(input: &str) -> i32 {
    0
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    hands: Vec<Hand>,
}

impl Game {
    pub(crate) fn set_hand_types(&mut self) {
        self.hands.iter_mut().for_each(|hand| {
            hand.set_hand_type();
        });
    }

    pub(crate) fn sort_hands(&self) -> Vec<Hand> {
        let mut hands = self.hands.clone();
        hands.sort_by(|a, b| a.cmp(b));
        hands
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Hand {
    cards: String,
    bid: u32,
    hand_type: Option<HandType>,
}

impl Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        // dbg!(self.clone());
        // dbg!(other.clone());
        let self_type = self.hand_type.as_ref().unwrap();
        // dbg!(self_type.clone());
        let other_type = other.hand_type.as_ref().unwrap();
        // dbg!(other_type.clone());
        return if self_type == other_type {
            // println!("checking high card");
            let result = cmp_high_card(&self.cards, &other.cards);
            // dbg!(self.clone());
            // println!("result: self is {:?}", result);
            // dbg!(result.clone());
            return result;
        } else {
            self_type.cmp(&other_type)
        };
    }


    fn set_hand_type(&mut self) {
        self.hand_type = Option::from(if self.is_five_of_a_kind() {
            HandType::FiveOfAKind
        } else if self.is_four_of_a_kind() {
            HandType::FourOfAKind
        } else if self.is_full_house() {
            HandType::FullHouse
        } else if self.is_three_of_a_kind().is_some() {
            HandType::ThreeOfAKind
        } else if self.is_two_pairs() {
            HandType::TwoPairs
        } else if self.is_one_pair() {
            HandType::OnePair
        } else {
            HandType::HighCard
        });
    }

    fn is_five_of_a_kind(&self) -> bool {
        let first = self.cards.chars().nth(0).unwrap();
        self.cards.chars().all(|c| c == first)
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.cards.chars().find(|c| self.cards.matches(*c).count() == 4).is_some()
    }

    fn is_full_house(&self) -> bool {
        let three_of_a_kind = self.is_three_of_a_kind();
        dbg!(three_of_a_kind.clone());
        if three_of_a_kind.is_none() {
            return false;
        }
        let cards = self.cards.replace(three_of_a_kind.unwrap().to_string().as_str(), "");
        dbg!(cards.clone());
        let mut cards = cards.chars();
        let one = cards.next().unwrap();
        let two = cards.next().unwrap();
        dbg!(one.clone());
        dbg!(two.clone());
        if one == two {
        // if cards.next().unwrap() == cards.next().unwrap() {
            return true;
        }
        false
    }

    fn is_three_of_a_kind(&self) -> Option<char> {
        self.cards.chars().find(|c| self.cards.matches(*c).count() == 3)
    }

    fn is_two_pairs(&self) -> bool {
        let first = self.cards.chars().find(|c| self.cards.matches(*c).count() == 2);
        if first.is_none() {
            return false;
        }
        let cards = self.cards.replace(first.unwrap().to_string().as_str(), "");
        let second = cards.chars().find(|c| self.cards.matches(*c).count() == 2);
        if second.is_none() {
            return false;
        }
        true
    }

    fn is_one_pair(&self) -> bool {
        self.cards.chars().find(|c| self.cards.matches(*c).count() == 2).is_some()
    }
}

fn cmp_high_card(me: &String, other: &String) -> std::cmp::Ordering {
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    // let me = me.chars().map(score_card).collect::<Vec<u32>>();
    // me.iter().cmp(other.chars().map(score_card))

    for i in 0..me.len() {
        let me_score = score_card(me.chars().nth(i).unwrap());
        // dbg!(me_score.clone());
        let other_score = score_card(other.chars().nth(i).unwrap());
        // dbg!(other_score.clone());
        match other_score.cmp(&me_score) { // TODO scary
            std::cmp::Ordering::Equal => continue,
            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
        }
    }
    std::cmp::Ordering::Equal
}

fn score_card(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

fn parse(input: &str) -> Game {
    // dbg!(input);
    Game {
        hands: input.lines().map(|hand| {
            let mut hand = hand.split(" "); //.collect::<Vec<&str>>().into_iter();
            let cards = hand.next();
            let bid = hand.next();
            Hand {
                cards: cards.unwrap().to_string(),
                bid: bid.unwrap().parse::<u32>().unwrap(),
                hand_type: None,
            }
        }).collect::<Vec<Hand>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../test.txt");
        let actual = parse(input).hands.len();
        let expected = 5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../test.txt");
        let solution = 6440;
        assert_eq!(solve_part1(input), solution);
    }

    #[test]
    fn test_sort_hands() {
        let input = "32T3K 765
T55J5 684
44444 123";
        let mut game = parse(input);
        game.set_hand_types();
        let actual = game.sort_hands();
        dbg!(actual.clone());
        let expected = vec![
            Hand { cards: "44444".to_string(), bid: 123, hand_type: Option::from(HandType::FiveOfAKind) },
            Hand { cards: "T55J5".to_string(), bid: 684, hand_type: Option::from(HandType::ThreeOfAKind) },
            Hand { cards: "32T3K".to_string(), bid: 765, hand_type: Option::from(HandType::OnePair) },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sort_hands2() {
        let input = "T55J5 684
QQQJA 483";
        let mut game = parse(input);
        game.set_hand_types();
        let actual = game.sort_hands();
        // dbg!(actual.clone());
        let expected = vec![
            Hand { cards: "QQQJA".to_string(), bid: 483, hand_type: Option::from(HandType::ThreeOfAKind) },
            Hand { cards: "T55J5".to_string(), bid: 684, hand_type: Option::from(HandType::ThreeOfAKind) },
        ];
        assert_eq!(actual, expected);
    }

    // TODO
    #[test]
    fn test_full_house() {
        let input = "T55J5 684";
        let game = parse(input);
        let actual = game.hands[0].is_full_house();
        let expected = false;
        assert_eq!(actual, expected);
    }


    #[test]
    fn test_solve2() {
        let input = include_str!("../test.txt");
        let solution = 0;
        assert_eq!(solve_part2(input), solution);
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let solution = 248422077;
        assert_eq!(solve_part1(input), solution);
    }

    // #[test]
    // fn test_solve_part2() {
    //     let input = include_str!("../input.txt");
    //     let solution = 0;
    //     assert_eq!(solve_part2(input), solution);
    // }
}
