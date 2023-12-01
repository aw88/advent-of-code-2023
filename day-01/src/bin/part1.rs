fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);

    println!("Result: {}", result);
}

fn to_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

fn make_numbers(line: &str) -> u32 {
    let first = line.chars().find(|c: &char| c.is_numeric()).unwrap();
    let last = line.chars().rev().find(|c: &char| c.is_numeric()).unwrap();

    let str_num = format!("{}{}", first, last);

    u32::from_str_radix(&str_num, 10).unwrap()
}

fn part1(input: &str) -> u32 {
    let lines = to_lines(input);
    lines.iter().map(|line| make_numbers(line)).sum::<u32>()
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
    fn test_make_numbers() {
        assert_eq!(make_numbers("a1dfg5oidf8sdf9a"), 19);
        assert_eq!(make_numbers("72"), 72);
        assert_eq!(make_numbers("9"), 99);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("test1.txt");

        assert_eq!(part1(input), 142);
    }
}