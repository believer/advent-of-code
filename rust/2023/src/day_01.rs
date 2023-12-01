use regex::Regex;

// Day 1: Trebuchet?!
//
// The first day of the Advent of Code 2023 is here!
// The first part was pretty straightforward, but the second part
// was a bit more tricky. I couldn't find a way to do it in code,
// so I massaged the data in vim.

#[aoc_generator(day1, part1)]
pub fn input_generator_part1(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .map(|n| {
            // First and last digits of the number
            let t = n.to_string();
            let i = t.chars().next().unwrap();
            let i2 = t.chars().next_back().unwrap();

            i.to_string() + &i2.to_string()
        })
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

#[aoc_generator(day1, part2)]
pub fn input_generator_part2(input: &str) -> Vec<u32> {
    let mut data: Vec<u32> = vec![];

    for line in input.lines() {
        let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let cleaned_data = re.replace_all(line, |caps: &regex::Captures| match &caps[0] {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => unreachable!(),
        });

        let filtered_data = cleaned_data
            .split_whitespace()
            .map(|l| {
                l.chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
            .map(|n| {
                // First and last digits of the number
                let t = n.to_string();
                let i = t.chars().next().unwrap();
                let i2 = t.chars().next_back().unwrap();
                i.to_string() + &i2.to_string()
            })
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>()[0];

        data.push(filtered_data);
    }

    data
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
/// assert_eq!(solve_part_01(&input_generator_part1(data)), 54304);
/// ```
#[aoc(day1, part1)]
pub fn solve_part_01(input: &[u32]) -> u32 {
    input.iter().sum()
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
/* DOCTEST DISABLED
/// ```
/// use advent_of_code_2023::day_01::*;
/// let data = include_str!("../input/2023/day1.txt");
/// assert_eq!(solve_part_02(&input_generator_part2(data)), 54418);
/// ```
*/
#[aoc(day1, part2)]
pub fn solve_part_02(input: &[u32]) -> u32 {
    input.iter().sum()
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

        assert_eq!(solve_part_01(&input_generator_part1(data)), 142)
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

        assert_eq!(solve_part_02(&input_generator_part2(data)), 281)
    }

    #[test]
    fn handle_overlap() {
        let data = "dleightwolvbvmsggs9njseven5fivethreenine";

        assert_eq!(solve_part_02(&input_generator_part2(data)), 89)
    }
}
