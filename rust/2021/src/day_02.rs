use crate::common;

// Day 2: TBD

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    common::input_vec(input)
}

/* Part One
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2021::day_02::*;
/// let data = include_str!("../input/2021/day2.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 1947824);
/// ```
#[aoc(day2, part1)]
pub fn solve_part_01(input: &[String]) -> u32 {
    let mut depth = 0;
    let mut horizontal_position = 0;

    for i in input {
        let parts = i.split_whitespace().collect::<Vec<_>>();
        let value = parts[1].parse::<u32>().unwrap();

        match parts[0] {
            "forward" => horizontal_position += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => unreachable!("Invalid command"),
        }
    }

    depth * horizontal_position
}

/* Part Two
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2021::day_02::*;
/// let data = include_str!("../input/2021/day2.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 0);
/// ```
#[aoc(day2, part2)]
pub fn solve_part_02(input: &[String]) -> u32 {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal_position = 0;

    for i in input {
        let parts = i.split_whitespace().collect::<Vec<_>>();
        let value = parts[1].parse::<u32>().unwrap();

        match parts[0] {
            "forward" => {
                horizontal_position += value;
                depth += value * aim;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => unreachable!("Invalid command"),
        }
    }

    depth * horizontal_position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(solve_part_01(&input_generator(data)), 150)
    }

    #[test]
    fn sample_02() {
        let data = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(solve_part_02(&input_generator(data)), 900)
    }
}
