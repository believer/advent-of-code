// Day 3 - Perfectly Spherical Houses in a Vacuum
//
// Pretty simple movements in a grid. Then just count the number of
// coordinates visited.
//
// Updates:
//
// I was using a HashMap to store coordinates AND presents. Turns out
// that I don't need the presents. So I changed it to a HashSet.
// I also used with_capacity() to pre-allocate the HashSet as the
// maximum possible houses visited should be the length of the input.
// These changes made the solution run 50% faster.

use std::collections::HashSet;

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
#[aoc(day3, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    let mut houses: HashSet<(isize, isize)> = HashSet::with_capacity(input.len());
    let mut santa = Santa::new();

    houses.insert((santa.x, santa.y));

    for direction in input {
        santa.move_to(direction);
        houses.insert((santa.x, santa.y));
    }

    houses.len()
}

/* Part Two
*/
#[aoc(day3, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let mut houses: HashSet<(isize, isize)> = HashSet::with_capacity(input.len());
    let mut santa = Santa::new();
    let mut robosanta = Santa::new();

    houses.insert((santa.x, santa.y));
    houses.insert((robosanta.x, robosanta.y));

    for directions in input.chunks(2) {
        match directions {
            [santa_move, robosanta_move] => {
                santa.move_to(santa_move);
                robosanta.move_to(robosanta_move);

                houses.insert((santa.x, santa.y));
                houses.insert((robosanta.x, robosanta.y));
            }
            _ => panic!("Invalid input"),
        }
    }

    houses.len()
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
