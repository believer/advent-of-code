//! Day 21

use crate::{
    grid::Grid,
    point::{Point, DOWN, LEFT, RIGHT, UP},
};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct Input {
    map: Grid<u8>,
    start: Point,
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Input {
    let map = Grid::from(input);
    let start = map.find(b'S').unwrap();

    Input { map, start }
}

/* Part One
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_21::*;
let data = include_str!("../input/2023/day21.txt");
assert_eq!(solve_part_01(&input_generator(data)), 3788);
```"#]
#[aoc(day21, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    let mut todo = VecDeque::new();
    let mut answer = HashSet::new();
    let mut seen = HashSet::new();

    todo.push_back((input.start, 64));
    seen.insert(input.start);

    while let Some((pos, steps)) = todo.pop_front() {
        // If the number of steps is even, we can go back
        // and forth between the two positions.
        if steps % 2 == 0 {
            answer.insert(pos);
        }

        // If no more steps, skip
        if steps == 0 {
            continue;
        }

        // Check in each direction
        for direction in [UP, DOWN, LEFT, RIGHT].iter() {
            let next = pos + *direction;
            let is_seen = seen.contains(&next);
            let is_in_map = input.map.contains(next);

            if !is_in_map || is_seen || input.map[next] == b'#' {
                continue;
            }

            // Save the position and decrement the steps
            seen.insert(next);
            todo.push_back((next, steps - 1));
        }
    }

    answer.len()
}

/* Part Two
*
*
*/
#[doc = r#"```
use advent_of_code_2023::day_21::*;
let data = include_str!("../input/2023/day21.txt");
assert_eq!(solve_part_02(&input_generator(data)), 245114020323037);
```"#]
#[aoc(day21, part2)]
pub fn solve_part_02(input: &Input) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
        42
    )]
    fn sample_01(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }
}
