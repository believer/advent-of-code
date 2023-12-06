// Day 7:

#[derive(Debug)]
pub struct Input {}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Input {
    println!("{input}");
    Input {}
}

/* Part One
*
*/
// Your puzzle answer was
/*
/// ```
/// use advent_of_code_2023::day_07::*;
/// let data = include_str!("../input/2023/day7.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 861300);
/// ```
*/
#[aoc(day7, part1)]
pub fn solve_part_01(_input: &Input) -> u64 {
    0
}

/* Part Two
*
* Here we combine all the race times and distance from the previous data into
* two large numbers. This means we only have one race, but since the time
* is longer, we have a lot more ways to beat the record.
*
*/
/*
/// ```
/// use advent_of_code_2023::day_07::*;
/// let data = include_str!("../input/2023/day7.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 28101347);
/// ```
*/
#[aoc(day7, part2)]
pub fn solve_part_02(_input: &Input) -> u64 {
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
