//! Day 23: A Long Walk
//!
//! Can solve both parts using a BFS. It's a bit slow, but it works.
//! A simplification in part 2 is that we can find all the junctions in the grid
//! and only traverse those instead of the entire grid.

use crate::{
    grid::Grid,
    point::{Point, CARDINALS, DOWN, LEFT, RIGHT, UP},
};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Input {
    trails: Grid<u8>,
    start: Point,
    end: Point,
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Input {
    let trails = Grid::from(input);

    // I calculated these at first, but they are the same
    // for both the test data and the real input so we can
    // just hardcode them.
    let start = Point::new(1, 0);
    let end = Point::new(trails.width - 2, trails.height - 1);

    Input { trails, start, end }
}

/* Part One
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_23::*;
let data = include_str!("../input/2023/day23.txt");
assert_eq!(solve_part_01(&input_generator(data)), 2042);
```"#]
#[aoc(day23, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    let mut todo = VecDeque::new();
    let mut seen: HashSet<Point> = HashSet::new();
    let mut most_steps = 0;

    seen.insert(input.start);
    todo.push_back((0, input.start, seen));

    while let Some((steps, pos, seen)) = todo.pop_front() {
        // If we reach the end, save the steps if it is higher
        // than the current highest
        if pos == input.end {
            most_steps = most_steps.max(steps);
            continue;
        }

        if !input.trails.contains(pos) {
            continue;
        }

        let trail = input.trails[pos];

        if trail == b'.' || trail == b'v' {
            let next_pos = pos + DOWN;
            let mut new_seen = seen.clone();

            if new_seen.insert(next_pos) {
                todo.push_back((steps + 1, next_pos, new_seen));
            }
        }

        if trail == b'.' || trail == b'^' {
            let next_pos = pos + UP;
            let mut new_seen = seen.clone();

            if new_seen.insert(next_pos) {
                todo.push_back((steps + 1, next_pos, new_seen));
            }
        }

        if trail == b'.' || trail == b'>' {
            let next_pos = pos + RIGHT;
            let mut new_seen = seen.clone();

            if new_seen.insert(next_pos) {
                todo.push_back((steps + 1, next_pos, new_seen));
            }
        }

        if trail == b'.' || trail == b'<' {
            let next_pos = pos + LEFT;
            let mut new_seen = seen.clone();

            if new_seen.insert(next_pos) {
                todo.push_back((steps + 1, next_pos, new_seen));
            }
        }
    }

    most_steps
}

/* Part Two
*
*
*/
#[aoc(day23, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let mut points_of_interest = HashSet::new();
    let mut edges = HashMap::new();
    let mut result = 0;
    let mut seen = HashSet::new();
    let mut todo = VecDeque::new();

    points_of_interest.insert(input.start);
    points_of_interest.insert(input.end);
    seen.insert(input.start);
    todo.push_back((input.start, seen, 0));

    // Instead of traversing the entire grid, we can find
    // all the junctions and then only traverse those.
    for y in 0..input.trails.height {
        for x in 0..input.trails.width {
            let p = Point::new(x, y);
            if input.trails[p] != b'#' {
                let mut neighbors = 0;

                for o in CARDINALS {
                    let next = p + o;
                    if input.trails.contains(next) && input.trails[next] != b'#' {
                        neighbors += 1;
                    }
                }

                if neighbors > 2 {
                    points_of_interest.insert(p);
                }
            }
        }
    }

    for &start in &points_of_interest {
        edges.insert(start, bfs(&input.trails, &points_of_interest, start));
    }

    while let Some((pos, seen, steps)) = todo.pop_front() {
        if pos == input.end {
            result = result.max(steps);
            continue;
        }

        for &(next, next_steps) in &edges[&pos] {
            if !seen.contains(&next) {
                let mut next_seen = seen.clone();
                next_seen.insert(next);

                todo.push_back((next, next_seen, steps + next_steps));
            }
        }
    }

    result
}

fn bfs(grid: &Grid<u8>, points_of_interest: &HashSet<Point>, start: Point) -> Vec<(Point, usize)> {
    let mut todo = VecDeque::new();
    let mut seen: HashSet<Point> = HashSet::new();
    let mut dist = vec![];

    seen.insert(start);
    todo.push_back((start, 0));

    while let Some((pos, steps)) = todo.pop_front() {
        if pos != start && points_of_interest.contains(&pos) {
            dist.push((pos, steps));
            continue;
        }

        for direction in CARDINALS {
            let next = pos + direction;

            if grid.contains(next) && grid[next] != b'#' && seen.insert(next) {
                todo.push_back((next, steps + 1));
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
        94
    )]
    fn sample_01(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case(
        "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#",
        154
    )]
    fn sample_02(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_02(&input_generator(input)), expected);
    }
}
