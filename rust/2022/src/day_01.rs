use std::{cmp, num::ParseIntError};

// Day 1

type Input = Vec<Result<u32, ParseIntError>>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|l| l.trim()).map(|l| l.parse()).collect()
}

/* Part One
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_01::*;
/// let data = include_str!("../input/2022/day1.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 69528);
/// ```
#[aoc(day1, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    let mut max: u32 = 0;
    let mut current: u32 = 0;

    for i in input {
        match i {
            Ok(v) => {
                current += v;
            }
            Err(_) => {
                max = cmp::max(max, current);
                current = 0;
            }
        }
    }

    max
}

/* Part Two
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_01::*;
/// let data = include_str!("../input/2022/day1.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 206152);
/// ```
#[aoc(day1, part2)]
pub fn solve_part_02(input: &Input) -> u32 {
    let mut all = vec![];
    let mut current: u32 = 0;
    let mut iter = 0;

    for i in input {
        iter += 1;

        match i {
            Ok(v) => {
                current += v;

                // Handle last value
                if iter == input.len() {
                    all.push(current);
                }
            }
            Err(_) => {
                all.push(current);
                current = 0;
            }
        }
    }

    all.sort();
    // Convert to an iterator, reverse it, get the first three values, and sum them
    all.iter().rev().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(solve_part_01(&input_generator(data)), 24000)
    }

    #[test]
    fn sample_02() {
        let data = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(solve_part_02(&input_generator(data)), 45000)
    }
}
