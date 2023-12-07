// Day 8:

#[derive(Debug)]
pub struct Input {
    data: u64,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    println!("{}", input);
    Input { data: 0 }
}

/* Part One
*
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_08::*;
/// let data = include_str!("../input/2023/day8.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 250474325);
/// ```
#[aoc(day8, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    input.data
}

/* Part Two
*
*
*/
/// ```
/// use advent_of_code_2023::day_08::*;
/// let data = include_str!("../input/2023/day8.txt");
/// assert_eq!(solve_part_02(&input_generator_part2(data)), 248909434);
/// ```
#[aoc(day8, part2)]
pub fn solve_part_02(_input: &Input) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 0);
    }
}
