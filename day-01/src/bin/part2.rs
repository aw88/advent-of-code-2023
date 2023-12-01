fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);

    println!("Result: {}", result);
}

fn to_lines(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

enum NumOrder {
    First,
    Last,
}

fn find_number(line: &str, order: NumOrder) -> &'static str {
    let numbers = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    match *numbers
        .iter()
        .map(|number| match order {
            NumOrder::First => line.find(number).map(|idx| (idx, number)),
            NumOrder::Last => line.rfind(number).map(|idx| (idx, number)),
        })
        .flatten()
        .min_by(|a, b| match order {
            NumOrder::First => a.0.cmp(&b.0),
            NumOrder::Last => a.0.cmp(&b.0).reverse(),
        })
        .unwrap()
        .1
    {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        s => s,
    }
}

fn make_numbers(line: &str) -> u32 {
    let first = find_number(line, NumOrder::First);
    let last = find_number(line, NumOrder::Last);

    let str_num = format!("{}{}", first, last);

    u32::from_str_radix(&str_num, 10).unwrap()
}

fn part2(input: &str) -> u32 {
    let lines = to_lines(input);

    lines.iter().map(|line| make_numbers(&line)).sum::<u32>()
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
    fn test_find_first_number() {
        assert_eq!(find_number("eightwo", NumOrder::First), "8");
        assert_eq!(find_number("eigh7yse7en", NumOrder::First), "7");
    }

    #[test]
    fn test_find_last_number() {
        assert_eq!(find_number("eightwo", NumOrder::Last), "2");
    }

    #[test]
    fn test_make_numbers() {
        assert_eq!(make_numbers("a1dfg5oidf8sdf9a"), 19);
        assert_eq!(make_numbers("72"), 72);
        assert_eq!(make_numbers("9"), 99);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test2.txt");

        assert_eq!(part2(input), 281);
    }
}
