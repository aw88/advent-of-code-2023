fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);

    println!("Result: {}", result);
}

fn lookup_map(map: &Vec<(usize, usize, usize)>, value: usize) -> usize {
    map.iter()
        .find(|(_, from, size)| value >= *from && value < (from + size))
        .and_then(|(to, from, _)| Some(to + (value - from)))
        .unwrap_or(value)
}

fn read_seed_ranges(input: &str) -> Vec<(usize, usize)> {
    input
        .split("\n")
        .nth(0)
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1]))
        .collect()
}

fn read_map(input: &str, map_name: &str) -> Vec<(usize, usize, usize)> {
    input
        .split("\n")
        .skip_while(|s| !s.starts_with(map_name))
        .skip(1)
        .take_while(|l| l.starts_with(|c: char| c.is_numeric()))
        .map(|l| {
            let numbers: Vec<usize> = l.split_whitespace().map(|s| s.parse().unwrap()).collect();
            (numbers[0], numbers[1], numbers[2])
        })
        .collect()
}

macro_rules! pipeline {
    ( $expr:expr => $($funs:tt)=>+ ) => {
        {
            let ret = $expr;
            $(
                let ret = lookup_map(&$funs, ret);
            )*
            ret
        }
    };
}

fn part2(input: &str) -> usize {
    let seed_ranges = read_seed_ranges(input);

    let seed_to_soil = read_map(input, "seed-to-soil");
    let soil_to_fertilizer = read_map(input, "soil-to-fertilizer");
    let fertilizer_to_water = read_map(input, "fertilizer-to-water");
    let water_to_light = read_map(input, "water-to-light");
    let light_to_temperature = read_map(input, "light-to-temperature");
    let temperature_to_humidity = read_map(input, "temperature-to-humidity");
    let humidity_to_location = read_map(input, "humidity-to-location");

    seed_ranges
        .iter()
        .map(|seed_range| {
            let seeds = (seed_range.0..seed_range.1).collect::<Vec<usize>>();

            // Write some progress to the terminal
            println!(
                "SEEDS: {} to {} ({})",
                seed_range.0,
                seed_range.1,
                seed_range.1 - seed_range.0
            );

            seeds
                .iter()
                .map(|seed| {
                    pipeline!(*seed
                        => seed_to_soil
                        => soil_to_fertilizer
                        => fertilizer_to_water
                        => water_to_light
                        => light_to_temperature
                        => temperature_to_humidity
                        => humidity_to_location)
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_seed_ranges() {
        let input = "seeds: 1 2 3 4\n\nseed-to-soil map:\n 1 2 3 4";
        assert_eq!(read_seed_ranges(input), vec![(1, 3), (3, 7)]);

        let input = include_str!("test1.txt");
        assert_eq!(read_seed_ranges(input), vec![(79, 93), (55, 68)]);
    }

    #[test]
    fn test_read_map() {
        let input = include_str!("test1.txt");
        assert_eq!(
            read_map(input, "seed-to-soil"),
            vec![(50, 98, 2), (52, 50, 48)]
        );

        assert_eq!(
            read_map(input, "temperature-to-humidity"),
            vec![(0, 69, 1), (1, 0, 69),]
        );
    }

    #[test]
    fn test_lookup_map() {
        let map = vec![(50, 98, 2), (52, 50, 48)];

        assert_eq!(lookup_map(&map, 79), 81);
        assert_eq!(lookup_map(&map, 14), 14);
        assert_eq!(lookup_map(&map, 55), 57);
        assert_eq!(lookup_map(&map, 13), 13);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test1.txt");

        assert_eq!(part2(input), 46);
    }
}
