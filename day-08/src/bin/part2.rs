use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);

    println!("Result: {}", result);
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next();

    let parts = lines.map(|l| {
        let mut parts = l.split(" = (");
        let index = parts.next().unwrap();
        let mut directions = parts.next().unwrap().split(", ");
        (
            index,
            (
                directions.next().unwrap(),
                directions.next().unwrap().strip_suffix(")").unwrap(),
            ),
        )
    });

    let nodes: HashMap<&str, (&str, &str)> = parts.collect();

    (instructions, nodes)
}

fn part2(input: &str) -> usize {
    let (instructions, nodes) = parse_input(input);

    let current = nodes.keys().filter(|n| n.ends_with("A")).map(|s| *s);

    let steps = current.map(|s| {
        let mut instructions = instructions.iter().cycle();
        let mut c = s;
        let mut step = 0;
        while !c.ends_with("Z") {
            step += 1;
            c = match instructions.next() {
                Some('R') => nodes.get(c).unwrap().1,
                Some('L') => nodes.get(c).unwrap().0,
                _ => unreachable!(),
            };
        }

        step
    });

    steps.fold(1, |acc, next| lcm(acc, next))
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("test3.txt");
        assert_eq!(part2(input), 6);
    }
}
