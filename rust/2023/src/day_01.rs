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

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
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
    let mut sum = 0;

    for line in input.lines() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));

        let first_digit = digits.next().unwrap();
        let last_digit = digits.last().unwrap_or(first_digit);

        sum += first_digit * 10 + last_digit;
    }

    sum
}

#[rustfmt::skip]
const DIGITS_AS_TEXT: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

fn digit_to_number(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse::<u32>().unwrap(),
    }
}

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
    let mut sum = 0;

    for line in input.lines() {
        let mut first_position = None;
        let mut first_digit = None;
        let mut last_position = None;
        let mut last_digit = None;

        // We only need the first and the last digit
        // so we can look from the front and the back to find them.
        for digit in DIGITS_AS_TEXT {
            // Find first digit
            if let Some(position) = line.find(digit) {
                if position <= first_position.unwrap_or(position) {
                    first_position = Some(position);
                    first_digit = Some(digit);
                }
            }

            // Find last digit
            if let Some(position) = line.rfind(digit) {
                if position >= last_position.unwrap_or(position) {
                    last_position = Some(position);
                    last_digit = Some(digit);
                }
            }
        }

        let first = first_digit.map(digit_to_number).unwrap();
        let last = last_digit.map(digit_to_number).unwrap();

        sum += first * 10 + last;
    }

    sum
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
