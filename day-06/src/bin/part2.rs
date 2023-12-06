fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);

    println!("Result: {}", result);
}

fn parse_races(input: &str) -> (usize, usize) {
    let mut lines = input.split("\n");

    let time = lines
        .nth(0)
        .and_then(|s| s.strip_prefix("Time:"))
        .map(|s| s.split_whitespace().fold("".to_string(), |acc, n| format!("{}{}", acc, n)))
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap();

    let distance = lines
        .nth(0)
        .and_then(|s| s.strip_prefix("Distance:"))
        .map(|s| s.split_whitespace().fold("".to_string(), |acc, n| format!("{}{}", acc, n)))
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap();

    (time, distance)
}

fn calculate_distance(hold_time: usize, total_time: usize) -> usize {
    (total_time - hold_time) * hold_time
}

fn winning_moves(time: usize, winning_distance: usize) -> usize {
    (1..time)
        .map(|hold| calculate_distance(hold, time))
        .filter(|distance| distance > &winning_distance)
        .count()
}

fn part2(input: &str) -> usize {
    let (time, distance) = parse_races(input);

    winning_moves(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_races() {
        let input = include_str!("test.txt");
        assert_eq!(parse_races(input), (71530, 940200));
        assert_eq!(
            parse_races("Time: 1 2 3\nDistance: 4 5 6"),
            (123, 456)
        );
    }

    #[test]
    fn test_calculate_distance() {
        assert_eq!(calculate_distance(1, 7), 6);
        assert_eq!(calculate_distance(2, 7), 10);
        assert_eq!(calculate_distance(3, 7), 12);
        assert_eq!(calculate_distance(4, 7), 12);
        assert_eq!(calculate_distance(5, 7), 10);
        assert_eq!(calculate_distance(6, 7), 6);
    }

    #[test]
    fn test_winning_moves() {
        assert_eq!(winning_moves(7, 9), 4);
        assert_eq!(winning_moves(15, 40), 8);
        assert_eq!(winning_moves(30, 200), 9);
        assert_eq!(winning_moves(71530, 940200), 71503);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test.txt");

        assert_eq!(part2(input), 71503);
    }
}
