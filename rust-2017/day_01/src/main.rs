use std::time::Instant;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = parse_input(include_str!("../input"));

    println!("Results for Day 1");
    println!("============================");

    // Part 1
    let now = Instant::now();
    let part_01 = day_01::part_01(&input)?;

    println!("Part 1: {:?} ({:.2?})", part_01, now.elapsed());

    // Part 2
    let now = Instant::now();
    let part_02 = day_01::part_02(&input)?;

    println!("Part 2: {:?} ({:.2?})", part_02, now.elapsed());

    Ok(())
}

fn parse_input(input: &str) -> Vec<char> {
    input.trim().chars().collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_01() {
        assert_eq!(day_01::part_01(&parse_input("1122")).unwrap(), 3);
        assert_eq!(day_01::part_01(&parse_input("1111")).unwrap(), 4);
        assert_eq!(day_01::part_01(&parse_input("1234")).unwrap(), 0);
        assert_eq!(day_01::part_01(&parse_input("91212129")).unwrap(), 9);
    }
}
