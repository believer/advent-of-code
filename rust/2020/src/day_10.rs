use crate::common;
use itertools::zip;
use itertools::Itertools;
use std::collections::HashSet;

// Day 10

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> HashSet<u32> {
    common::input_hashset(input)
}

///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_10::*;
/// let input = include_str!("../input/2020/day10.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 2475);
/// ```
#[aoc(day10, part1)]
pub fn solve_part_01(input: &HashSet<u32>) -> u32 {
    let mut ones = 0;
    let mut threes = 0;
    let mut data = input.clone();

    data.insert(0);
    data.insert(*input.iter().max().unwrap() + 3);

    for (a, b) in zip(
        &data.iter().sorted().collect::<Vec<_>>(),
        &data.iter().sorted().collect::<Vec<_>>()[1..],
    ) {
        match *b - *a {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }

    ones * threes
}

/////your puzzle answer was.
///// ```
///// use advent_of_code_2020::day_10::*;
///// let input = include_str!("../input/2020/day10.txt");
///// assert_eq!(solve_part_02(&input_generator(input)), 2986195);
///// ```
//#[aoc(day10, part2)]
//pub fn solve_part_02(input: &[u64]) -> u64 {
//    find_weakness(input, 25)
//}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn find_broken_number_for_sample() {
        let data = "16
10
15
5
1
11
7
19
6
12
4
";

        assert_eq!(solve_part_01(&input_generator(data)), 35)
    }
}
