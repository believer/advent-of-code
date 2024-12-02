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
//
// I watched Chris Biscardi's solution to this problem and he had something similar to my solution.
// BUT, he said something like "I'm gonna flip destination and source, because the order is
// confusing to me". This got me thinking, maybe we can go backwards? Start from a location of
// 1 and reverse the map list. Increment location until we hit a valid seed number.
// We can immediately stop here because that location will be the lowest.
//
// After some fiddling, I got it to work. It's so much faster since we don't have to allocate
// a huge list of seeds. From 21 seconds (parallelized) to 238 milliseconds (!).
// That's more than 98% faster.

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
    fn perform_conversion(&self, location: u64, forward: bool) -> Option<u64> {
        // Setup the source and target based on the direction
        let (src, target) = match forward {
            true => (self.source, self.destination),
            false => (self.destination, self.source),
        };
        let bounds = src..=src + self.range_length;

        // Checking if the range contains the location makes it less
        // error prone, since my last attempt had an off by one error.
        // It's also more readable and made the code > 16% faster.
        if !bounds.contains(&location) {
            return None;
        }

        Some(target + location - src)
    }

    fn convert(&self, location: u64) -> Option<u64> {
        self.perform_conversion(location, true)
    }

    fn convert_backwards(&self, location: u64) -> Option<u64> {
        self.perform_conversion(location, false)
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
    // ChatGPT helped with creating a generic transform function
    // instead of duplicating the code for forwards and backwards.
    fn transform<F>(&self, location: u64, conversion_func: F) -> u64
    where
        F: Fn(&Conversion, u64) -> Option<u64>,
    {
        self.conversions
            .iter()
            .find_map(|c| conversion_func(c, location))
            .unwrap_or(location)
    }

    fn transform_forwards(&self, location: u64) -> u64 {
        self.transform(location, |c, loc| c.convert(loc))
    }

    fn transform_backwards(&self, location: u64) -> u64 {
        self.transform(location, |c, loc| c.convert_backwards(loc))
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
    maps.iter()
        .fold(location, |loc, map| map.transform_forwards(loc))
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
*/
#[aoc(day5, part2)]
pub fn solve_part_02(input: &Input) -> u64 {
    let Input { seeds, maps } = input;
    let mut lowest_end_location = 0;

    // Seeds are ranges in chunks of 2, start and length.
    // This creates a vector of ranges. Not like the first implementation
    // where I created a flat_map with ALL of the range values.
    let seed_ranges = seeds
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0];
            let end = start + chunk[1];

            start..end
        })
        .collect::<Vec<_>>();

    // Start at location 1 and traverse the maps backwards until we find a
    // valid seed value. This is the lowest end location.
    for loc in 0..=u64::MAX {
        let mut location = loc;

        for map in maps.iter().rev() {
            location = map.transform_backwards(location);
        }

        // Check if the location is in any of the seed ranges
        if seed_ranges.iter().any(|r| r.contains(&location)) {
            lowest_end_location = loc;
            break;
        }
    }

    lowest_end_location
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
