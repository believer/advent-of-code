use std::collections::{HashMap, HashSet};

// Day 3 -

type Input = Vec<(String, String)>;
type InputPartTwo = Vec<String>;

#[aoc_generator(day3, part1)]
pub fn input_generator_part_1(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let line_length = l.len();

            let (first, second) = l.split_at(line_length / 2);
            (first.to_string(), second.to_string())
        })
        .collect()
}

#[aoc_generator(day3, part2)]
pub fn input_generator_part_2(input: &str) -> InputPartTwo {
    input.lines().map(|l| l.to_string()).collect()
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_03::*;
/// let data = include_str!("../input/2022/day3.txt");
/// assert_eq!(solve_part_01(&input_generator_part_1(data)), 7831);
/// ```
#[aoc(day3, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    let mut sum = 0;
    let mut priorities: HashMap<char, u32> = HashMap::new();

    let lowercase = (10..36)
        .map(|i| char::from_digit(i, 36).unwrap())
        .collect::<Vec<_>>();
    let alphabet = lowercase
        .iter()
        .map(|c| c.to_uppercase().next().unwrap())
        .collect::<Vec<_>>();

    let mut i = 1;

    for letter in lowercase {
        priorities.insert(letter, i);
        i += 1;
    }

    for letter in alphabet {
        priorities.insert(letter, i);
        i += 1;
    }

    for (first, second) in input {
        let compartment: HashSet<char> = first.chars().collect();

        second.chars().any(|c| {
            if compartment.contains(&c) {
                sum += priorities.get(&c).unwrap();
                true
            } else {
                false
            }
        });
    }

    sum
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_03::*;
/// let data = include_str!("../input/2022/day3.txt");
/// assert_eq!(solve_part_02(&input_generator_part_2(data)), 2683);
/// ```
#[aoc(day3, part2)]
pub fn solve_part_02(input: &InputPartTwo) -> u32 {
    let mut sum = 0;
    let mut priorities: HashMap<char, u32> = HashMap::new();

    let lowercase = (10..36)
        .map(|i| char::from_digit(i, 36).unwrap())
        .collect::<Vec<_>>();
    let alphabet = lowercase
        .iter()
        .map(|c| c.to_uppercase().next().unwrap())
        .collect::<Vec<_>>();

    let mut i = 1;

    for letter in lowercase {
        priorities.insert(letter, i);
        i += 1;
    }

    for letter in alphabet {
        priorities.insert(letter, i);
        i += 1;
    }

    for group in input.chunks(3) {
        let mut candidates: HashSet<char> = group.first().unwrap().chars().collect();
        let second: HashSet<char> = group.get(1).unwrap().chars().collect();
        let third: HashSet<char> = group.get(2).unwrap().chars().collect();

        candidates.retain(|c| second.contains(c) && third.contains(c));

        sum += candidates
            .iter()
            .map(|c| priorities.get(c).unwrap())
            .sum::<u32>();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(solve_part_01(&input_generator_part_1(data)), 157)
    }

    #[test]
    fn sample_02() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(solve_part_02(&input_generator_part_2(data)), 70)
    }
}
