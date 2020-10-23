use std::io;

mod part_01;
mod part_02;

fn main() -> io::Result<()> {
    let input = parse_input(include_str!("../input"));

    println!("Results for Day 1");
    println!("============================");

    // Part 1
    let part_01_fuel = part_01::main(&input)?;
    println!("Part 1: {:?}", part_01_fuel);

    // Part 1
    let part_02_fuel = part_02::main(&input)?;
    println!("Part 2: {:?}", part_02_fuel);

    Ok(())
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_1: &'static str = "
    12
    14
    1969
    100756";

    const EXAMPLE_DATA_02: &'static str = "
    14
    1969
    100756";

    #[test]
    fn example_part_1() {
        assert_eq!(
            part_01::main(&parse_input(EXAMPLE_DATA_1)).unwrap(),
            2 + 2 + 654 + 33583
        )
    }

    #[test]
    fn example_part_2() {
        assert_eq!(
            part_02::main(&parse_input(EXAMPLE_DATA_02)).unwrap(),
            2 + 966 + 50346
        )
    }
}
