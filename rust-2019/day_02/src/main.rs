use std::time::Instant;

struct IntCode {
    input: Vec<u32>,
    pointer: usize,
}

impl IntCode {
    fn new(input: &Vec<u32>) -> IntCode {
        IntCode {
            input: input.clone(),
            pointer: 0,
        }
    }

    fn patch(&mut self, index: usize, value: u32) {
        self.input[index] = value
    }

    fn run(&mut self) -> u32 {
        self.pointer = 0;

        loop {
            match self.input[self.pointer] {
                99 => break self.input[0],
                opcode => {
                    let noun = self.input[self.input[self.pointer + 1] as usize];
                    let verb = self.input[self.input[self.pointer + 2] as usize];
                    let output_position = self.input[self.pointer + 3] as usize;

                    self.input[output_position] = match opcode {
                        1 => noun + verb,
                        2 => noun * verb,
                        _ => panic!("Unexpected code {}", opcode),
                    };

                    self.pointer += 4;
                }
            }
        }
    }
}

fn main() {
    let input = parse_input(include_str!("../input"));

    println!("Results for Day 2");
    println!("============================");

    // Part 1
    let now = Instant::now();
    let mut part_01 = IntCode::new(&input);

    part_01.patch(1, 12);
    part_01.patch(2, 2);

    println!("Part 1: {:?} ({:.2?})", part_01.run(), now.elapsed());

    // Part 2
    let now = Instant::now();
    let needle = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut part_02 = IntCode::new(&input);
            part_02.patch(1, noun);
            part_02.patch(2, verb);

            if part_02.run() == needle {
                println!("Part 2: {:?} ({:.2?})", 100 * noun + verb, now.elapsed());
                break;
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.split(',').map(|num| num.parse::<u32>().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_1: &'static str = "1,0,0,0,99";
    const EXAMPLE_DATA_2: &'static str = "1,1,1,4,99,5,6,0,99";

    #[test]
    fn part_01_example() {
        let input = parse_input(EXAMPLE_DATA_1);
        let mut computer = IntCode::new(&input);

        computer.run();

        assert_eq!(computer.input, [2, 0, 0, 0, 99])
    }

    #[test]
    fn part_01_example_2() {
        let input = parse_input(EXAMPLE_DATA_2);
        let mut computer = IntCode::new(&input);

        computer.run();

        assert_eq!(computer.input, [30, 1, 1, 4, 2, 5, 6, 0, 99])
    }

    #[test]
    fn part_01_example_result() {
        let input = parse_input(EXAMPLE_DATA_1);
        let mut computer = IntCode::new(&input);

        assert_eq!(computer.run(), 2)
    }

    #[test]
    fn part_01_example_result_2() {
        let input = parse_input(EXAMPLE_DATA_2);
        let mut computer = IntCode::new(&input);

        assert_eq!(computer.run(), 30)
    }
}
