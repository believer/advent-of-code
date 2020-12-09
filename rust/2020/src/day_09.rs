use crate::common;
use itertools::iproduct;
use std::collections::HashSet;

// Day 9

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    common::input_vec(input)
}

fn calculate_sums(input: &[u64]) -> HashSet<u64> {
    iproduct!(input.iter(), input.iter())
        .map(|tuple| tuple.0 + tuple.1)
        .collect()
}

fn find_weakness(input: &[u64], preamble: usize) -> u64 {
    let mut not_in: HashSet<u64> = HashSet::new();

    for i in preamble..input.len() {
        let (previous, _) = input.split_at(i);
        let (_, last_five) = previous.split_at(i - preamble);
        let sums = calculate_sums(last_five);

        let v = input[i];

        if !sums.contains(&v) {
            not_in.insert(v);
        }
    }

    *not_in.iter().next().unwrap()
}

///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_09::*;
/// let input = include_str!("../input/2020/day9.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 21806024);
/// ```
#[aoc(day9, part1)]
pub fn solve_part_01(input: &[u64]) -> u64 {
    find_weakness(input, 25)
}

///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_09::*;
/// let input = include_str!("../input/2020/day9.txt");
/// assert_eq!(solve_part_02(&input_generator(input)), 1539);
/// ```
#[aoc(day9, part2)]
pub fn solve_part_02(input: &[u64]) -> u64 {
    let weakness = find_weakness(input, 25);

    println!("{:?}", weakness);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn find_weakness_for_sample() {
        let data = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

        assert_eq!(find_weakness(&input_generator(data), 5), 127)
    }

    /// Test example data on part r
    #[test]
    fn sample_02() {
        let data = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

        assert_eq!(solve_part_02(&input_generator(data)), 62)
    }
}
