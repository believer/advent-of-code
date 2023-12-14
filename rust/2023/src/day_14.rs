//! Day 14: Parabolic Reflector Dish
//!
//! This one wasn't too hard. Move things around until the desired state.
//! When I saw the number of cycles in part 2 (like many times this year)
//! I knew brute forcing it wouldn't work. The description even mentions
//! cycles so the grid must repeat itself. So, find the cycle
//! and then find the target iteration in the cycle.
//!
//! Will probably refactor this later to make it more readable.

use std::collections::HashMap;

use crate::grid::Grid;
use crate::point::{Point, DOWN, LEFT, RIGHT, UP};

#[derive(Debug)]
pub struct Input {
    grid: Grid<u8>,
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    Input {
        grid: Grid::from(input),
    }
}

/* Part One
*
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_14::*;
let data = include_str!("../input/2023/day14.txt");
assert_eq!(solve_part_01(&input_generator(data)), 108614);
```"#]
#[aoc(day14, part1)]
pub fn solve_part_01(input: &Input) -> i32 {
    let mut grid = input.grid.clone();

    for x in 0..grid.width {
        for y in 0..grid.height {
            let point = Point::new(x, y);

            // Only move round rocks
            if grid[point] == b'O' {
                let mut y = y;

                // Move until we hit the north wall
                while y > 0 {
                    let above = Point::new(x, y) + UP;
                    let current = Point::new(x, y);

                    match grid[above] {
                        b'.' => grid.swap(above, current),
                        _ => break,
                    };

                    y -= 1;
                }
            }
        }
    }

    grid.data
        .iter()
        .enumerate()
        .map(|(i, &x)| match x {
            b'O' => grid.height - (i as i32) / grid.width,
            _ => 0,
        })
        .sum()
}

/* Part Two
*
*
*/
#[doc = r#"```
use advent_of_code_2023::day_14::*;
let data = include_str!("../input/2023/day14.txt");
assert_eq!(solve_part_02(&input_generator(data)), 96447);
```"#]
#[aoc(day14, part2)]
pub fn solve_part_02(input: &Input) -> i32 {
    let mut grid = input.grid.clone();
    let mut seen = HashMap::with_capacity((grid.width * grid.height) as usize);

    seen.insert(grid.clone(), 0);

    // There must be a cycle in the grid. Loop until we get to a
    // state we've seen before
    let (start, end) = loop {
        // North
        for x in 0..grid.width {
            for y in 0..grid.height {
                let point = Point::new(x, y);

                // Only move round rocks
                if grid[point] == b'O' {
                    let mut y = y;

                    // Move until we hit the north wall
                    while y > 0 {
                        let above = Point::new(x, y) + UP;
                        let current = Point::new(x, y);

                        match grid[above] {
                            b'.' => grid.swap(above, current),
                            _ => break,
                        };

                        y -= 1;
                    }
                }
            }
        }

        // West
        for x in 0..grid.width {
            for y in 0..grid.height {
                let point = Point::new(x, y);

                if grid[point] == b'O' {
                    let mut x = x;

                    while x > 0 {
                        let left = Point::new(x, y) + LEFT;
                        let current = Point::new(x, y);

                        match grid[left] {
                            b'.' => grid.swap(left, current),
                            _ => break,
                        };

                        x -= 1;
                    }
                }
            }
        }

        // South
        for x in 0..grid.width {
            // Start from the bottom otherwise rocks from
            // the top will seem blocked by other rocks
            for y in (0..grid.height).rev() {
                let point = Point::new(x, y);

                if grid[point] == b'O' {
                    let mut y = y;

                    while y < grid.height - 1 {
                        let below = Point::new(x, y) + DOWN;
                        let current = Point::new(x, y);

                        match grid[below] {
                            b'.' => grid.swap(below, current),
                            _ => break,
                        };

                        y += 1;
                    }
                }
            }
        }

        // East
        for y in 0..grid.height {
            for x in (0..grid.width).rev() {
                let point = Point::new(x, y);

                if grid[point] == b'O' {
                    let mut x = x;

                    while x < grid.width - 1 {
                        let right = Point::new(x, y) + RIGHT;
                        let current = Point::new(x, y);

                        match grid[right] {
                            b'.' => grid.swap(right, current),
                            _ => break,
                        };

                        x += 1;
                    }
                }
            }
        }

        // Check if we've seen this state before
        if let Some(count) = seen.get(&grid) {
            break (*count, seen.len());
        }

        let count = seen.len();
        seen.insert(grid.clone(), count);
    };

    // Find the target iteration in the cycle
    let cycle = end - start;
    let remainder = (1_000_000_000 - start) % cycle;
    let target = start + remainder;
    let (target_grid, _) = seen.iter().find(|(_, &v)| v == target).unwrap();

    target_grid
        .data
        .iter()
        .enumerate()
        .map(|(i, &x)| match x {
            b'O' => target_grid.height - (i as i32) / target_grid.width,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        136
    )]
    fn sample_01(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        64
    )]
    fn sample_02(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(solve_part_02(&input_generator(input)), expected);
    }
}
