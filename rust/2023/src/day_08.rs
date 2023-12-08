use std::collections::HashMap;

// Day 8:

#[derive(Debug)]
pub struct Input {
    right_left: String,
    instructions: HashMap<String, (String, String)>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let mut input = input.split("\n\n");

    let right_left = input.next().unwrap();
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
        right_left: right_left.to_string(),
        instructions,
    }
}

/* Part One
*
*
*/
// Your puzzle answer was
/*
/// ```
/// use advent_of_code_2023::day_08::*;
/// let data = include_str!("../input/2023/day8.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 250474325);
/// ```
*/
#[aoc(day8, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    let mut steps = 0;
    let mut location = "AAA".to_string();

    while location != "ZZZ" {
        for char in input.right_left.chars() {
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
pub fn solve_part_02(_input: &Input) -> u64 {
    0
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

    fn sample_02() {
        let data = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(solve_part_02(&input_generator(data)), 6);
    }
}
