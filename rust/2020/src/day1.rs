use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> HashSet<u64> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part_01(input: &HashSet<u64>) -> u64 {
    for x in input {
        let y = 2020 - x;
        if input.contains(&y) {
            return x * y;
        }
    }

    0
}

#[aoc(day1, part2)]
pub fn solve_part_02(input: &HashSet<u64>) -> u64 {
    for x in input {
        for y in input {
            if x + y > 2020 {
                continue;
            }

            let z = 2020 - x - y;

            if input.contains(&z) {
                return x * y * z;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "
1721
979
366
299
675
1456
";

        assert_eq!(solve_part_01(&input_generator(data)), 514579)
    }

    #[test]
    fn sample_02() {
        let data = "
1721
979
366
299
675
1456
";

        assert_eq!(solve_part_02(&input_generator(data)), 241861950)
    }
}
