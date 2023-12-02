use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);

    println!("Result: {}", result);
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    number: u32,
    rounds: Vec<GameRound>,
}

impl Game {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    fn is_possible(&self) -> bool {
        self.rounds.iter().all(|r| {
            r.red <= Self::MAX_RED && r.green <= Self::MAX_GREEN && r.blue <= Self::MAX_BLUE
        })
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split(": ").collect::<Vec<_>>();

        Ok(Game {
            number: parts[0].split(" ").last().unwrap().parse().unwrap(),
            rounds: parts[1]
                .split("; ")
                .map(|round| round.parse().unwrap())
                .collect(),
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
struct GameRound {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameRound {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }

    fn reduce(left: Self, right: Self) -> Self {
        Self::new(
            usize::max(left.red, right.red),
            usize::max(left.green, right.green),
            usize::max(left.blue, right.blue),
        )
    }

    #[rustfmt::skip]
    fn with_red(red: usize) -> Self {
        Self { red, ..Default::default() }
    }
    #[rustfmt::skip]
    fn with_green(green: usize) -> Self {
        Self { green, ..Default::default() }
    }
    #[rustfmt::skip]
    fn with_blue(blue: usize) -> Self {
        Self { blue, ..Default::default() }
    }
}

impl Default for GameRound {
    fn default() -> Self {
        GameRound {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for GameRound {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.trim()
            .split(", ")
            .map(
                |p| match p.trim().split(" ").collect::<Vec<_>>().as_slice() {
                    [count, "red"] => GameRound::with_red(count.parse().unwrap()),
                    [count, "green"] => GameRound::with_green(count.parse().unwrap()),
                    [count, "blue"] => GameRound::with_blue(count.parse().unwrap()),
                    _ => GameRound::default(),
                },
            )
            .reduce(GameRound::reduce)
            .unwrap())
    }
}

fn to_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

fn part1(input: &str) -> u32 {
    to_lines(input)
        .iter()
        .map(|line| line.parse::<Game>())
        .flatten()
        .filter(|g| g.is_possible())
        .map(|g| g.number)
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
    fn test_parse_game_round() {
        assert_eq!(
            "4 red, 3 blue, 2 green".parse(),
            Ok(GameRound::new(4, 2, 3))
        );
        assert_eq!(
            "9 green, 1 red, 27 blue".parse(),
            Ok(GameRound::new(1, 9, 27))
        );
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            "Game 1: 1 red, 2 green, 3 blue".parse(),
            Ok(Game {
                number: 1,
                rounds: vec![GameRound::new(1, 2, 3)]
            })
        );
        assert_eq!(
            "Game 3: 1 red, 5 blue, 1 green; 5 red, 1 blue, 5 green".parse(),
            Ok(Game {
                number: 3,
                rounds: vec![GameRound::new(1, 1, 5), GameRound::new(5, 5, 1)]
            })
        );
    }

    #[test]
    fn test_is_game_possible() {
        let game = Game {
            number: 1,
            rounds: vec![GameRound::new(Game::MAX_RED, Game::MAX_GREEN, Game::MAX_BLUE)]
        };
        assert_eq!(game.is_possible(), true);

        let game = Game {
            number: 1,
            rounds: vec![GameRound::new(Game::MAX_RED + 1, Game::MAX_GREEN, Game::MAX_BLUE)]
        };
        assert_eq!(game.is_possible(), false);

        let game = Game {
            number: 1,
            rounds: vec![GameRound::new(Game::MAX_RED, Game::MAX_GREEN + 1, Game::MAX_BLUE)]
        };
        assert_eq!(game.is_possible(), false);

        let game = Game {
            number: 1,
            rounds: vec![GameRound::new(Game::MAX_RED, Game::MAX_GREEN, Game::MAX_BLUE + 1)]
        };
        assert_eq!(game.is_possible(), false);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("test1.txt");

        assert_eq!(part1(input), 8);
    }
}
