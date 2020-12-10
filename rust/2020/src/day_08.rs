use crate::common;
use lazy_static::lazy_static;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

// Day 8 - Handheld Halting

lazy_static! {
    static ref RE: Regex = Regex::new(r"(acc|jmp|nop)\s([+-]\d+)").unwrap();
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    NoOp,
    Jump,
    Accumulate,
}

impl FromStr for Operation {
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

#[derive(Debug)]
pub struct Instruction {
    id: String,
    operation: Operation,
    argument: i32,
}

impl Instruction {
    fn new(s: &str) -> Option<Instruction> {
        let ins = RE.captures(&s)?;
        let id: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();

        Some(Instruction {
            id,
            operation: ins.get(1)?.as_str().parse().unwrap(),
            argument: ins.get(2)?.as_str().parse().unwrap(),
        })
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Instruction::new(s).ok_or_else(|| panic!("Invalid Instruction"))
    }
}

struct Program {
    accumulator: i32,
    seen: HashSet<String>,
    step: usize,
}

impl Program {
    fn next_step(&mut self, instruction: &Instruction) {
        match instruction.operation {
            Operation::NoOp => (),
            Operation::Accumulate => self.accumulator += instruction.argument,
            Operation::Jump => self.step = (self.step as i32 + instruction.argument) as usize,
        };
    }

    fn step_one(&mut self) {
        self.step += 1
    }

    fn add_seen_and_step(&mut self, instruction: &Instruction) {
        self.seen.insert(instruction.id.to_string());
        self.next_step(instruction);
    }

    fn calculate_to_error(&mut self, instructions: &[Instruction]) -> Option<i32> {
        loop {
            match instructions.get(self.step) {
                Some(instruction) => {
                    if self.seen.get(&instruction.id).is_some() {
                        return Some(self.accumulator);
                    }

                    self.add_seen_and_step(&instruction);

                    if let Operation::Jump = instruction.operation {
                        continue;
                    }

                    self.step_one();
                }
                None => return None,
            }
        }
    }

    fn calculate_with_fix(&mut self, instructions: &[Instruction]) -> Option<i32> {
        loop {
            match instructions.get(self.step) {
                Some(instruction) => {
                    if self.step == instructions.len() {
                        return Some(self.accumulator);
                    }

                    if self.seen.get(&instruction.id).is_some() {
                        return None;
                    }

                    self.add_seen_and_step(&instruction);

                    if let Operation::Jump = instruction.operation {
                        continue;
                    }

                    self.step_one();
                }
                None => return Some(self.accumulator),
            }
        }
    }

