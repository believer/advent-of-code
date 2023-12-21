// Day 1: Trebuchet?!
//
// The first day of the Advent of Code 2023 is here!
// The first part was pretty straightforward, but the second part
// was a bit more tricky. I couldn't find a way to do it in code,
// so I massaged the data in vim.
//
// Edit: I've created solutions that work for both parts. It made the solutions look
// "slower" than the first example, because more work is done in the solution functions.
// But, it's actually faster when you compare the total time, so I'll make this "initial solution"
// for performance.
//
// Edit 2023-12-21: I saw a really elegant solution to this problem yesterday.
// Since the word numbers only overlay with at most one character, we can replace
// them while remaining the first and last character. For example, "one" becomes "o1e"
// or "two" becomes "t2o". Then we can sum like in the first part.

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

fn sum_first_and_last_digit(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));

    let first_digit = digits.next().unwrap();
    let last_digit = digits.last().unwrap_or(first_digit);

    first_digit * 10 + last_digit
}

/* Part One
*
* We are given a list of numbers mixed with letters. We need to extract the FIRST and LAST number of
* each line as an integer, and sum them up.
*
* Example:
*
* 1abc2
* pqr3stu8vwx
* a1b2c3d4e5f
* treb7uchet
*
* Or, 12 + 38 + 15 + 77 = 82
*
* Note that the last line only has one number, so it is repeated.
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_01::*;
/// let data = include_str!("../input/2023/day1.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 54304);
/// ```
#[aoc(day1, part1)]
pub fn solve_part_01(input: &str) -> u32 {
    input.lines().map(sum_first_and_last_digit).sum()
}

const TRANSLATIONS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

/* Part Two
*
* In part two we find out that the strings can contain words that
* are numbers. One to nine needs to be replaced with the actual number.
*
* Example:
* two1nine
* eightwothree
* abcone2threexyz
* xtwone3four
* 4nineeightseven2
* zoneight234
* 7pqrstsixteen
*
* 29, 83, 13, 24, 42, 14, and 76
*
* Some of the numbers overlap, like eightwothree should be 823 when cleaned.
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_01::*;
/// let data = include_str!("../input/2023/day1.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 54418);
/// ```
#[aoc(day1, part2)]
pub fn solve_part_02(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // Replace the words with numbers, but keep the first and last character
            // For example, "one" becomes "o1e" or "two" becomes "t2o"
            TRANSLATIONS
                .iter()
                .fold(line.to_string(), |line, (from, to)| line.replace(from, to))
        })
        .map(|line| sum_first_and_last_digit(&line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(solve_part_01(&input_generator(data)), 142)
    }

    #[test]
    fn sample_02() {
        let data = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve_part_02(&input_generator(data)), 281)
    }

    #[test]
    fn handle_overlap() {
        let data = "dleightwolvbvmsggs9njseven5fivethreenine";

        assert_eq!(solve_part_02(&input_generator(data)), 89)
    }
}
