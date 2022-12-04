use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

// Day 4 - Camp Cleanup
//
// Today we got a list of with elves on cleaning duty. They are assigned
// in pairs with a range of sections they are responsible for.

type Input = Vec<Vec<RangeInclusive<u32>>>;

// Split the pair into two ranges
pub fn parse_pair(pair: &str) -> Vec<u32> {
    pair.split(',')
        .flat_map(|n| {
            n.split('-')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<u32>>()
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| parse_pair(l))
        .map(|v| vec![v[0]..=v[1], v[2]..=v[3]])
        .collect()
}

/* Part One
 *
 * In this part, the elves are assigned to clean up the camp in pairs.
 * For example, consider the following list of section assignment pairs:
 *
 * 2-4,6-8
 * 2-3,4-5
 * 5-7,7-9
 * 2-8,3-7
 * 6-6,4-6
 * 2-6,4-8
 *
 * This can be represented as:
 *
 * .234.....  2-4
 * .....678.  6-8
 *
 * .23......  2-3
 * ...45....  4-5
 *
 * ....567..  5-7
 * ......789  7-9
 *
 * .2345678.  2-8
 * ..34567..  3-7
 *
 * .....6...  6-6
 * ...456...  4-6
 *
 * .23456...  2-6
 * ...45678.  4-8
 *
 * Find how many pairs overlap completely. In the example above, the answer would be 2.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_04::*;
/// let data = include_str!("../input/2022/day4.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 550);
/// ```
#[aoc(day4, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    let mut overlap = 0;

    for group in input {
        let first = group.first().unwrap();
        let second = group.get(1).unwrap();

        if first.contains(second.start()) && first.contains(second.end())
            || second.contains(first.start()) && second.contains(first.end())
        {
            overlap += 1;
        }
    }

    overlap
}

/* Part Two
 *
 * In this part, we need to find the pairs that overlap partially.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_04::*;
/// let data = include_str!("../input/2022/day4.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 931);
/// ```
#[aoc(day4, part2)]
pub fn solve_part_02(input: &Input) -> u32 {
    let mut overlap = 0;

    for group in input {
        let first = group.first().unwrap();
        let second = group.get(1).unwrap();

        if max(first.start(), second.start()) <= min(first.end(), second.end()) {
            overlap += 1;
        }
    }

    overlap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(solve_part_01(&input_generator(data)), 2)
    }

    #[test]
    fn sample_02() {
        let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(solve_part_02(&input_generator(data)), 4)
    }
}
