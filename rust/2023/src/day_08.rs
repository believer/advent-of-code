use crate::math;
use std::collections::HashMap;

// Day 8: Haunted Wasteland
//
// The first part was pretty straight forward, but I tried doing the calculations
// the same way for the second part, and it wouldn't finish. I wanted to inspect
// the values, but it crashed my terminal (one more than one occasion...).
//
// I knew that there had to be some trick to it. After looking at the step by step
// description of the second part, I realized that once we had reached a XXZ location,
// it would just loop. This meant that we can calculate the steps for each
// start location, and then find the least common multiple of all the steps.
// The answer turned out to be _huge_, so no real way of brute forcing it.

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Input {
    directions: Vec<Direction>,
    instructions: HashMap<String, (String, String)>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let input = input.split("\n\n").collect::<Vec<_>>();

    let directions = input[0].chars().map(Direction::from).collect::<Vec<_>>();
    let instructions = input[1]
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
        directions,
        instructions,
    }
}

/* Part One
*
* We get a list of directions, and a list of instructions.
* Steps through the directions, and select the correct instruction (left or right).
* Keep track of the steps, and return the number of steps when we reach the ZZZ location.
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_08::*;
let data = include_str!("../input/2023/day8.txt");
assert_eq!(solve_part_01(&input_generator(data)), 22411);
```"#]
#[aoc(day8, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    let mut steps = 0;
    let mut location = "AAA".to_string();

    while location != "ZZZ" {
        for char in &input.directions {
            location = match char {
                Direction::Right => input.instructions.get(&location).unwrap().1.to_string(),
                Direction::Left => input.instructions.get(&location).unwrap().0.to_string(),
            };

            steps += 1;
        }
    }

    steps
}

/* Part Two
*
* The input is the same, but we treat every location that ends
* with an A as a start location. We then need to calculate the
* number of steps it takes for _all of them_ to reach a location
* that ends with a Z.
*
*/
#[doc = r#"```
use advent_of_code_2023::day_08::*;
let data = include_str!("../input/2023/day8.txt");
assert_eq!(solve_part_02(&input_generator(data)), 11188774513823);
```"#]
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
        let mut current_direction = input.directions.get(index).unwrap();
        let mut steps: i64 = 0;

        while !current.ends_with('Z') {
            current = match current_direction {
                Direction::Right => input.instructions.get(&current).unwrap().1.to_string(),
                Direction::Left => input.instructions.get(&current).unwrap().0.to_string(),
            };

            index = (index + 1) % input.directions.len();
            current_direction = input.directions.get(index).unwrap();
            steps += 1;
        }

        all_steps.push(steps);
    }

    all_steps.iter().fold(1, |acc, s| math::lcm(acc, *s))
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
