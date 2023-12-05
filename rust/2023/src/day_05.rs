// Day 5

pub struct Input {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
pub struct Conversion {
    destination: u64,
    source: u64,
    range_left: u64,
}

impl Conversion {
    fn convert(&self, location: u64) -> Option<u64> {
        // Check if location is within range
        let lower_bound = self.source;
        let upper_bound = self.source + self.range_left;

        match location {
            x if x < lower_bound => None,
            x if x > upper_bound => None,
            _ => Some(self.destination + location - self.source),
        }
    }
}

impl From<&str> for Conversion {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();

        let destination = parts.next().unwrap().parse::<u64>().unwrap();
        let source = parts.next().unwrap().parse::<u64>().unwrap();
        let range_left = parts.next().unwrap().parse::<u64>().unwrap();

        Self {
            destination,
            source,
            range_left,
        }
    }
}

#[derive(Debug)]
pub struct Map {
    conversions: Vec<Conversion>,
}

impl Map {
    fn transform(&self, location: u64) -> u64 {
        self.conversions
            .iter()
            .find_map(|c| c.convert(location))
            // If no conversion is found, return the original location
            .unwrap_or(location)
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        // Skip header
        let conversions_lines = s.trim().lines().skip(1);
        let conversions = conversions_lines.map(|line| line.trim().into()).collect();

        Self { conversions }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    // Split input into parts, seeds and maps
    let parts = input.split("\n\n").collect::<Vec<_>>();

    // Parse seeds
    let seeds = parts[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let maps = parts[1..].iter().map(|p| (*p).into()).collect::<Vec<Map>>();

    Input { seeds, maps }
}

/* Part One
*
*/
// Your puzzle answer was
/*
/// ```
/// use advent_of_code_2023::day_05::*;
/// let data = include_str!("../input/2023/day5.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 54304);
/// ```
*/
#[aoc(day5, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    let Input { seeds, maps } = input;

    seeds
        .iter()
        .map(|s| {
            let mut location = *s;

            for map in maps.iter() {
                location = map.transform(location);
            }

            location
        })
        .min()
        .unwrap()
}

/* Part Two
*
*/
// Your puzzle answer was
/*
/// ```
/// use advent_of_code_2023::day_05::*;
/// let data = include_str!("../input/2023/day5.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 54418);
/// ```
*/
#[aoc(day5, part2)]
pub fn solve_part_02(_input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(solve_part_01(&input_generator(data)), 35)
    }
}
