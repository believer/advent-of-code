//! Day 15: Lens Library
//!
//! The first part was a quick one using the as_bytes() method on strings.
//! The second part was a bit trickier, but not too bad. The steps were
//! simple, but mutating the HashMap and the inner vec was the tricky part.
//!
//! Refactored part 2 to use one list for the boxes and a HashMap for the
//! current focal lengths. This made it easier, and faster, to mutate the boxes and
//! calculate the focusing power.

use std::collections::HashMap;

pub struct Input {
    steps: Vec<String>,
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Input {
    Input {
        steps: input
            .split(',')
            .map(|x| x.to_string().replace('\n', ""))
            .collect(),
    }
}

fn hash(steps: &str) -> usize {
    let mut current_value = 0;

    for c in steps.as_bytes().iter() {
        current_value += *c as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

/* Part One
*
* Get the ASCII value of each character in the sequence and add it to 0.
* Then multiply the result by 17 and get the remainder of 256.
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_15::*;
let data = include_str!("../input/2023/day15.txt");
assert_eq!(solve_part_01(&input_generator(data)), 516070);
```"#]
#[aoc(day15, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    input.steps.iter().map(|step| hash(step)).sum()
}

/* Part Two
*
* Using the same hash function, identify the box number and depending on
* the operation, add or remove the sequence from the box.
*
* Then get the "focusing power" by multiplying the box number + 1 with the
* sequence number + 1 and the operation.
*
*/
#[doc = r#"```
use advent_of_code_2023::day_15::*;
let data = include_str!("../input/2023/day15.txt");
assert_eq!(solve_part_02(&input_generator(data)), 244981);
```"#]
#[aoc(day15, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let mut boxes = vec![Vec::new(); 256];
    let mut focal_lengths: HashMap<&str, usize> = HashMap::new();

    for step in input.steps.iter() {
        // Add operation
        if step.contains('=') {
            let (label, focal_length) = step.split_once('=').unwrap();
            let focal_length = focal_length.parse::<usize>().unwrap();
            let key = hash(label);

            focal_lengths.insert(label, focal_length);

            if !boxes[key].contains(&label) {
                boxes[key].push(label);
            }
        // Remove operation
        } else {
            let label = step[..step.len() - 1].to_string();
            let key = hash(&label);

            boxes[key].retain(|x| *x != label);
        }
    }

    let mut sum = 0;

    for (box_number, labels) in boxes.iter().enumerate() {
        for (lens_slot, label) in labels.iter().enumerate() {
            let focal_length = focal_lengths.get(label).unwrap();

            sum += (box_number + 1) * (lens_slot + 1) * focal_length;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    fn hasher(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(hash(input), expected);
    }

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 1320)]
    fn sample_01(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 145)]
    fn sample_02(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_02(&input_generator(input)), expected);
    }
}