    fn new() -> Program {
        Program {
            accumulator: 0,
            seen: HashSet::new(),
            step: 0,
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    common::input_vec(input)
}

/* Part One
 *
 * Your flight to the major airline hub reaches cruising altitude without incident.
 * While you consider checking the in-flight menu for one of those drinks that come
 * with a little umbrella, you are interrupted by the kid sitting next to you.
 *
 * Their handheld game console won't turn on! They ask if you can take a look.
 *
 * You narrow the problem down to a strange infinite loop in the boot
 * code (your puzzle input) of the device. You should be able to fix it,
 * but first you need to be able to run the code in isolation.
 *
 * The boot code is represented as a text file with one instruction per line of text.
 * Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).
 *
 * acc increases or decreases a single global value called the accumulator by the value given in the argument.
 * For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0.
 * After an acc instruction, the instruction immediately below it is executed next.
 * jmp jumps to a new instruction relative to itself. The next instruction to execute
 * is found using the argument as an offset from the jmp instruction;
 * for example, jmp +2 would skip the next instruction, jmp +1 would continue to
 * the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
 * nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
 * For example, consider the following program:
 *
 * nop +0
 * acc +1
 * jmp +4
 * acc +3
 * jmp -3
 * acc -99
 * acc +1
 * jmp -4
 * acc +6
 *
 * These instructions are visited in this order:
 *
 * nop +0  | 1
 * acc +1  | 2, 8(!)
 * jmp +4  | 3
 * acc +3  | 6
 * jmp -3  | 7
 * acc -99 |
 * acc +1  | 4
 * jmp -4  | 5
 * acc +6  |
 *
 * First, the nop +0 does nothing.
 * Then, the accumulator is increased from 0 to 1 (acc +1) and jmp +4
 * sets the next instruction to the other acc +1 near the bottom.
 * After it increases the accumulator from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3.
 * It sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.
 *
 * This is an infinite loop: with this sequence of jumps, the program will run forever.
 * The moment the program tries to run any instruction a second time, you know it will never terminate.
 *
 * Immediately before the program would run an instruction a second time, the value in the accumulator is 5.
 *
 * Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator?
*/
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_08::*;
/// let input = include_str!("../input/2020/day8.txt");
/// assert_eq!(solve_part_01(&input_generator(input)).unwrap(), 1489);
/// ```
#[aoc(day8, part1)]
pub fn solve_part_01(input: &[Instruction]) -> Option<i32> {
    Program::new().calculate_to_error(&input)
}

/* Part Two
 *
 * After some careful analysis, you believe that exactly one instruction is corrupted.
 *
 * Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp.
 * (No acc instructions were harmed in the corruption of this boot code.)
 *
 * The program is supposed to terminate by attempting to execute an instruction
 * immediately after the last instruction in the file. By changing exactly one jmp
 * or nop, you can repair the boot code and make it terminate correctly.
 *
 * For example, consider the same program from above:
 *
 * nop +0
 * acc +1
 * jmp +4
 * acc +3
 * jmp -3
 * acc -99
 * acc +1
 * jmp -4
 * acc +6
 *
 * If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction
 * infinite loop, never leaving that instruction. If you change almost any of the jmp instructions,
 * the program will still eventually find another jmp instruction and loop forever.
 *
 * However, if you change the second-to-last instruction (from jmp -4 to nop -4),
 * the program terminates! The instructions are visited in this order:
 *
 * nop +0  | 1
 * acc +1  | 2
 * jmp +4  | 3
 * acc +3  |
 * jmp -3  |
 * acc -99 |
 * acc +1  | 4
 * nop -4  | 5
 * acc +6  | 6
 *
 * After the last instruction (acc +6), the program terminates by attempting to run the
 * instruction below the last instruction in the file. With this change, after the program
 * terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).
 *
 * Fix the program so that it terminates normally by changing exactly one jmp (to nop)
 * or nop (to jmp). What is the value of the accumulator after the program terminates?
*/
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_08::*;
/// let input = include_str!("../input/2020/day8.txt");
/// assert_eq!(solve_part_02(&input_generator(input)).unwrap(), 1539);
/// ```
#[aoc(day8, part2)]
pub fn solve_part_02(input: &[Instruction]) -> Option<i32> {
    for i in 0..input.len() {
        let instructions: Vec<Instruction> = input
            .iter()
            .enumerate()
            .map(|(j, l)| {
                if i == j {
                    let op = match &l.operation {
                        Operation::NoOp => Operation::Jump,
                        Operation::Jump => Operation::NoOp,
                        Operation::Accumulate => Operation::Accumulate,
                    };

                    Instruction {
                        operation: op,
                        id: l.id.to_string(),
                        argument: l.argument,
                    }
                } else {
                    Instruction {
                        operation: l.operation,
                        id: l.id.to_string(),
                        argument: l.argument,
                    }
                }
            })
            .collect();

        match Program::new().calculate_with_fix(&instructions) {
            Some(v) => return Some(v),
            None => continue,
        }
    }

    None
}

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

        assert_eq!(solve_part_01(&input_generator(data)).unwrap(), 5)
    }

    /// Test example data on part 2
    #[test]
    fn sample_02() {
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

        assert_eq!(solve_part_02(&input_generator(data)).unwrap(), 8)
    }
}
