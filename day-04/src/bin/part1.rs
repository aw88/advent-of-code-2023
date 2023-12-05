use core::str::FromStr;
use std::{collections::HashSet, fmt::Error};

fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);

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
    fn new(number: usize, playing_numbers: HashSet<usize>, winning_numbers: HashSet<usize>) -> Self {
        Self { number, playing_numbers, winning_numbers }
    }

    fn value(&self) -> u32 {
        match self.playing_numbers.intersection(&self.winning_numbers).count() as u32 {
            0 => 0,
            matches => u32::pow(2, matches - 1),
        }
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(": ").collect::<Vec<_>>();
        let card_number: usize = parts[0].trim().strip_prefix("Card").and_then(|s| s.trim().parse().ok()).unwrap();

        let numbers: Vec<HashSet<usize>> = parts[1]
            .split(" | ")
            .map(|part| part.split_whitespace().flat_map(|s| s.parse::<usize>()).collect())
            .collect();

        Ok(Card::new(card_number, numbers[0].clone(), numbers[1].clone()))
    }
}

fn part1(input: &str) -> u32 {
    let lines = to_lines(input);
    let cards = lines.iter().map(|l| l.parse::<Card>().unwrap());

    cards.map(|c| c.value()).sum::<u32>()
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
        assert_eq!("Card 1: 1 2 3 | 4 5 6".parse(), Ok(Card {
            number: 1,
            playing_numbers: HashSet::from([1, 2, 3]),
            winning_numbers: HashSet::from([4, 5, 6]),
        }))
    }

    #[test]
    fn test_part1() {
        let input = include_str!("test1.txt");

        assert_eq!(part1(input), 13);
    }
}