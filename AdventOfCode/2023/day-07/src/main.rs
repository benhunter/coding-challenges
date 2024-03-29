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
    sorted.iter().rev().enumerate().map(|(i, hand)| {
        hand.bid * (i as u32 + 1)
    }).sum::<u32>()
}

fn solve_part2(input: &str) -> u32 {
    let mut game = parse(input);
    game.set_hand_types_part2();
    let sorted = game.sort_hands_part2();

    // Filtered debug output
    // sorted.iter().rev().enumerate()
    // .filter(|(i, hand)| {
    //     hand.hand_type == Option::from(HandType::ThreeOfAKind)
    //         &&
    //         hand.cards.contains('J')
    // !hand.cards.contains('J')
    // true
    // hand.cards.matches('J').count() == 5
    // })
    // .for_each(|(i, hand)| {
    //     let winnings = hand.bid * (i as u32 + 1);
    //     println!("{:?}, position: {}, winnings: {}", hand.clone(), i + 1, winnings);
    // });

    sorted.iter().rev().enumerate().map(|(i, hand)| {
        let winnings = hand.bid * (i as u32 + 1);
        winnings
    }).sum::<u32>()
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

    pub(crate) fn set_hand_types_part2(&mut self) {
        self.hands.iter_mut().for_each(|hand| {
            hand.set_hand_type_part2();
        });
    }

    pub(crate) fn sort_hands(&self) -> Vec<Hand> {
        let mut hands = self.hands.clone();
        hands.sort_by(|a, b| a.cmp(b));
        hands
    }

    pub(crate) fn sort_hands_part2(&self) -> Vec<Hand> {
        let mut hands = self.hands.clone();
        hands.sort_by(|a, b| a.cmp_part2(b));
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
        let self_type = self.hand_type.as_ref().unwrap();
        let other_type = other.hand_type.as_ref().unwrap();
        return if self_type == other_type {
            let result = cmp_high_card(&self.cards, &other.cards, score_card);
            return result;
        } else {
            self_type.cmp(&other_type)
        };
    }

    fn cmp_part2(&self, other: &Hand) -> std::cmp::Ordering {
        let self_type = self.hand_type.as_ref().unwrap();
        let other_type = other.hand_type.as_ref().unwrap();
        return if self_type == other_type {
            let result = cmp_high_card(&self.cards, &other.cards, score_card_part2);
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
        } else if self.is_one_pair().is_some() {
            HandType::OnePair
        } else {
            HandType::HighCard
        });
    }

    fn set_hand_type_part2(&mut self) {
        self.hand_type = Option::from(if self.is_five_of_a_kind_part2() {
            HandType::FiveOfAKind
        } else if self.is_four_of_a_kind_part2() {
            HandType::FourOfAKind
        } else if self.is_full_house_part2() {
            HandType::FullHouse
        } else if self.is_three_of_a_kind_part2().is_some() {
            HandType::ThreeOfAKind
        } else if self.is_two_pairs() {
            HandType::TwoPairs
        } else if self.is_one_pair_part2() {
            HandType::OnePair
        } else {
            HandType::HighCard
        });
    }

    fn is_five_of_a_kind(&self) -> bool {
        let first = self.cards.chars().nth(0).unwrap();
        self.cards.chars().all(|c| c == first)
    }

    fn is_five_of_a_kind_part2(&self) -> bool {
        self.is_five_of_a_kind()
            || (self.is_four_of_a_kind() && self.cards.contains('J'))
            || (self.is_three_of_a_kind().is_some() && self.cards.matches('J').count() == 2)
            || (self.is_one_pair().is_some() && self.cards.matches('J').count() == 3)
            || (self.cards.matches('J').count() == 4)
            || (self.cards.matches('J').count() == 5)
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.cards.chars().find(|c| self.cards.matches(*c).count() == 4).is_some()
    }

    fn is_four_of_a_kind_part2(&self) -> bool {
        if self.cards.chars().find(|c| self.cards.matches(*c).count() == 4).is_some() {
            return true;
        }

        let joker_count = self.cards.matches('J').count();
        let three = self.is_three_of_a_kind();
        if three.is_some() {
            let cards = self.cards.replace(three.unwrap().to_string().as_str(), "");
            if cards.contains('J') {
                return true;
            }
        }

        let cards = self.cards.replace('J', "");

        cards.chars().find(|c| cards.matches(*c).count() == 2).is_some() && joker_count == 2
            || joker_count == 3
    }

    fn is_full_house(&self) -> bool {
        let three_of_a_kind = self.is_three_of_a_kind();
        if three_of_a_kind.is_none() {
            return false;
        }
        let cards = self.cards.replace(three_of_a_kind.unwrap().to_string().as_str(), "");
        let mut cards = cards.chars();

        cards.next().unwrap() == cards.next().unwrap()
    }

    fn is_full_house_part2(&self) -> bool {
        let joker_count = self.cards.matches('J').count();
        let cards = self.cards.replace('J', "");

        fn is_two_pairs(cards: String) -> bool {
            let first = cards.chars().find(|c| cards.matches(*c).count() == 2);
            if first.is_none() {
                return false;
            }
            let cards = cards.replace(first.unwrap().to_string().as_str(), "");
            let second = cards.chars().find(|c| cards.matches(*c).count() == 2);
            return !second.is_none();
        }

        is_two_pairs(cards.clone()) && joker_count == 1
            || self.is_full_house()
    }

    fn is_three_of_a_kind(&self) -> Option<char> {
        self.cards.chars().find(|c| self.cards.matches(*c).count() == 3)
    }

    fn is_three_of_a_kind_part2(&self) -> Option<char> {
        if self.cards.contains('J') {
            let c = self.cards.chars().find(|c| self.cards.matches(*c).count() == 2);
            if c.is_some() {
                return c;
            }
        }
        self.is_three_of_a_kind()
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

    fn is_one_pair(&self) -> Option<char> {
        self.cards.chars().find(|c| self.cards.matches(*c).count() == 2)
    }

    fn is_one_pair_part2(&self) -> bool {
        self.cards.contains('J') || self.is_one_pair().is_some()
    }
}

