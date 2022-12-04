use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

// Day 4 - Camp Cleanup

type Input = Vec<Vec<RangeInclusive<u32>>>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .flat_map(|n| {
                    n.split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect()
        })
        .map(|v: Vec<u32>| vec![v[0]..=v[1], v[2]..=v[3]])
        .collect()
}

/* Part One
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
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_04::*;
/// let data = include_str!("../input/2022/day4.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 0);
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
