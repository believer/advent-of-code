use crate::common;
use std::collections::HashSet;

// Day 1 - Report Repair
// A version of "Two Sum Problem"
// https://leetcode.com/problems/two-sum/solution/
//
// I did the naive solutions first which are O(n^2) and O(n^3).
// It was fast enough to get a quick answer. I then refactored
// using a HashSet which gets it down to O(n) and O(n^2) and it
// could be seen in the performance testing.

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> HashSet<u32> {
    common::input_hashset(input)
}

/* Part One
 *
 * Before you leave, the Elves in accounting just need you to fix your
 * expense report (your puzzle input); apparently, something isn't quite adding up.
 *
 * Specifically, they need you to find the two entries that sum to
 * `2020` and then multiply those two numbers together.
 *
 * For example, suppose your expense report contained the following:
 *
 * 1721
 * 979
 * 366
 * 299
 * 675
 * 1456
 *
 * In this list, the two entries that sum to 2020 are 1721 and 299.
 * Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
 *
 * Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
*/
#[aoc(day1, part1)]
pub fn solve_part_01(input: &HashSet<u32>) -> u32 {
    for x in input {
        let y = 2020 - x;
        if input.contains(&y) {
            return x * y;
        }
    }

    0
}

/* Part Two
 *
 * The Elves in accounting are thankful for your help; one of them even offers you
 * a starfish coin they had left over from a past vacation. They offer you a second
 * one if you can find three numbers in your expense report that meet the same criteria.
 *
 * Using the above example again, the three entries that sum to 2020 are 979, 366, and 675.
 * Multiplying them together produces the answer, 241861950.
 *
 * In your expense report, what is the product of the three entries that sum to 2020?
*/
#[aoc(day1, part2)]
pub fn solve_part_02(input: &HashSet<u32>) -> u32 {
    for x in input {
        for y in input {
            if x + y > 2020 {
                continue;
            }

            let z = 2020 - x - y;

            if input.contains(&z) {
                return x * y * z;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
1721
979
366
299
675
1456
";

        assert_eq!(solve_part_01(&input_generator(data)), 514579)
    }

    #[test]
    fn sample_02() {
        let data = "
1721
979
366
299
675
1456
";

        assert_eq!(solve_part_02(&input_generator(data)), 241861950)
    }
}
