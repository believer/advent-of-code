// Day 1 - Calorie Counting
//
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
 * The jungle must be too overgrown and difficult to navigate in
 * vehicles or access from the air; the Elves' expedition traditionally
 * goes on foot. As your boats approach land, the Elves begin taking
 * inventory of their supplies. One important consideration is
 * food - in particular, the number of Calories each Elf is carrying (your puzzle input).
 *
 * The Elves take turns writing down the number of Calories contained
 * by the various meals, snacks, rations, etc. that they've brought with
 * them, one item per line. Each Elf separates their own inventory from
 * the previous Elf's inventory (if any) by a blank line.
 *
 * For example, suppose the Elves finish writing their items'
 * Calories and end up with the following list:
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
 * 7000
 * 8000
 * 9000
 *
 * 10000
 * This list represents the Calories of the food carried by five Elves:
 *
 * The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
 * The second Elf is carrying one food item with 4000 Calories.
 * The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
 * The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
 * The fifth Elf is carrying one food item with 10000 Calories.
 *
 * In case the Elves get hungry and need extra snacks, they need to
 * know which Elf to ask: they'd like to know how many Calories are
 * being carried by the Elf carrying the most Calories. In the example
 * above, this is 24000 (carried by the fourth Elf).
 *
 * Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_01::*;
/// let data = include_str!("../input/2022/day1.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 69528);
/// ```
#[aoc(day1, part1)]
pub fn solve_part_01(elves: &CaloriesPerElf) -> u32 {
    // Get the Elf carrying the most calories of snacks
    *elves.first().unwrap()
}

/* Part Two
 *
 * By the time you calculate the answer to the Elves' question,
 * they've already realized that the Elf carrying the most Calories
 * of food might eventually run out of snacks.
 *
 * To avoid this unacceptable situation, the Elves would instead
 * like to know the total Calories carried by the top three Elves
 * carrying the most Calories. That way, even if one of those Elves
 * runs out of snacks, they still have two backups.
 *
 * In the example above, the top three Elves are the fourth Elf (with 24000 Calories),
 * then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories).
 * The sum of the Calories carried by these three elves is 45000.
 *
 * Find the top three Elves carrying the most Calories.
 * How many Calories are those Elves carrying in total?
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_01::*;
/// let data = include_str!("../input/2022/day1.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 206152);
/// ```
#[aoc(day1, part2)]
pub fn solve_part_02(elves: &CaloriesPerElf) -> u32 {
    // Get the amount of calories carried by the top three Elves and sum it
    elves.iter().take(3).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(solve_part_01(&input_generator(data)), 24000)
    }

    #[test]
    fn sample_02() {
        let data = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(solve_part_02(&input_generator(data)), 45000)
    }
}
