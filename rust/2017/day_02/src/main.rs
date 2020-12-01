use std::time::Instant;

trait VecExt {
    fn sorted(self) -> Self;
}

impl<T> VecExt for Vec<T>
where
    T: std::cmp::Ord,
{
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let input = parse_input(include_str!("../input"));

    println!("Results for Day 2");
    println!("============================");

    // Part 1
    let now = Instant::now();
    let part_01 = day_02::part_01(&input);

    println!("Part 1: {:?} ({:.2?})", part_01, now.elapsed());

    // Part 2
    let now = Instant::now();
    let part_02 = day_02::part_02(&input);

    println!("Part 2: {:?} ({:.2?})", part_02, now.elapsed());
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
                .sorted()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_part_01() {
        let data = "
5 1 9 5
7 5 3
2 4 6 8
";

        assert_eq!(day_02::part_01(&parse_input(data)), 18);
    }

    #[test]
    fn real_input_part_01() {
        let input = parse_input(include_str!("../input"));
        assert_eq!(day_02::part_01(&input), 34925)
    }

    #[test]
    fn test_data_part_02() {
        let data = "
5 9 2 8
9 4 7 3
3 8 6 5
";

        assert_eq!(day_02::part_02(&parse_input(data)), 9)
    }
}
