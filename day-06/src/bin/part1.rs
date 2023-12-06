fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);

    println!("Result: {}", result);
}

fn parse_races(input: &str) -> Vec<(usize, usize)> {
    let mut lines = input.split("\n");

    let times = lines
        .nth(0)
        .and_then(|s| s.strip_prefix("Time:"))
        .map(|s| s.split_whitespace().flat_map(|n| n.parse::<usize>()))
        .unwrap();

    let distances = lines
        .nth(0)
        .and_then(|s| s.strip_prefix("Distance:"))
        .map(|s| s.split_whitespace().flat_map(|n| n.parse::<usize>()))
        .unwrap();

    times.zip(distances).collect()
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

fn part1(input: &str) -> usize {
    let races = parse_races(input);

    races
        .iter()
        .map(|(time, distance)| winning_moves(*time, *distance))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_races() {
        let input = include_str!("test.txt");
        assert_eq!(parse_races(input), vec![(7, 9), (15, 40), (30, 200)]);
        assert_eq!(
            parse_races("Time: 1 2 3\nDistance: 4 5 6"),
            vec![(1, 4), (2, 5), (3, 6)]
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
    }

    #[test]
    fn test_part1() {
        let input = include_str!("test.txt");

        assert_eq!(part1(input), 288);
    }
}
