use crate::common;

// Day 9 - Encoding Error

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    common::input_vec(input)
}

// This solution is so much faster than my initial version. The first
// solution is linked in the previous performance results table in th README.
// Courtesy of @chsiedentop
// https://twitter.com/chsiedentop/status/1336947802887180290
fn find_broken_number(input: &[u64], preamble: usize) -> Option<u64> {
    let mut sums = vec![];
    let input_without_preamble = input.len() - preamble;

    // Create all sums
    for i in 0..input_without_preamble {
        for j in 1..preamble {
            sums.push(input[i] + input[i + j])
        }
    }

    let window_size = preamble * (preamble - 1);

    for i in 0..input_without_preamble {
        let target = input[i + preamble];
        let start = (preamble - 1) * i;
        let end = start + window_size;

        if let false = sums[start..end].iter().any(|p| *p == target) {
            return Some(target);
        }
    }

    None
}

fn find_weakness(input: &[u64], preamble: usize) -> Option<u64> {
    let broken = find_broken_number(input, preamble).unwrap();
    let mut acc = 0;
    let mut sums = vec![0];

    // Find all sums
    for v in input {
        acc += v;
        sums.push(acc);
    }

    // Find the sums that match our weakness
    for i in 0..sums.len() {
        let mut j = i + 2;

        while j < sums.len() && sums[j] - sums[i] <= broken {
            if sums[j] - sums[i] == broken {
                let max = input[i..j].iter().max().unwrap();
                let min = input[i..j].iter().min().unwrap();

                return Some(*max + *min);
            }

            j += 1;
        }
    }

    None
}

/* Part One
 *
 * With your neighbor happily enjoying their video game,
 * you turn your attention to an open data port on the little screen in the seat in front of you.
 *
 * Though the port is non-standard, you manage to connect it to your computer through
 * the clever use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle input).
 *
 * The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which,
 * conveniently for you, is an old cypher with an important weakness.
 *
 * XMAS starts by transmitting a preamble of 25 numbers. After that, each number you
 * receive should be the sum of any two of the 25 immediately previous numbers.
 * The two numbers will have different values, and there might be more than one such pair.
 *
 * For example, suppose your preamble consists of the numbers 1 through 25 in a random order.
 * To be valid, the next number must be the sum of two of those numbers:
 *
 * 26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
 * 49 would be a valid next number, as it is the sum of 24 and 25.
 * 100 would not be valid; no two of the previous 25 numbers sum to 100.
 * 50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be different.
 * Suppose the 26th number is 45, and the first number (no longer an option, as it is more than 25 numbers ago)
 * was 20. Now, for the next number to be valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that add up to it:
 *
 * 26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
 * 65 would not be valid, as no two of the available numbers sum to it.
 * 64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.
 * Here is a larger example which only considers the previous 5 numbers (and has a preamble of length 5):
 *
 * 35
 * 20
 * 15
 * 25
 * 47
 * 40
 * 62
 * 55
 * 65
 * 95
 * 102
 * 117
 * 150
 * 182
 * 127
 * 219
 * 299
 * 277
 * 309
 * 576
 *
 * In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers; the only number that does not follow this rule is 127.
 *
 * The first step of attacking the weakness in the XMAS data is to find the first number in the
 * list (after the preamble) which is not the sum of two of the 25 numbers before it.
 *
 * What is the first number that does not have this property?
*/
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_09::*;
/// let input = include_str!("../input/2020/day9.txt");
/// assert_eq!(solve_part_01(&input_generator(input)).unwrap(), 21806024);
/// ```
#[aoc(day9, part1)]
pub fn solve_part_01(input: &[u64]) -> Option<u64> {
    find_broken_number(input, 25)
}

///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_09::*;
/// let input = include_str!("../input/2020/day9.txt");
/// assert_eq!(solve_part_02(&input_generator(input)).unwrap(), 2986195);
/// ```
#[aoc(day9, part2)]
pub fn solve_part_02(input: &[u64]) -> Option<u64> {
    find_weakness(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn find_broken_number_for_sample() {
        let data = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

        assert_eq!(find_broken_number(&input_generator(data), 5).unwrap(), 127)
    }

    /// Test example data on part 2
    #[test]
    fn find_weakness_for_sample() {
        let data = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

        assert_eq!(find_weakness(&input_generator(data), 5).unwrap(), 62)
    }
}
