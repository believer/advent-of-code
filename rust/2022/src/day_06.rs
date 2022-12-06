use std::collections::HashSet;

// Day 6 - Tuning Trouble

type Input = Vec<char>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Input {
    input.chars().collect()
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_06::*;
/// let data = include_str!("../input/2022/day6.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 0);
/// ```
#[aoc(day6, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    let mut pos = 0;

    for (i, c) in input.windows(4).enumerate() {
        let set: HashSet<_> = HashSet::from_iter(c.iter().cloned());

        if set.len() == c.len() {
            pos = i + c.len();
            break;
        }
    }

    pos
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_06::*;
/// let data = include_str!("../input/2022/day6.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 0);
/// ```
#[aoc(day6, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let mut pos = 0;

    for (i, c) in input.windows(14).enumerate() {
        let set: HashSet<_> = HashSet::from_iter(c.iter().cloned());

        if set.len() == c.len() {
            pos = i + c.len();
            break;
        }
    }

    pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let sample_01 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let sample_02 = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(solve_part_01(&input_generator(sample_01)), 7);
        assert_eq!(solve_part_01(&input_generator(sample_02)), 5);
    }

    #[test]
    fn sample_02() {
        let sample_01 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let sample_02 = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(solve_part_02(&input_generator(sample_01)), 19);
        assert_eq!(solve_part_02(&input_generator(sample_02)), 23);
    }
}
