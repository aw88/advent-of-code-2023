use std::{collections::HashMap, fmt::Error, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);

    println!("Result: {}", result);
}

fn to_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Card {
    Value(usize),
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            c if c.is_numeric() => Self::Value(c.to_digit(10).unwrap() as usize),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    bid: usize,
    cards: Vec<Card>,
}

impl Hand {
    fn new(bid: usize, cards: Vec<Card>) -> Self {
        Self { bid, cards }
    }

    fn hand_type(&self) -> HandType {
        let card_counts = self.cards.iter().fold(HashMap::new(), |mut acc, next| {
            let count = acc.entry(next).or_insert(0);
            *count += 1;
            acc
        });

        match card_counts.keys().len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if *card_counts.values().max().unwrap() == 3 {
                    HandType::FullHouse
                } else {
                    HandType::FourOfAKind
                }
            }
            3 => {
                if *card_counts.values().max().unwrap() == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type() > other.hand_type() {
            Some(std::cmp::Ordering::Greater)
        } else if self.hand_type() < other.hand_type() {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(self.cards.cmp(&other.cards))
        }
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let cards: Vec<Card> = parts
            .next()
            .map(|hand| hand.chars().map(|c| Card::from(c)))
            .unwrap()
            .collect();

        let bid: usize = parts.next().unwrap().parse().unwrap();

        Ok(Hand::new(bid, cards))
    }
}

fn part1(input: &str) -> usize {
    let mut hands = to_lines(input)
        .iter()
        .flat_map(|l| l.parse::<Hand>())
        .collect::<Vec<_>>();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(r, hand)| hand.bid * (r + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lines() {
        let input = "A\nB\nC";

        assert_eq!(to_lines(input), ["A", "B", "C"]);
    }

    #[test]
    fn test_card_order() {
        assert!(Card::A > Card::T);
        assert!(Card::Value(9) > Card::Value(5));
        assert!(Card::Value(2) < Card::Value(3));
    }

    #[test]
    fn test_card_parse() {
        assert_eq!(Card::from('A'), Card::A);
        assert_eq!(Card::from('7'), Card::Value(7));
    }

    #[test]
    fn test_hand_parse() {
        assert_eq!(
            "44JJ9 449".parse::<Hand>().unwrap(),
            Hand::new(
                449,
                vec![
                    Card::Value(4),
                    Card::Value(4),
                    Card::J,
                    Card::J,
                    Card::Value(9)
                ]
            )
        );

        assert_eq!(
            "TAK79 984".parse::<Hand>().unwrap(),
            Hand::new(
                984,
                vec![Card::T, Card::A, Card::K, Card::Value(7), Card::Value(9)]
            )
        );

        assert_eq!(
            "KKJKJ 959".parse::<Hand>().unwrap(),
            Hand::new(959, vec![Card::K, Card::K, Card::J, Card::K, Card::J])
        );
    }

    #[test]
    fn test_hand_type() {
        let hand: Hand = "32T3K 765".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::OnePair);

        let hand: Hand = "T55J5 684".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);

        let hand: Hand = "KK677 28".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::TwoPair);

        let hand: Hand = "KTJJT 220".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::TwoPair);

        let hand: Hand = "QQQJA 483".parse().unwrap();
        assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_hand_ordering() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::HighCard < HandType::OnePair);

        let four_of_a_kind: Hand = "KKKK3 123".parse().unwrap();
        let full_house: Hand = "22255 123".parse().unwrap();
        assert!(four_of_a_kind > full_house);

        let four_of_another_kind: Hand = "55554 123".parse().unwrap();
        assert!(four_of_a_kind > four_of_another_kind);

        let four_of_another_kind: Hand = "AKKKK 123".parse().unwrap();
        assert!(four_of_a_kind < four_of_another_kind);
    }

    #[test]
    fn test_make_numbers() {}

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");

        assert_eq!(part1(input), 6440);
    }
}
