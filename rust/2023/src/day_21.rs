//! Day 21: Step Counter
//!
//! Part one wasn't that hard using a breadth-first search (BFS). Part two
//! is trickier, but we can use a similar approach. We can also make some
//! assumptions from the input:
//!
//! - The column of the start point, S, is always empty.
//! - The row of the start point, S, is always empty.
//! - The other edge of the map is always empty
//!
//! This means that we can move in a straight line until the furthest
//! distance. We can also find any distance in the grid by comparing
//! to where we enter it.
//!
//! The width of the grid is and uneven number (131) since we're
//! starting in the middle. This means that each iteration will be offset
//! a bit, making them different.

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

fn find_steps(input: &Input, steps: usize) -> usize {
    let mut todo = VecDeque::new();
    let mut answer = HashSet::new();
    let mut seen = HashSet::new();

    todo.push_back((input.start, steps));
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

/* Part One
*
*/
#[aoc(day21, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    find_steps(input, 64)
}

/* Part Two
*
*/
#[aoc(day21, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let steps = 26501365;

    // Using the assumptions from the top of the file, we can
    // find the total width of the grids that we can reach.
    // This value turns out to be 202300, which seems like a hint
    // to the current year. We remove one since we're starting in the middle.
    let grid_width = ((steps / input.map.width) as f64).floor() - 1.0;

    // Odd grids are the starting grid and every other grid. Even grids
    // are the ones in between. These formulas calculate the number of
    // grids we can reach.
    let odd_grids = ((grid_width / 2.0).floor() * 2.0 + 1.0).powf(2.0);
    let even_grids = (((grid_width + 1.0) / 2.0).floor() * 2.0).powf(2.0);

    // Points we're able to reach in the grids. The number of steps
    // is one large enough to reach around the edges of the grid. Plus
    // one to make it odd.
    let odd_points = find_steps(input, input.map.width as usize * 2 + 1);
    let even_points = find_steps(input, input.map.width as usize * 2);

    // We can't reach every corner of the final grids. We can almost reach the
    // furthest end in a straight line, but the corners won't be accessible due to the
    // Manhattan distance. Since the grids are uneven, we need to subtract one from
    // the size. So, calculate the number of points we can reach in the corner grids.

    let top_middle = Point::new(input.start.x, input.map.height - 1);
    let right_middle = Point::new(0, input.start.y);
    let bottom_middle = Point::new(input.start.x, 0);
    let left_middle = Point::new(input.map.width - 1, input.start.y);

    let corner_points = [top_middle, right_middle, bottom_middle, left_middle]
        .iter()
        .map(|p| {
            find_steps(
                &Input {
                    start: *p,
                    map: input.map.clone(),
                },
                input.map.width as usize - 1,
            )
        })
        .sum::<usize>();

    // We need to calculate a small slivers of one of the outer grids
    // that we can reach. The number of steps to reach the outer grid
    // is the size / 2 - 1.
    let small_position_steps = (input.map.height as f64 / 2.0).floor() - 1.0;
    let small_top_right = Point::new(0, input.map.height - 1);
    let small_top_left = Point::new(input.map.width - 1, input.map.height - 1);
    let small_bottom_right = Point::new(0, 0);
    let small_bottom_left = Point::new(input.map.width - 1, 0);

    let small_points = [
        small_top_right,
        small_top_left,
        small_bottom_right,
        small_bottom_left,
    ]
    .iter()
    .map(|p| {
        find_steps(
            &Input {
                start: *p,
                map: input.map.clone(),
            },
            small_position_steps as usize,
        )
    })
    .sum::<usize>();

    // We then need to calculate the number of points we can reach in the
    // larger slivers of the outer grids.

    let large_position_steps = ((input.map.height as f64 * 3.0) / 2.0).floor() - 1.0;
    let large_top_right = Point::new(0, input.map.height - 1);
    let large_top_left = Point::new(input.map.width - 1, input.map.height - 1);
    let large_bottom_right = Point::new(0, 0);
    let large_bottom_left = Point::new(input.map.width - 1, 0);

    let large_points = [
        large_top_right,
        large_top_left,
        large_bottom_right,
        large_bottom_left,
    ]
    .iter()
    .map(|p| {
        find_steps(
            &Input {
                start: *p,
                map: input.map.clone(),
            },
            large_position_steps as usize,
        )
    })
    .sum::<usize>();

    odd_grids as usize * odd_points
        + even_grids as usize * even_points
        + corner_points
        + (grid_width as usize + 1) * small_points
        + (grid_width as usize) * large_points
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
