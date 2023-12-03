use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);

    println!("Result: {}", result);
}

enum BoardEntry {
    Number(u32),
    Symbol(char),
}

fn part2(input: &str) -> u32 {
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

    let mut gear_ratios: Vec<u32> = vec![];

    for (p, entry) in board.iter() {
        if let BoardEntry::Symbol('*') = entry {
            let mut gears: HashSet<(u32, usize, usize)> = HashSet::new();

            let neighbours = [
                (p.0 - 1, p.1 - 1), (p.0, p.1 - 1), (p.0 + 1, p.1 - 1),
                (p.0 - 1, p.1), (p.0 + 1, p.1),
                (p.0 - 1, p.1 + 1), (p.0, p.1 + 1), (p.0 + 1, p.1 + 1),
            ];

            for neighbour in neighbours {
                if let Some(BoardEntry::Number(n)) = board.get(&neighbour) {
                    gears.insert((*n, neighbour.0, neighbour.1));
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
                        gears.insert((*n, neighbour.0, neighbour.1));
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
                        gears.insert((*n, neighbour.0, neighbour.1));
                    }
                }
            }

            if gears.len() == 2 {
                gear_ratios.push(gears.iter().map(|(n,_,_)| n).product());
            }
        }
    }

    // numbers.iter().filter(|(_,v)| **v == 0).map(|(n, _)| n).sum()
    gear_ratios.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("test2.txt");
        assert_eq!(part2(input), 467835);
    }
}
