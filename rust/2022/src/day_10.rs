use crate::common;

// Day 10 - Part 1

type Input = Vec<u32>;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    common::input_vec(input)
}

/* Part One
*/
// /// Your puzzle answer was
// /// ```
// /// use advent_of_code_2022::day_10::*;
// /// let data = include_str!("../input/2022/day10.txt");
// /// assert_eq!(solve_part_01(&input_generator(data)), 0);
// /// ```
#[aoc(day10, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    println!("{:?}", input);
    0
}

/* Part Two
*/
// /// Your puzzle answer was
// /// ```
// /// use advent_of_code_2022::day_10::*;
// /// let data = include_str!("../input/2022/day10.txt");
// /// assert_eq!(solve_part_02(&input_generator(data)), 0);
// /// ```
#[aoc(day10, part2)]
pub fn solve_part_02(_input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "0";

        assert_eq!(solve_part_01(&input_generator(data)), 0)
    }
}
