//! Day 15: Lens Library
//!
//! The first part was a quick one using the as_bytes() method on strings.
//! The second part was a bit trickier, but not too bad. The steps were
//! simple, but mutating the HashMap and the inner vec was the tricky part.

use std::collections::HashMap;

pub struct Input {
    init_sequence: Vec<Vec<u8>>,
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Input {
    Input {
        init_sequence: input.split(',').map(|x| x.as_bytes().to_vec()).collect(),
    }
}

fn hash(sequence: &[u8]) -> u32 {
    let mut current_value: u32 = 0;

    for c in sequence.iter() {
        current_value += *c as u32;
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
/* Doesn't work right now?
#[doc = r#"```
use advent_of_code_2023::day_15::*;
let data = include_str!("../input/2023/day15.txt");
assert_eq!(solve_part_01(&input_generator(data)), 516070);
```"#]
*/
#[aoc(day15, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    input
        .init_sequence
        .iter()
        .fold(0, |acc, sequence| acc + hash(sequence))
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
pub fn solve_part_02(input: &Input) -> u32 {
    let mut boxes: HashMap<u32, Vec<String>> = HashMap::with_capacity(256);

    for sequence in input.init_sequence.iter() {
        let sequence = String::from_utf8(sequence.to_vec()).unwrap();

        // Add operations
        if sequence.contains('=') {
            let (label, _) = sequence.split_once('=').unwrap();
            let hash = hash(label.as_bytes());
            let b = boxes.entry(hash).or_default();

            if let Some(index) = b.iter().position(|x| x.contains(label)) {
                b[index] = sequence.clone();
            } else {
                b.push(sequence.clone());
            }
        }

        // Remove operations
        if sequence.contains('-') {
            let (label, _) = sequence.split_once('-').unwrap();
            let hash = hash(label.as_bytes());

            if let Some(b) = boxes.get_mut(&hash) {
                if let Some(index) = b.iter().position(|x| x.contains(label)) {
                    b.remove(index);
                }
            }
        }
    }

    boxes
        .iter()
        .flat_map(|(box_number, sequences)| {
            sequences.iter().enumerate().map(move |(index, sequence)| {
                let (_, operation) = sequence.split_once('=').unwrap();
                (box_number + 1) * (index as u32 + 1) * operation.parse::<u32>().unwrap()
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    fn hasher(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(hash(input.as_bytes()), expected);
    }

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 1320)]
    fn sample_01(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 145)]
    fn sample_02(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(solve_part_02(&input_generator(input)), expected);
    }
}
