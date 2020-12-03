use crate::common;
use anyhow::{anyhow, Error};
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;

// Day 2 - Password Philosophy
//
// Updated multiple times to get faster speeds.
// Fastest, also a very clean solution, seems to be using a RegEx

lazy_static! {
    /// Input is in the form "1-3 a: abcdef"
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").unwrap();
}

/// Official Toboggan Corporate Policy
pub struct OTCP {
    password: String,
    policy: char,
    bounds: RangeInclusive<usize>,
}

impl std::str::FromStr for OTCP {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s).ok_or_else(|| anyhow!("Unable to parse"))
    }
}

fn parse(line: &str) -> Option<OTCP> {
    let results = RE.captures(line)?;
    let lower_bound = results.get(1)?.as_str().parse().ok()?;
    let upper_bound = results.get(2)?.as_str().parse().ok()?;

    Some(OTCP {
        password: results.get(4)?.as_str().to_string(),
        bounds: (lower_bound..=upper_bound),
        policy: results.get(3)?.as_str().chars().next()?,
    })
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<OTCP> {
    common::input_vec(input)
}

/* Part One
 *
 * Your flight departs in a few days from the coastal airport;
 * the easiest way down to the coast from here is via toboggan.
 *
 * The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
 * "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
 *
 * Their password database seems to be a little corrupted:
 * some of the passwords wouldn't have been allowed by the
 * Official Toboggan Corporate Policy that was in effect when they were chosen.
 *
 * To try to debug the problem, they have created a list (your puzzle input)
 * of passwords (according to the corrupted database)
 * and the corporate policy when that password was set.
 *
 * For example, suppose you have the following list:
 *
 * 1-3 a: abcde
 * 1-3 b: cdefg
 * 2-9 c: ccccccccc
 *
 * Each line gives the password policy and then the password.
 * The password policy indicates the lowest and highest number of times
 * a given letter must appear for the password to be valid.
 * For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
 *
 * In the above example, 2 passwords are valid. The middle password, cdefg, is not;
 * it contains no instances of b, but needs at least 1. The first and third passwords are valid:
 * they contain one a or nine c, both within the limits of their respective policies.
 *
 * How many passwords are valid according to their policies?
*/
///Your puzzle answer was
/// ```
/// use advent_of_code_2020::day_02::*;
/// let input = include_str!("../input/2020/day2.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 524);
/// ```
#[aoc(day2, part1)]
pub fn solve_part_01(input: &Vec<OTCP>) -> usize {
    input
        .iter()
        .filter(|otcp| {
            otcp.bounds.contains(
                &(*otcp.password)
                    .chars()
                    .filter(|c| c == &otcp.policy)
                    .count(),
            )
        })
        .count()
}

/* Part Two
 *
 * While it appears you validated the passwords correctly,
 * they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
 *
 * The shopkeeper suddenly realizes that he just accidentally explained the password
 * policy rules from his old job at the sled rental place down the street!
 * The Official Toboggan Corporate Policy actually works a little differently.
 *
 * Each policy actually describes two positions in the password, where 1 means the first character,
 * 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!)
 * Exactly one of these positions must contain the given letter.
 * Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
 *
 * Given the same example list from above:
 *
 * 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
 * 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
 * 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
 *
 * How many passwords are valid according to the new interpretation of the policies?
*/
///your puzzle answer was
/// ```
/// use advent_of_code_2020::day_02::*;
/// let input = include_str!("../input/2020/day2.txt");
/// assert_eq!(solve_part_02(&input_generator(input)), 485);
/// ```
#[aoc(day2, part2)]
pub fn solve_part_02(input: &Vec<OTCP>) -> usize {
    input
        .iter()
        .filter(|otcp| {
            let (lower, upper) = otcp.bounds.clone().into_inner();
            let lower_value = otcp.password.chars().nth(lower - 1).unwrap();
            let upper_value = otcp.password.chars().nth(upper - 1).unwrap();

            (lower_value == otcp.policy) ^ (upper_value == otcp.policy)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn sample_01() {
        let data = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

        assert_eq!(solve_part_01(&input_generator(data)), 2)
    }

    /// Test example data on part 2
    #[test]
    fn sample_02() {
        let data = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

        assert_eq!(solve_part_02(&input_generator(data)), 1)
    }
}
