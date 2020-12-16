use std::collections::hash_map::Entry;
use std::collections::HashMap;

// Day 15 - Rambunctious Recitation
//
// This is The Van Eck Sequence
// https://www.youtube.com/watch?v=etMJxB-igrc
//
// I had multiple lookups in the HashMap which slowed down my first solution
// significantly. Got a tip from Reddit to use the Entry API, this way I could
// reduce the HashMap to only contain the last value. It also made the solution a
// whole lot faster.

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn solver(input: &[usize], end_on_turn: usize) -> usize {
    let mut seen: HashMap<_, _> = input[..input.len() - 1]
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect();

    (input.len()..end_on_turn).fold(*input.last().unwrap(), |n, i| match seen.entry(n) {
        Entry::Occupied(mut occ) => i - occ.insert(i - 1) - 1,
        Entry::Vacant(vac) => {
            vac.insert(i - 1);
            0
        }
    })
}

/* Part One
 *
 * You catch the airport shuttle and try to book a new flight to your vacation island.
 * Due to the storm, all direct flights have been cancelled,
 * but a route is available to get around the storm. You take it.
 *
 * While you wait for your flight, you decide to check in with the Elves back at the North Pole.
 * They're playing a memory game and are ever so excited to explain the rules!
 *
 * In this game, the players take turns saying numbers.
 * They begin by taking turns reading from a list of starting numbers (your puzzle input).
 * Then, each turn consists of considering the most recently spoken number:
 *
 * If that was the first time the number has been spoken, the current player says 0.
 * Otherwise, the number had been spoken before; the current player announces how
 * many turns apart the number is from when it was previously spoken.
 * So, after the starting numbers, each turn results in that player speaking
 * aloud either 0 (if the last number is new) or an age (if the last number is a repeat).
 *
 * For example, suppose the starting numbers are 0,3,6:
 *
 * Turn 1: The 1st number spoken is a starting number, 0.
 * Turn 2: The 2nd number spoken is a starting number, 3.
 * Turn 3: The 3rd number spoken is a starting number, 6.
 * Turn 4: Now, consider the last number spoken, 6.
 * Since that was the first time the number had been spoken, the 4th number spoken is 0.
 * Turn 5: Next, again consider the last number spoken, 0.
 * Since it had been spoken before, the next number to speak is the difference
 * between the turn number when it was last spoken (the previous turn, 4) and the
 * turn number of the time it was most recently spoken before then (turn 1).
 * Thus, the 5th number spoken is 4 - 1, 3.
 * Turn 6: The last number spoken, 3 had also been spoken before, most recently on
 * turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
 * Turn 7: Since 3 was just spoken twice in a row, and the last two turns
 * are 1 turn apart, the 7th number spoken is 1.
 * Turn 8: Since 1 is new, the 8th number spoken is 0.
 * Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between them, 4.
 * Turn 10: 4 is new, so the 10th number spoken is 0.
 * (The game ends when the Elves get sick of playing or dinner is ready, whichever comes first.)
 *
 * Their question for you is: what will be the 2020th number spoken?
 * In the example above, the 2020th number spoken will be 436.
 *
 * Here are a few more examples:
 *
 * Given the starting numbers 1,3,2, the 2020th number spoken is 1.
 * Given the starting numbers 2,1,3, the 2020th number spoken is 10.
 * Given the starting numbers 1,2,3, the 2020th number spoken is 27.
 * Given the starting numbers 2,3,1, the 2020th number spoken is 78.
 * Given the starting numbers 3,2,1, the 2020th number spoken is 438.
 * Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
 * Given your starting numbers, what will be the 2020th number spoken?
 */
/// Your puzzle answer was
/// ```
/// use advent_of_code_2020::day_15::*;
/// let input = include_str!("../input/2020/day15.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 1696);
/// ```
#[aoc(day15, part1)]
pub fn solve_part_01(input: &[usize]) -> usize {
    solver(&input, 2020)
}

/* Part Two
 *
 * Impressed, the Elves issue you a challenge: determine the 30000000th
 * number spoken. For example, given the same starting numbers as above:
 *
 * Given 0,3,6, the 30000000th number spoken is 175594.
 * Given 1,3,2, the 30000000th number spoken is 2578.
 * Given 2,1,3, the 30000000th number spoken is 3544142.
 * Given 1,2,3, the 30000000th number spoken is 261214.
 * Given 2,3,1, the 30000000th number spoken is 6895259.
 * Given 3,2,1, the 30000000th number spoken is 18.
 * Given 3,1,2, the 30000000th number spoken is 362.
 *
 * Given your starting numbers, what will be the 30000000th number spoken?
*/
// Ignored test, takes too long
// /// Your puzzle answer was
// /// ```
// /// use advent_of_code_2020::day_15::*;
// /// let input = include_str!("../input/2020/day15.txt");
// /// assert_eq!(solve_part_02(&input_generator(input)), 37385);
// /// ```
#[aoc(day15, part2)]
pub fn solve_part_02(input: &[usize]) -> usize {
    solver(&input, 30_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test example data on part 1
    #[test]
    fn test_example_part_1() {
        let data = "0,3,6";

        assert_eq!(solve_part_01(&input_generator(data)), 436)
    }

    // Test example data on part 1
    #[test]
    fn test_example_part_1_2() {
        let data = "1,3,2";

        assert_eq!(solve_part_01(&input_generator(data)), 1)
    }

    // Test example data on part 1
    #[test]
    fn test_example_part_1_3() {
        let data = "2,1,3";

        assert_eq!(solve_part_01(&input_generator(data)), 10)
    }

    // Test example data on part 1
    #[test]
    fn test_example_part_1_4() {
        let data = "1,2,3";

        assert_eq!(solve_part_01(&input_generator(data)), 27)
    }

    // Test example data on part 1
    #[test]
    fn test_example_part_1_5() {
        let data = "2,3,1";

        assert_eq!(solve_part_01(&input_generator(data)), 78)
    }

    // Test example data on part 1
    #[test]
    fn test_example_part_1_6() {
        let data = "3,2,1";

        assert_eq!(solve_part_01(&input_generator(data)), 438)
    }

    // Test example data on part 1
    #[test]
    fn test_example_part_1_7() {
        let data = "3,1,2";

        assert_eq!(solve_part_01(&input_generator(data)), 1836)
    }

    // This test is too slow
    // To run, use cargo test -- --ignored
    #[test]
    #[ignore]
    fn test_example_part_2() {
        let data = "0,3,6";

        assert_eq!(solve_part_02(&input_generator(data)), 175594)
    }
}
