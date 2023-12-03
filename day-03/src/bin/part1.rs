use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);

    println!("Result: {}", result);
}

enum BoardEntry {
    Number(u32),
    Symbol(char),
}

fn part1(input: &str) -> u32 {
    let lines = input.split("\n").collect::<Vec<&str>>();

    let numbers_re = Regex::new(r"(\d+)").unwrap();
    let symbols_re = Regex::new(r"([^0-9\.])").unwrap();

    let mut board: HashMap<(usize, usize), BoardEntry> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, number) in numbers_re.captures_iter(line)
            .map(|c| {
                let capture = c.get(0).unwrap();
                (capture.start(), capture.as_str())
            }) {
            board.insert((x, y), BoardEntry::Number(number.parse().unwrap()));
        }

        for (x, symbol) in symbols_re.captures_iter(line)
            .map(|c| {
                let capture = c.get(0).unwrap();
                (capture.start(), capture.as_str())
            }) {
            board.insert((x, y), BoardEntry::Symbol(symbol.chars().nth(0).unwrap()));
        }
    }

    let mut numbers: HashSet<(u32, usize, usize)> = HashSet::new();

    for (p, entry) in board.iter() {
        if let BoardEntry::Symbol(_) = entry {
            let neighbours = [
                (p.0 - 1, p.1 - 1), (p.0, p.1 - 1), (p.0 + 1, p.1 - 1),
                (p.0 - 1, p.1), (p.0 + 1, p.1),
                (p.0 - 1, p.1 + 1), (p.0, p.1 + 1), (p.0 + 1, p.1 + 1),
            ];

            for neighbour in neighbours {
                if let Some(BoardEntry::Number(n)) = board.get(&neighbour) {
                    numbers.insert((*n, neighbour.0, neighbour.1));
                }
            }

            let neighbours = [
                (p.0 - 2, p.1 - 1),
                (p.0 - 2, p.1),
                (p.0 - 2, p.1 + 1),
            ];

            for neighbour in neighbours {
                if let Some(BoardEntry::Number(n)) = board.get(&neighbour) {
                    if *n > 9 {
                        numbers.insert((*n, neighbour.0, neighbour.1));
                    }
                }
            }

            let neighbours = [
                (p.0 - 3, p.1 - 1),
                (p.0 - 3, p.1),
                (p.0 - 3, p.1 + 1),
            ];

            for neighbour in neighbours {
                if let Some(BoardEntry::Number(n)) = board.get(&neighbour) {
                    if *n > 99 {
                        numbers.insert((*n, neighbour.0, neighbour.1));
                    }
                }
            }
        }
    }

    numbers.iter().map(|(n, _, _)| n).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test1.txt");
        assert_eq!(part1(input), 4361);

        let input = concat!(
            ".123..\n",
            "....*.\n",
            "......"
        );
        assert_eq!(part1(input), 123);
        
        let input = concat!(
            "......\n",
            ".123*.\n",
            "......"
        );
        assert_eq!(part1(input), 123);

        let input = concat!(
            "......\n",
            "....*.\n",
            ".123.."
        );
        assert_eq!(part1(input), 123);

        let input = concat!(
            ".12...\n",
            ".12.*.\n",
            ".12..."
        );
        assert_eq!(part1(input), 0);

        let input = concat!(
            ".12..\n",
            ".12*.\n",
            "....."
        );
        assert_eq!(part1(input), 24);
    }
}