fn cmp_high_card(me: &String, other: &String, score_card: fn(c: char) -> u32) -> std::cmp::Ordering {
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2

    for i in 0..me.len() {
        let me_score = score_card(me.chars().nth(i).unwrap());
        let other_score = score_card(other.chars().nth(i).unwrap());
        match other_score.cmp(&me_score) { // scary switcheroo
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

fn score_card_part2(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1, // now lowest individual card in case of tie breaker
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
    Game {
        hands: input.lines().map(|hand| {
            let mut hand = hand.split(" ");
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
    use crate::HandType::FullHouse;
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
        let expected = vec![
            Hand { cards: "QQQJA".to_string(), bid: 483, hand_type: Option::from(HandType::ThreeOfAKind) },
            Hand { cards: "T55J5".to_string(), bid: 684, hand_type: Option::from(HandType::ThreeOfAKind) },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_full_house() {
        let input = "T55J5 684";
        let game = parse(input);
        let actual = game.hands[0].is_full_house();
        let expected = false;
        assert_eq!(actual, expected);
    }


    #[test]
    fn test_part2() {
        let input = include_str!("../test.txt");
        let solution = 5905;
        assert_eq!(solve_part2(input), solution);
    }

    // part 2
    #[test]
    fn test_part2_two_pair_no_joker() {
        // no two pair should have a joker (it would be a three of a kind)
        let input = include_str!("../input.txt");
        let mut game = parse(input);
        game.set_hand_types_part2();
        let sorted = game.sort_hands_part2();
        sorted.iter().filter(|hand| hand.hand_type == Option::from(HandType::TwoPairs)).for_each(|hand| {
            assert!(!hand.cards.contains('J'));
        });
    }

    #[test]
    fn test_part2_highcard_no_joker() {
        // no high card should have a joker (it would be a pair)
        let input = include_str!("../input.txt");
        let mut game = parse(input);
        game.set_hand_types_part2();
        let sorted = game.sort_hands_part2();
        sorted.iter().filter(|hand| hand.hand_type == Option::from(HandType::HighCard)).for_each(|hand| {
            assert!(!hand.cards.contains('J'));
        });
    }

    #[test]
    fn test_part2_pairs() {
        // Hand { cards: "25A99", bid: 265, hand_type: Some(HighCard) }
        let pair = Hand { cards: "25A99".to_string(), bid: 265, hand_type: None };
        assert!(pair.is_one_pair_part2());
    }

    #[test]
    fn test_part2_not_four_of_a_kind() {
        // Hand { cards: "JJ243", bid: 652, hand_type: Some(FourOfAKind) }, position: 795, winnings: 518340
        let three_of_a_kind = Hand { cards: "JJ243".to_string(), bid: 652, hand_type: None };
        assert!(!three_of_a_kind.is_four_of_a_kind_part2());
        assert!(three_of_a_kind.is_three_of_a_kind_part2().is_some());
    }

    #[test]
    fn test_part2_not_full_house() {
        let four_of_a_kind = Hand { cards: "J55AJ".to_string(), bid: 518, hand_type: Some(HandType::FullHouse) }; //, position: 732, winnings: 379176
        assert!(four_of_a_kind.is_four_of_a_kind_part2());

        // Hand { cards: "J667J", bid: 15, hand_type: Some(FullHouse) }, position: 733, winnings: 10995
        // Hand { cards: "J7J79", bid: 279, hand_type: Some(FullHouse) }, position: 734, winnings: 204786
    }

    #[test]
    fn test_part2_should_be_full_house() {
        let full_house = Hand { cards: "KKAJA".to_string(), bid: 444, hand_type: Some(FullHouse) }; //, position: 702, winnings: 311688
        assert!(full_house.is_full_house_part2());
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../input.txt");
        let solution = 248422077;
        assert_eq!(solve_part1(input), solution);
    }

    #[test]
    fn test_solve_part2() {
        let input = include_str!("../input.txt");
        let actual = solve_part2(input);
        let solution = 249817836;
        assert_eq!(actual, solution);
    }
}