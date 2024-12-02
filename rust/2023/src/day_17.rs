//! Day 17: Clumsy Crucible
//!
//! Quickly understood that it was a pathfinding problem. I've used
//! Dijkstra's algorithm from the `pathfinding` crate before, but
//! I wanted to try implementing it myself.
//!
//! My solution didn't handle part 2 in that it needed to take at least
//! 4 steps before reaching the end as well. My lowest cost path
//! was too low. So, to get the answer I printed all of the lowest costs
//! and guessed until I got the right answer haha.
//!
//! I then found the bug and fixed it. Just needed to add a check to
//! make sure that the path was at least 4 steps long before saving
//! the cost.

use crate::grid::Grid;
use crate::heap::MinHeap;
use crate::point::{Point, DOWN, ORIGIN, RIGHT};
use std::collections::HashMap;

pub struct Input {
    grid: Grid<u8>,
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Input {
    Input {
        grid: Grid::from(input),
    }
}

fn dijkstra(grid: &Grid<u8>, min_steps: u32, max_steps: u32) -> u32 {
    // End position is the bottom right corner
    let end = Point::new(grid.width - 1, grid.height - 1);

    // Set starting cost to the max because we want to compare
    // to get the lowest possible cost
    let mut minimum_cost = u32::MAX;

    // Setup the heap and visited map. The heap stores as a
    // key-value pair where the key is the cost and the value
    // is the state
    let mut todo = MinHeap::new();
    let mut visited = HashMap::new();

    // Setup starting positions and directions
    for start in [(ORIGIN, RIGHT, 1), (ORIGIN, DOWN, 1)] {
        todo.push(0, start);
        visited.insert(start, 0);
    }

    // Dijkstra's algorithm with the special rules from the problem
    // Only move straight if we have not moved straight for max steps
    // Need to move at least min steps before turning left or right
    while let Some((cost, (position, direction, steps))) = todo.pop() {
        let mut push = |direction: Point, straight: u32| {
            let next = position + direction;
            let state = (next, direction, straight);

            if grid.contains(next) {
                let value = (grid[next] as char).to_digit(10).unwrap();
                let next_cost = cost + value;
                let should_store = !visited.contains_key(&state) || next_cost < visited[&state];

                if should_store {
                    todo.push(next_cost, state);
                    visited.insert(state, next_cost);
                }
            }
        };

        // We've reached the end, save the minimum cost if it is lower
        // than the current minimum cost
        if position == end && steps >= min_steps {
            minimum_cost = minimum_cost.min(cost);
        }

        // Move straight if we have not moved straight for max steps
        if steps < max_steps {
            push(direction, steps + 1);
        }

        // Turn left and right if we have moved at least min steps
        if steps >= min_steps {
            push(direction.counter_clockwise(), 1);
            push(direction.clockwise(), 1);
        }
    }

    minimum_cost
}

/* Part One
*
* Find the lowest cost path from the top left corner to the bottom
* right corner. You can only move a maximum of 3 steps in a straight
* line before needing to turn left or right.
*/
#[aoc(day17, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    dijkstra(&input.grid, 1, 3)
}

/* Part Two
*
* Find the lowest cost path from the top left corner to the bottom
* right corner. You can only move a maximum of 10 steps in a straight
* line before needing to turn left or right. You must take at least
* 4 steps before turning left or right, this includes at least
* 4 steps before reaching the end.
*/
#[aoc(day17, part2)]
pub fn solve_part_02(input: &Input) -> u32 {
    dijkstra(&input.grid, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        102
    )]
    fn sample_01(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        94
    )]
    #[case(
        "111111111111
999999999991
999999999991
999999999991
999999999991",
        71
    )]
    fn sample_02(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(solve_part_02(&input_generator(input)), expected);
    }
}
