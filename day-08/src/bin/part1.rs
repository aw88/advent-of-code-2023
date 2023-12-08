use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);

    println!("Result: {}", result);
}

fn parse_input(input: &str) -> (impl Iterator<Item = char> + '_, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars().into_iter().cycle();
    lines.next();

    let parts = lines.map(|l| {
            let mut parts = l.split(" = (");
            let index = parts.next().unwrap();
            let mut directions = parts.next().unwrap().split(", ");
            (index, (directions.next().unwrap(), directions.next().unwrap().strip_suffix(")").unwrap()))
        });

    let nodes: HashMap<&str, (&str, &str)> = parts.collect();

    (instructions, nodes)
}

fn part1(input: &str) -> u32 {
    let (mut instructions, nodes) = parse_input(input);

    let mut current = "AAA";
    let mut step = 0;
    while current != "ZZZ" {
        step += 1;
        current = match instructions.next() {
            Some('R') => nodes.get(current).unwrap().1,
            Some('L') => nodes.get(current).unwrap().0,
            _ => unreachable!(),
        }

    }

    step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test1.txt");
        assert_eq!(part1(input), 6);

        let input = include_str!("test2.txt");
        assert_eq!(part1(input), 2);
    }
}
