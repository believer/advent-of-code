use std::collections::HashSet;

// Day 6 - Tuning Trouble
//
// I quickly remembered the solution from day 1 last year, which I blogged about at
// https://willcodefor.beer/posts/aoc221, that used windows. Turns out it was a perfect
// match for the problem.

type Input = Vec<char>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Input {
    input.chars().collect()
}

fn find_start_of_message_marker(input: &Input, window_size: usize) -> usize {
    // Find the first window that has all unique characters
    let start_of_marker = input
        .windows(window_size)
        .enumerate()
        .find(|(_, window)| window.iter().collect::<HashSet<_>>().len() == window.len())
        .unwrap()
        .0;

    // Add the window size to get the position where the first marker is completed
    start_of_marker + window_size
}

/* Part One
 *
 * We're looking for the first four-character window that has all unique characters.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_06::*;
/// let data = include_str!("../input/2022/day6.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 1300);
/// ```
#[aoc(day6, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    find_start_of_message_marker(input, 4)
}

/* Part Two
 *
 * The window size is now 14, but everything else is the same.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_06::*;
/// let data = include_str!("../input/2022/day6.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 3986);
/// ```
#[aoc(day6, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    find_start_of_message_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_mjqjpqmgbljsphdztnvjfqwrcgsmlb() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        assert_eq!(solve_part_01(&input_generator(data)), 7);
        assert_eq!(solve_part_02(&input_generator(data)), 19);
    }

    #[test]
    fn handles_bvwbjplbgvbhsrlpgdmjqwftvncz() {
        let data = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(solve_part_01(&input_generator(data)), 5);
        assert_eq!(solve_part_02(&input_generator(data)), 23);
    }

    #[test]
    fn handles_nppdvjthqldpwncqszvftbrmjlhg() {
        let data = "nppdvjthqldpwncqszvftbrmjlhg";

        assert_eq!(solve_part_01(&input_generator(data)), 6);
        assert_eq!(solve_part_02(&input_generator(data)), 23);
    }

    #[test]
    fn handles_nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg() {
        let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!(solve_part_01(&input_generator(data)), 10);
        assert_eq!(solve_part_02(&input_generator(data)), 29);
    }

    #[test]
    fn handles_zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw() {
        let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(solve_part_01(&input_generator(data)), 11);
        assert_eq!(solve_part_02(&input_generator(data)), 26);
    }
}
