// Day 5

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

/* Part One
*
*/
// Your puzzle answer was
/*
/// ```
/// use advent_of_code_2023::day_05::*;
/// let data = include_str!("../input/2023/day5.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 54304);
/// ```
*/
#[aoc(day5, part1)]
pub fn solve_part_01(_input: &str) -> u32 {
    0
}

/* Part Two
*
*/
// Your puzzle answer was
/*
/// ```
/// use advent_of_code_2023::day_05::*;
/// let data = include_str!("../input/2023/day5.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 54418);
/// ```
*/
#[aoc(day5, part2)]
pub fn solve_part_02(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "";

        assert_eq!(solve_part_01(&input_generator(data)), 0)
    }
}
