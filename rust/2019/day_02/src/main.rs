use std::time::Instant;

mod int_code;
mod part_01;
mod part_02;

fn main() {
    let input = parse_input(include_str!("../input"));

    println!("Results for Day 2");
    println!("============================");

    // Part 1
    let now = Instant::now();
    let part_01_result = part_01::main(&input, 12, 2);

    println!("Part 1: {:?} ({:.2?})", part_01_result, now.elapsed());

    // Part 2
    let now = Instant::now();
    let part_02_result = part_02::main(&input);

    println!("Part 2: {:?} ({:.2?})", part_02_result, now.elapsed());
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
    use crate::int_code::IntCode;

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
