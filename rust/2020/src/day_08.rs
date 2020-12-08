use crate::common;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

// Day 8

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Operation {
    NoOp,
    Jump,
    Accumulate,
}

impl std::str::FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "nop" => Operation::NoOp,
            "jmp" => Operation::Jump,
            "acc" => Operation::Accumulate,
            _ => unreachable!("Invalid value"),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Instruction {
    pub id: String,
    operation: Operation,
    argument: i32,
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ins: Vec<String> = s.split_whitespace().map(|l| l.to_string()).collect();
        let id: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

        Ok(Instruction {
            id,
            operation: ins[0].parse().unwrap(),
            argument: ins[1].parse().unwrap(),
        })
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    common::input_vec(input)
}

///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_08::*;
/// let input = include_str!("../input/2020/day8.txt");
/// assert_eq!(solve_part_01(&input_generator_part_01(input).unwrap()), 226);
/// ```
#[aoc(day8, part1)]
pub fn solve_part_01(input: &[Instruction]) -> i32 {
    let mut accumulator = 0;
    let mut n = 0;
    let mut seen: HashSet<&String> = HashSet::new();

    loop {
        match input.get(n) {
            Some(instruction) => {
                if let Some(_) = seen.get(&instruction.id) {
                    break;
                }

                seen.insert(&instruction.id);

                match instruction.operation {
                    Operation::NoOp => n = n + 1,
                    Operation::Accumulate => {
                        accumulator += instruction.argument;
                        n = n + 1;
                    }
                    Operation::Jump => n = (n as i32 + instruction.argument) as usize,
                };
            }
            None => break,
        }
    }

    accumulator
}

/////your puzzle answer was.
///// ```
///// use advent_of_code_2020::day_08::*;
///// let input = include_str!("../input/2020/day8.txt");
///// assert_eq!(solve_part_02(&input_generator_part_02(input).unwrap()), 9569);
///// ```
//#[aoc(day8, part2)]
//pub fn solve_part_02(_input: &[String]) -> u32 {
//    0
//}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn sample_01() {
        let data = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

        assert_eq!(solve_part_01(&input_generator(data)), 5)
    }
}
