fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);

    println!("Result: {}", result);
}

fn part2(input: &str) -> i64 {
    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.split_whitespace().flat_map(|n| n.parse()).rev().collect())
        .collect();

    sequences.iter().map(|seq| next_number(seq)).sum()
}

fn next_number(sequence: &Vec<i64>) -> i64 {
    let steps = steps(sequence);
    if steps.iter().all(|n| *n == steps[0]) {
        sequence.last().unwrap() + steps[0]
    } else {
        let next_step = next_number(&steps);
        sequence.last().unwrap() + next_step
    }
}

fn steps(sequence: &Vec<i64>) -> Vec<i64> {
    let mut sequence = sequence.iter();
    let mut last = sequence.next().unwrap();

    sequence
        .map(|n| {
            let next = n - last;
            last = n;
            next
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steps() {
        assert_eq!(steps(&vec![1, 2, 3]), vec![1, 1]);
        assert_eq!(steps(&vec![1, 3, 6, 10]), vec![2, 3, 4]);
    }

    #[test]
    fn test_next_number() {
        assert_eq!(next_number(&vec![0, 0, 0, 0]), 0);
        assert_eq!(next_number(&vec![1, 1, 1, 1]), 1);
        assert_eq!(next_number(&vec![1, 2, 3, 4]), 5);
        assert_eq!(next_number(&vec![2, 4, 6, 8]), 10);
        assert_eq!(next_number(&vec![15, 10, 6, 3]), 1);
        assert_eq!(next_number(&vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test1.txt");
        assert_eq!(part2(input), 2);
    }
}
