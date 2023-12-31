fn main() {
    let input = include_str!("input.txt");
    let result = part1(input);

    println!("Result: {}", result);
}

fn lookup_map(map: &Vec<(usize, usize, usize)>, value: usize) -> usize {
    map.iter()
        .find(|(_, from, size)| value >= *from && value < (from + size))
        .and_then(|(to, from, _)| Some(to + (value - from)))
        .unwrap_or(value)
}

fn read_seeds(input: &str) -> Vec<usize> {
    input
        .split("\n")
        .nth(0)
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|s| s.parse().unwrap())
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

fn part1(input: &str) -> usize {
    let seeds = read_seeds(input);

    let seed_to_soil = read_map(input, "seed-to-soil");
    let soil_to_fertilizer = read_map(input, "soil-to-fertilizer");
    let fertilizer_to_water = read_map(input, "fertilizer-to-water");
    let water_to_light = read_map(input, "water-to-light");
    let light_to_temperature = read_map(input, "light-to-temperature");
    let temperature_to_humidity = read_map(input, "temperature-to-humidity");
    let humidity_to_location = read_map(input, "humidity-to-location");

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_seeds() {
        let input = "seeds: 1 2 3\n\nseed-to-soil map:\n 1 2 3";
        assert_eq!(read_seeds(input), vec![1, 2, 3]);

        let input = include_str!("test1.txt");
        assert_eq!(read_seeds(input), vec![79, 14, 55, 13]);
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
    fn test_part1() {
        let input = include_str!("test1.txt");

        assert_eq!(part1(input), 35);
    }
}
