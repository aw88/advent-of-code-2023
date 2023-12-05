use core::str::FromStr;
use std::{collections::HashSet, fmt::Error};

fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);

    println!("Result: {}", result);
}

fn to_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[derive(Debug, PartialEq)]
struct Card {
    number: usize,
    playing_numbers: HashSet<usize>,
    winning_numbers: HashSet<usize>,
}

impl Card {
    fn new(
        number: usize,
        playing_numbers: HashSet<usize>,
        winning_numbers: HashSet<usize>,
    ) -> Self {
        Self {
            number,
            playing_numbers,
            winning_numbers,
        }
    }

    fn matches(&self) -> usize {
        self.playing_numbers
            .intersection(&self.winning_numbers)
            .count()
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").collect::<Vec<_>>();
        let card_number: usize = parts[0]
            .trim()
            .strip_prefix("Card")
            .and_then(|s| s.trim().parse().ok())
            .unwrap();

        let numbers: Vec<HashSet<usize>> = parts[1]
            .split(" | ")
            .map(|part| {
                part.split_whitespace()
                    .flat_map(|s| s.parse::<usize>())
                    .collect()
            })
            .collect();

        Ok(Card::new(
            card_number,
            numbers[0].clone(),
            numbers[1].clone(),
        ))
    }
}

fn part2(input: &str) -> usize {
    let lines = to_lines(input);
    let cards = lines
        .iter()
        .map(|l| l.parse::<Card>().unwrap())
        .collect::<Vec<_>>();
    let mut results: Vec<usize> = vec![1; cards.len()];

    for card in cards.iter() {
        let cards_won = card.number..(card.number + card.matches());
        for i in cards_won {
            results[i] += results[card.number - 1];
        }
    }

    results.iter().sum()
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
    fn test_parse_card() {
        assert_eq!(
            "Card 1: 1 2 3 | 4 5 6".parse(),
            Ok(Card {
                number: 1,
                playing_numbers: HashSet::from([1, 2, 3]),
                winning_numbers: HashSet::from([4, 5, 6]),
            })
        )
    }

    #[test]
    fn test_card_matches() {
        let card = Card::new(1, HashSet::from([1, 2, 3]), HashSet::from([2, 3, 4]));
        assert_eq!(card.matches(), 2);

        let card = Card::new(2, HashSet::from([1, 2, 3]), HashSet::from([4, 5, 6]));
        assert_eq!(card.matches(), 0);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test1.txt");

        assert_eq!(part2(input), 30);
    }
}
