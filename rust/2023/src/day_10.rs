//! Day 10: Pipe Maze
//!
//! I had the correct calculations for part 1 for a long time, but I was missing
//! a check for the valid start direction. I was checking all directions
//! around the start. So, I just added the allowed start directions manually to
//! get the right answer. Then I created a real solution for it.
//!
//! I don't have the energy to do part 2 right now. I might come back to it later.
//! Did a refactor of the code using a new grid implementation that combines with my
//! point helper. I think it's a lot cleaner now.

use crate::{
    grid::Grid,
    point::{Point, DOWN, LEFT, RIGHT, UP},
};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    Ground,
}

impl Tile {
    fn is_valid_start(&self, next_tile: &Tile, direction: &Point) -> bool {
        use Tile::*;

        if !matches!(self, Tile::Start) {
            return true;
        };

        // Directions from input
        //   -
        // L S -
        //   F

        matches!(
            (next_tile, direction),
            (Horizontal, &RIGHT)
                | (Horizontal, &LEFT)
                | (Vertical, &UP)
                | (Vertical, &DOWN)
                | (NorthEast, &LEFT)
                | (NorthEast, &DOWN)
                | (NorthWest, &DOWN)
                | (NorthWest, &RIGHT)
                | (SouthEast, &LEFT)
                | (SouthEast, &UP)
                | (SouthWest, &RIGHT)
                | (SouthWest, &UP)
        )
    }
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Tile::Vertical,
            b'-' => Tile::Horizontal,
            b'L' => Tile::NorthEast,
            b'J' => Tile::NorthWest,
            b'F' => Tile::SouthEast,
            b'7' => Tile::SouthWest,
            b'S' => Tile::Start,
            b'.' => Tile::Ground,
            _ => panic!("Invalid pipe: {}", value),
        }
    }
}

fn directions(pipe: &Tile) -> Vec<Point> {
    match pipe {
        Tile::Vertical => vec![UP, DOWN],
        Tile::Horizontal => vec![LEFT, RIGHT],
        Tile::NorthEast => vec![UP, RIGHT],
        Tile::NorthWest => vec![UP, LEFT],
        Tile::SouthEast => vec![DOWN, RIGHT],
        Tile::SouthWest => vec![DOWN, LEFT],
        Tile::Start => vec![UP, RIGHT, DOWN, LEFT],
        Tile::Ground => vec![],
    }
}

#[derive(Debug)]
pub struct Input {
    grid: Grid<Tile>,
    start: Point,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    let grid: Grid<Tile> = Grid::from(input);
    let start = grid.find(Tile::Start).unwrap();

    Input { grid, start }
}

/* Part One
*
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_10::*;
let data = include_str!("../input/2023/day10.txt");
assert_eq!(solve_part_01(&input_generator(data)), 6882);
```"#]
#[aoc(day10, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    let Input { grid, start } = input;
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue = VecDeque::new();

    // Furthest point and steps
    let mut furthest = 0;

    // Add the starting point to the queue
    queue.push_back((*start, 0));

    // While there are still points to visit keep going through the pipe
    while let Some((current_point, steps)) = queue.pop_front() {
        let current_tile = grid[current_point];

        visited.insert(current_point);

        for direction in directions(&current_tile) {
            let new_point = current_point + direction;

            // Check if the next tile is a valid start direction
            if !current_tile.is_valid_start(&grid[new_point], &direction) {
                continue;
            }

            // If we haven't seen the point before, create the next point and steps.
            // If it is the furthest point, update the it.
            // Add the next point to the queue
            if !visited.contains(&new_point) {
                let next_steps = steps + 1;

                if furthest == 0 || next_steps > furthest {
                    furthest = next_steps
                }

                queue.push_back((new_point, next_steps));
            }
        }
    }

    furthest
}

/* Part Two
*
*
*/
/*
#[doc = r#"```
use advent_of_code_2023::day_10::*;
let data = include_str!("../input/2023/day10.txt");
assert_eq!(solve_part_02(&input_generator(data)), 0);
```"#]
*/
#[aoc(day10, part2)]
pub fn solve_part_02(_input: &Input) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 4);
    }

    #[test]
    fn sample_01_complex() {
        let data = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(solve_part_01(&input_generator(data)), 8);
    }
}
