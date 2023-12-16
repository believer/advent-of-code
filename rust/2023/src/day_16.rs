//! Day 16


use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashSet, VecDeque};

use crate::{
    grid::Grid,
    point::{Point, DOWN, LEFT, ORIGIN, RIGHT, UP},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Beam {
    head: Point,
    direction: Point,
}

impl Beam {
    fn new(head: Point, direction: Point) -> Self {
        Self { head, direction }
    }
}

fn next_direction(tile: &u8, direction: Point) -> Vec<Point> {
    match tile {
        b'.' => vec![direction],
        b'|' => match direction {
            UP | DOWN => vec![direction],
            LEFT | RIGHT => vec![UP, DOWN],
            _ => unreachable!(),
        },
        b'-' => match direction {
            LEFT | RIGHT => vec![direction],
            UP | DOWN => vec![LEFT, RIGHT],
            _ => unreachable!(),
        },
        b'/' => match direction {
            UP => vec![RIGHT],
            RIGHT => vec![UP],
            DOWN => vec![LEFT],
            LEFT => vec![DOWN],
            _ => unreachable!(),
        },
        b'\\' => match direction {
            UP => vec![LEFT],
            RIGHT => vec![DOWN],
            DOWN => vec![RIGHT],
            LEFT => vec![UP],
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn fire_ze_lasers(input: &Input, start: Beam) -> usize {
    let mut energized_tiles: HashSet<Point> = HashSet::new();
    let mut beams = VecDeque::new();
    let mut seen: HashSet<Beam> = HashSet::new();

    beams.push_back(start);

    while let Some(beam) = beams.pop_front() {
        let Beam { head, direction } = beam;

        if !input.tiles.contains(head) || !seen.insert(beam) {
            continue;
        }

        energized_tiles.insert(head);

        // Find the next directions to move the beam
        next_direction(&input.tiles[head], direction)
            .iter()
            .for_each(|&d| {
                beams.push_back(Beam::new(head + d, d));
            });
    }

    energized_tiles.len()
}

pub struct Input {
    tiles: Grid<u8>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    Input {
        tiles: Grid::from(input),
    }
}

/* Part One
*
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_16::*;
let data = include_str!("../input/2023/day16.txt");
assert_eq!(solve_part_01(&input_generator(data)), 7562);
```"#]
#[aoc(day16, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    fire_ze_lasers(input, Beam::new(ORIGIN, RIGHT))
}

/* Part Two
*
*
*/
#[doc = r#"```
use advent_of_code_2023::day_16::*;
let data = include_str!("../input/2023/day16.txt");
assert_eq!(solve_part_02(&input_generator(data)), 7793);
```"#]
#[aoc(day16, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let mut possible_starts = vec![];

    // Find all possible starting points from the edges of the grid
    for x in 0..input.tiles.width {
        possible_starts.push(Beam::new(Point::new(x, 0), DOWN));
        possible_starts.push(Beam::new(Point::new(x, input.tiles.height - 1), UP));
    }

    for y in 0..input.tiles.height {
        possible_starts.push(Beam::new(Point::new(0, y), RIGHT));
        possible_starts.push(Beam::new(Point::new(input.tiles.width - 1, y), LEFT));
    }

    // For each starting point, fire ze lasers and find the max
    // Use rayon to parallelize the search for a speed boost
    possible_starts
        .par_iter()
        .map(|start| fire_ze_lasers(input, *start))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....",
        46
    )]
    fn sample_01(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case(
        ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....",
        51
    )]
    fn sample_02(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_02(&input_generator(input)), expected);
    }
}
