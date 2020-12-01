use std::time::Instant;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = parse_input(include_str!("../input"));

    println!("Results for Day 2");
    println!("============================");

    // Part 1
    let now = Instant::now();
    let part_01 = day_02::part_01(&input)?;

    println!("Part 1: {:?} ({:.2?})", part_01, now.elapsed());

    // Part 2
    let now = Instant::now();
    let part_02 = day_02::part_02(&input)?;

    println!("Part 2: {:?} ({:.2?})", part_02, now.elapsed());

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
