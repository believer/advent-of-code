// Day 3 -

use std::collections::HashMap;

type Input = Vec<Direction>;

#[derive(Debug)]
pub enum Direction {
    Right,
    Down,
    Up,
    Left,
}

pub struct Santa {
    x: isize,
    y: isize,
}

impl Santa {
    fn new() -> Self {
        Santa { x: 0, y: 0 }
    }

    fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Up => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    input
        .split("")
        .filter(|s| !s.is_empty())
        .map(|s| match s {
            ">" => Direction::Right,
            "v" => Direction::Down,
            "^" => Direction::Up,
            "<" => Direction::Left,
            b => panic!("Invalid direction {b}"),
        })
        .collect()
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2015::day_03::*;
/// let data = include_str!("../input/2015/day3.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 2572);
/// ```
#[aoc(day3, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    let mut packages: HashMap<(isize, isize), u32> = HashMap::new();
    let mut santa = Santa::new();

    packages.insert((santa.x, santa.y), 1);

    for direction in input {
        santa.move_to(direction);

        *packages.entry((santa.x, santa.y)).or_insert(0) += 1;
    }

    packages.len()
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2015::day_03::*;
/// let data = include_str!("../input/2015/day3.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 2631);
/// ```
#[aoc(day3, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let mut packages: HashMap<(isize, isize), u32> = HashMap::new();
    let mut santa = Santa::new();
    let mut robosanta = Santa::new();

    *packages.entry((santa.x, santa.y)).or_insert(0) += 1;
    *packages.entry((robosanta.x, robosanta.y)).or_insert(0) += 1;

    for directions in input.chunks(2) {
        match directions {
            [santa_move, robosanta_move] => {
                santa.move_to(santa_move);
                robosanta.move_to(robosanta_move);

                *packages.entry((santa.x, santa.y)).or_insert(0) += 1;
                *packages.entry((robosanta.x, robosanta.y)).or_insert(0) += 1;
            }
            _ => panic!("Invalid input"),
        }
    }

    packages.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = ">";
    const SAMPLE_2: &str = "^>v<";
    const SAMPLE_3: &str = "^v^v^v^v^v";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE_1)), 2)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE_2)), 4)
    }

    #[test]
    fn sample_03() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE_3)), 2)
    }

    #[test]
    fn sample_part_2_01() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE_2)), 3)
    }

    #[test]
    fn sample_part_2_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE_3)), 11)
    }
}
