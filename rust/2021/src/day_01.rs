use crate::common;

// Day 1: TBD

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    common::input_vec(input)
}

/* Part One
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2021::day_01::*;
/// let data = include_str!("../input/2021/day1.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 0);
/// ```
#[aoc(day1, part1)]
pub fn solve_part_01(input: &Vec<u32>) -> u32 {
    let mut increases = 0;
    let mut last_value = input[0];

    for i in input {
        if i > &last_value {
            increases += 1
        }

        last_value = *i
    }

    increases
}

/* Part Two
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2021::day_01::*;
/// let data = include_str!("../input/2021/day1.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 1713);
/// ```
#[aoc(day1, part2)]
pub fn solve_part_02(_input: &Vec<u32>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(solve_part_01(&input_generator(data)), 7)
    }
}
