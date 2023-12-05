use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// Day 5: If You Give A Seed A Fertilizer
//
// I first created a brute force solution that just went through each line
// creating a hashmap of all the locations. Then step from map to map. This worked fine
// for the example data. But, the real data had huge numbers, and it broke down completely.
// Another lesson in never completely trusting the example data (see day 15 2022).
//
// The first part is fast because the list of seeds is small, but when we get to the second part
// the list of seeds is huge. The solution becomes very slow, so it's still pretty brute force.
// Using rayon to parallelize the search makes it faster, but it's still slow. Rayon decreases
// the time from 120 seconds to 21 seconds on my machine.

pub struct Input {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
pub struct Conversion {
    destination: u64,
    source: u64,
    range_length: u64,
}

impl Conversion {
    fn convert(&self, location: u64) -> Option<u64> {
        // Check if location is within range
        let lower_bound = self.source;
        let upper_bound = self.source + self.range_length;
        let bounds = lower_bound..=upper_bound;

        // Checking if the range contains the location makes it less
        // error prone, since my last attempt had an off by one error.
        // It's also more readable and made the code > 16% faster.
        if !bounds.contains(&location) {
            return None;
        }

        Some(self.destination + location - self.source)
    }
}

impl From<&str> for Conversion {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();

        let destination = parts.next().unwrap().parse::<u64>().unwrap();
        let source = parts.next().unwrap().parse::<u64>().unwrap();
        let range_length = parts.next().unwrap().parse::<u64>().unwrap();

        Self {
            destination,
            source,
            range_length,
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
        // Skip 1 to remove the header
        let conversions = s.lines().skip(1).map(|line| line.into()).collect();

        Self { conversions }
    }
}

// Step through the maps and until we find the location
fn find_location(maps: &[Map], location: u64) -> u64 {
    maps.iter().fold(location, |loc, map| map.transform(loc))
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
* Given a list of seeds, and a list of maps, walk through the maps
* to find the location of the seeds. Return the lowest location.
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_05::*;
/// let data = include_str!("../input/2023/day5.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 993500720);
/// ```
#[aoc(day5, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    let Input { seeds, maps } = input;

    seeds.iter().map(|s| find_location(maps, *s)).min().unwrap()
}

/* Part Two
*
* The seeds list is now a list of ranges in pairs of two.
* The first number is the start of the range, and the second
* number is the length of the range.
*
* Otherwise, the calculation is the same as part one. Just A LOT more seeds.
*
* No doctest for this one, it takes too long.
* Your puzzle answer was: 4917124
*/
#[aoc(day5, part2)]
pub fn solve_part_02(input: &Input) -> u64 {
    let Input { seeds, maps } = input;

    // Seeds are ranges in chunks of 2, start and length.
    // Convert them to a list. The input data has huge numbers,
    // so this list will also be huge.
    let seed_ranges =
        seeds
            .chunks(2)
            .flat_map(|chunk| {
                let start = chunk[0];
                let end = start + chunk[1];

                start..end
            })
            .collect::<Vec<_>>();

    seed_ranges
        // Search takes about 120 seconds on my machine
        // Using rayon to parallelize the search takes about 21 seconds
        // Thanks to ChatGPT for the suggestion
        .par_iter()
        .map(|s| find_location(maps, *s))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "seeds: 79 14 55 13

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

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 35)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(DATA)), 46)
    }
}
