use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

// Day 4 - Camp Cleanup
//
// Today we got a list of elves on cleaning duty. They are assigned
// in pairs, each with a range of sections they are responsible for.

type Groups = Vec<Vec<RangeInclusive<u32>>>;

pub fn parse_pair(pair: &str) -> Vec<u32> {
    // Split the pairs
    pair.split(',')
        .flat_map(|n| {
            // Find the sections for each elf
            n.split('-')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Groups {
    input
        .lines()
        .map(parse_pair)
        .map(|v| vec![v[0]..=v[1], v[2]..=v[3]])
        .collect()
}

/* Part One
 *
 * The elves are assigned to clean up the camp in pairs.
 * For example, consider the following list of section assignment
 * pairs (the puzzle input):
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
#[aoc(day4, part1)]
pub fn solve_part_01(groups: &Groups) -> u32 {
    let mut overlap = 0;

    for group in groups {
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
 * In this part, we need to find the pairs that partially overlap.
*/
#[aoc(day4, part2)]
pub fn solve_part_02(groups: &Groups) -> u32 {
    let mut overlap = 0;

    for group in groups {
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

    const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 2)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 4)
    }
}
