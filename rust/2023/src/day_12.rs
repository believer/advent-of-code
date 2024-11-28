//! Day 12: Hort Springs
//!
//! I didn't really have an idea how to solve this problem. I thought that it might be possible to
//! start looking from the end of the pattern somehow. After some reasoning with ChatGPT about
//! the problem it gave me something called "Dynamic Programming" where you break
//! the problem into smaller subproblems and then solve them.
//!
//! So, this solution was an iterative process of trying to understand and implementing
//! the algorithm.
//!
//! I later tested a solutions using recursion and memoization. It was a bit easier to
//! understand, but it was also slower.

use itertools::Itertools;

pub struct Input {
    data: Vec<(String, Vec<usize>)>,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    let data = input
        .lines()
        .map(|line| {
            let (springs, damaged) = line.split_once(' ').expect("Invalid input");
            let damaged = damaged
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (springs.to_string(), damaged)
        })
        .collect::<Vec<_>>();

    Input { data }
}

pub fn possible_arrangements(springs: &[u8], damaged: &[usize]) -> usize {
    // Setup the DP table
    let num_springs = springs.len();
    let num_damaged = damaged.len();
    let mut dp = vec![vec![vec![0; num_springs + 1]; num_damaged + 1]; num_springs + 1];

    // Base values
    dp[num_springs][num_damaged][0] = 1;
    dp[num_springs][num_damaged - 1][damaged[num_damaged - 1]] = 1;

    // Go through the conditions backwards
    for position in (0..num_springs).rev() {
        for (group, &j) in damaged.iter().enumerate() {
            for count in 0..=j {
                for &c in &[b'.', b'#'] {
                    // If the current character is not a '?' or the character type
                    // we're looking for, then we can skip it.
                    if ![c, b'?'].contains(&springs[position]) {
                        continue;
                    }

                    let p = position + 1;

                    let value = match (c, count) {
                        (b'.', 0) => dp[p][group][0],
                        (b'.', c) if damaged[group] == c => dp[p][group + 1][0],
                        (b'#', _) => dp[p][group][count + 1],
                        _ => 0,
                    };

                    dp[position][group][count] += value;
                }
            }
        }

        // If the current character is a '?' or '.', then we can also add the value from the next
        if matches!(springs[position], b'?' | b'.') {
            dp[position][num_damaged][0] += dp[position + 1][num_damaged][0];
        }
    }

    // The problem is solved backwards, so the answer will be in the first cell
    dp[0][0][0]
}

/* Part One
*
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_12::*;
let data = include_str!("../input/2023/day12.txt");
assert_eq!(solve_part_01(&input_generator(data)), 8270);
```"#]
#[aoc(day12, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    input
        .data
        .iter()
        .map(|(springs, damaged)| possible_arrangements(springs.as_bytes(), damaged))
        .sum::<usize>()
}

/* Part Two
*
*
*/
#[doc = r#"```
use advent_of_code_2023::day_12::*;
let data = include_str!("../input/2023/day12.txt");
assert_eq!(solve_part_02(&input_generator(data)), 204640299929836);
```"#]
#[aoc(day12, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    input
        .data
        .iter()
        .map(|(conditions, damaged)| {
            // Repeat the conditions and list of strings 5 times. Separate the patterns with '?'.
            let springs = (0..5).map(|_| conditions).join("?");
            let damaged = damaged.repeat(5);

            possible_arrangements(springs.as_bytes(), &damaged)
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        21
    )]
    fn sample_01(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
        525152
    )]
    fn sample_02(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_02(&input_generator(input)), expected);
    }
}
