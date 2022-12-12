// Day 1 - Calorie Counting

// Clippy suggested changing all.sort() to all.sort_unstable().
// It performs faster when sorting a primitive type without any
// difference in the result. It made the solution > 37% faster.
//
// I tried using a Peekable for part two, but it made the solution slower.
// So, I'll keep the faster/naive solution.
//
// After having stressed down during the day, I remembered that I can split by
// double line breaks and handle each Elf separetely. This made the solution a whopping
// 99% faster. For both parts!

type CaloriesPerElf = Vec<u32>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> CaloriesPerElf {
    // Split by double space, i.e., each Elf
    let elves: Vec<Vec<u32>> = input
        .split("\n\n")
        .map(|elf| {
            // Parse the Elf's item calories to u32
            elf.lines()
                .filter(|l| !l.is_empty())
                .map(|item| item.parse().unwrap())
                .collect()
        })
        .collect();

    // Sum up the calories carried by each Elf
    let mut calories_per_elf: Vec<u32> = elves.iter().map(|elf| elf.iter().sum()).collect();

    // Sort unstable is faster than sort. See comment in header.
    calories_per_elf.sort_unstable();
    calories_per_elf.reverse();

    calories_per_elf
}

/* Part One
 *
 * We get a list of elves, separated by a double line-break. Each elf carries a
 * couple of items and each line corresponds to that item's calories. For example:
 *
 * 1000
 * 2000
 * 3000
 *
 * 4000
 *
 * 5000
 * 6000
 *
 * We want to find the elf that is carrying the most calories. Sum up the calories
 * for each elf and find the largest number.
 *
 * From the example above, the calories for each elf are 6000, 4000, 11000. So, 11000
 * would be our answer.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_01::*;
/// let data = include_str!("../input/2022/day1.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 69528);
/// ```
#[aoc(day1, part1)]
pub fn solve_part_01(elves: &CaloriesPerElf) -> u32 {
    *elves.first().unwrap()
}

/* Part Two
 *
 * In part two we want to find the top three elves and sum up the
 * amount of calories they are carrying.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_01::*;
/// let data = include_str!("../input/2022/day1.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 206152);
/// ```
#[aoc(day1, part2)]
pub fn solve_part_02(elves: &CaloriesPerElf) -> u32 {
    elves.iter().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 24000)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 45000)
    }
}
