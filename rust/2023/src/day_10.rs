//! Day 10: Pipe Maze
//!
//! I had the correct calculations for part 1 for a long time, but I was missing
//! a check for the valid start direction. I was checking all directions
//! around the start. So, I just added the allowed start directions manually to
//! get the right answer. Then I created a real solution for it.

use crate::point::{Point, DOWN, LEFT, RIGHT, UP};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq)]
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

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            'F' => Tile::SouthEast,
            '7' => Tile::SouthWest,
            'S' => Tile::Start,
            '.' => Tile::Ground,
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

fn valid_start_direction(pipe: &Tile, direction: &Point) -> bool {
    use Tile::*;

    // Directions from input
    //   -
    // L S -
    //   F

    matches!(
        (pipe, direction),
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

#[derive(Debug)]
pub struct Input {
    grid: Vec<Vec<Tile>>,
    start: Point,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    let mut grid = Vec::new();
    let mut start = Point::new(0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, tile) in line.chars().enumerate() {
            let tile = tile.into();

            if let Tile::Start = tile {
                start = Point::new(x as i32, y as i32);
            }

            row.push(tile);
        }

        grid.push(row);
    }

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
    let rows = input.grid.len();
    let cols = input.grid[0].len();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue = VecDeque::new();

    // Add the starting point to the queue
    queue.push_back((input.start, 0));

    // Furthest point and steps
    let mut furthest: Option<(Point, u64)> = None;

    // While there are still points to visit keep going through the pipe
    while let Some((current_point, steps)) = queue.pop_front() {
        visited.insert(current_point);

        let current_tile = &input.grid[current_point.y as usize][current_point.x as usize];

        for direction in directions(current_tile) {
            let new_point = current_point + direction;
            let new_x = new_point.x as usize;
            let new_y = new_point.y as usize;

            if new_x < cols && new_y < rows && !visited.contains(&new_point) {
                let next_tile = &input.grid[new_y][new_x];

                // Check if the next tile is a valid start direction
                if current_tile == &Tile::Start && !valid_start_direction(next_tile, &direction) {
                    continue;
                }

                // Create the next point and steps
                // If it is the furthest point, update the furthest point and steps
                // Add the next point to the queue
                let next = (new_point, steps + 1);

                if furthest.is_none() || next.1 > furthest.unwrap().1 {
                    furthest = Some(next);
                }

                queue.push_back(next);
            }
        }
    }

    furthest.unwrap().1
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
pub fn solve_part_02(input: &Input) -> u64 {
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
