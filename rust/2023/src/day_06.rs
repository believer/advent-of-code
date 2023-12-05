// Day 6:

pub struct Input;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Input {
    println!("Input: {}", input);
    Input
}

/* Part One
*
*/
// Your puzzle answer was
/*
/// ```
/// use advent_of_code_2023::day_06::*;
/// let data = include_str!("../input/2023/day6.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 993500720);
/// ```
*/
#[aoc(day6, part1)]
pub fn solve_part_01(_input: &Input) -> u32 {
    0
}

/* Part Two
*
*/
/*
/// ```
/// use advent_of_code_2023::day_06::*;
/// let data = include_str!("../input/2023/day6.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 993500720);
/// ```
*/
#[aoc(day6, part2)]
pub fn solve_part_02(_input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 0)
    }
}
