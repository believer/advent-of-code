use crate::math;
use std::collections::HashMap;

// Day 8:

#[derive(Debug)]
pub struct Input {
    directions: String,
    instructions: HashMap<String, (String, String)>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let mut input = input.split("\n\n");

    let directions = input.next().unwrap();
    let instructions = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line = line.split(" = ");
            let start = line.next().unwrap().to_string();
            let map = line
                .next()
                .unwrap()
                .split_once(", ")
                .map(|(x, y)| (x.replace(['(', ')'], ""), y.replace([')', ')'], "")))
                .unwrap();

            (start, map)
        })
        .collect::<HashMap<String, (String, String)>>();

    Input {
        directions: directions.to_string(),
        instructions,
    }
}

/* Part One
*
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_08::*;
/// let data = include_str!("../input/2023/day8.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 22411);
/// ```
#[aoc(day8, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    let mut steps = 0;
    let mut location = "AAA".to_string();

    while location != "ZZZ" {
        for char in input.directions.chars() {
            match char {
                'R' => {
                    let (_, right) = input.instructions.get(&location).unwrap();
                    location = right.to_string();
                }
                'L' => {
                    let (left, _) = input.instructions.get(&location).unwrap();
                    location = left.to_string();
                }
                _ => unreachable!(),
            }
            steps += 1;
        }
    }

    steps
}

/* Part Two
*
*
*/
/*
/// ```
/// use advent_of_code_2023::day_08::*;
/// let data = include_str!("../input/2023/day8.txt");
/// assert_eq!(solve_part_02(&input_generator_part2(data)), 248909434);
/// ```
*/
#[aoc(day8, part2)]
pub fn solve_part_02(input: &Input) -> i64 {
    let mut all_steps = Vec::new();
    let start_locations = input
        .instructions
        .keys()
        .filter(|i| i.ends_with('A'))
        .cloned()
        .collect::<Vec<String>>();

    for start in start_locations {
        let mut current = start.clone();
        let mut index = 0;
        let mut current_direction = input.directions.chars().nth(index).unwrap();
        let mut steps = 0;

        while !current.ends_with('Z') {
            current = match current_direction {
                'R' => input.instructions.get(&current).unwrap().1.to_string(),
                'L' => input.instructions.get(&current).unwrap().0.to_string(),
                _ => unreachable!(),
            };

            index = (index + 1) % input.directions.len();
            current_direction = input.directions.chars().nth(index).unwrap();
            steps += 1;
        }

        all_steps.push(steps);
    }

    all_steps.iter().fold(1, |acc, s| math::lcm(acc, *s as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 2);
    }

    #[test]
    fn sample_02() {
        let data = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(solve_part_01(&input_generator(data)), 6);
    }

    #[test]
    fn sample_part_02() {
        let data = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(solve_part_02(&input_generator(data)), 6);
    }
}
