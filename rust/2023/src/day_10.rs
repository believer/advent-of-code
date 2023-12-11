//! Day 10: Pipe Maze
//!
//! The first part is done using Breadth-first search (BFS). We start at one point
//! and look in all directions for valid next points. The valid points are added to
//! a queue, and all points are marked as seen. Keep picking of the queue until
//! there are no items left.
//!
//! I had the correct calculations for part 1 for a long time, but I was missing
//! a check for the valid start direction. I was checking all directions
//! around the start. So, I just added the allowed start directions manually to
//! get the right answer. Then I created a real solution for it.
//!
//! Did a refactor of the code using a new grid implementation that combines with my
//! point helper. I think it's a lot cleaner now.
//!
//! Update:
//! I had no idea how to solve part 2. I had to look at the solutions thread for help.
//! I first tried expanding the grid and doing a flood fill, but that turned into a mess.
//!
//! Then I found a clean solution by @maneatingape that uses uses the
//! [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula)
//! and [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem). I had never heard
//! of these before, and I don't completely understand them yet. But I'm happy to have
//! learned something new.
//!
//! Starting at `S` find the loop. Each corner piece (`7`, `F`, `J`, `L` and finally `S`) is
//! considered a vertex and added to the running total for the area using the Shoelace
//! formula. Additionally we keep track of the perimeter length.
//!
//! This solution can be used for both parts and is much faster.
//! But, I like the BFS solution for part 1, so I'm keeping it around.

use crate::{grid::Grid, point::*};
use std::collections::{HashSet, VecDeque};

fn directions(pipe: &u8) -> Vec<Point> {
    match pipe {
        b'|' => vec![UP, DOWN],
        b'-' => vec![LEFT, RIGHT],
        b'L' => vec![UP, RIGHT],
        b'J' => vec![UP, LEFT],
        b'F' => vec![DOWN, RIGHT],
        b'7' => vec![DOWN, LEFT],
        b'S' => vec![UP, RIGHT, DOWN, LEFT],
        b'.' => vec![],
        _ => panic!("Invalid pipe: {}", pipe),
    }
}

// Directions from input
//   -
// L S -
//   F

fn is_valid_start(next_tile: &u8, direction: &Point) -> bool {
    matches!(
        (next_tile, direction),
        (b'-', &RIGHT)
            | (b'-', &LEFT)
            | (b'|', &UP)
            | (b'|', &DOWN)
            | (b'L', &LEFT)
            | (b'L', &DOWN)
            | (b'J', &DOWN)
            | (b'J', &RIGHT)
            | (b'F', &LEFT)
            | (b'F', &UP)
            | (b'7', &RIGHT)
            | (b'7', &UP)
    )
}

#[derive(Debug)]
pub struct Input {
    grid: Grid<u8>,
    start: Point,
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    let grid: Grid<u8> = Grid::from(input);
    let start = grid.find(b'S').unwrap();

    Input { grid, start }
}

/// Find how many points are enclosed by the loop. The loop is a polygon, so we can use
/// the Shoelace formula to calculate the area. Then we use Pick's theorem to find the
/// number of interior points.
/// https://en.wikipedia.org/wiki/Shoelace_formula
/// https://en.wikipedia.org/wiki/Pick%27s_theorem
pub fn solver(input: &Input) -> (i32, i32) {
    let Input { grid, start } = input;
    let determinant = |a: Point, b: Point| a.x * b.y - a.y * b.x;

    // Find start point and direction
    let mut corner = *start;

    // Directions are
    let mut direction = match grid[corner + RIGHT] {
        b'-' | b'J' | b'7' => RIGHT,
        _ => LEFT,
    };
    let mut position = corner + direction;
    let mut steps = 1;
    let mut area = 0;

    loop {
        // Following vertical/horizontal paths
        while grid[position] == b'|' || grid[position] == b'-' {
            position += direction;
            steps += 1;
        }

        // Change direction if we hit a corner
        direction = match grid[position] {
            b'7' if direction == UP => LEFT,
            b'F' if direction == UP => RIGHT,
            b'J' if direction == DOWN => LEFT,
            b'L' if direction == DOWN => RIGHT,
            b'J' | b'L' => UP,
            b'7' | b'F' => DOWN,
            _ => {
                // We're back at the start
                area += determinant(corner, position);
                break;
            }
        };

        area += determinant(corner, position);
        corner = position;
        position += direction;
        steps += 1;
    }

    // Perimeter is the number of steps taken
    // From the perimeter we can calculate the area of the polygon
    // using the Shoelace formula (S).
    //
    // We multiply the X coordinate of each point by the Y coordinate of the next point.
    // Subtract the Y coordinate of each point by the X coordinate of the next point.
    // The last multiplication is the X coordinate of the last point by
    // the Y coordinate of the first point.
    //
    // A = (1/2) * |(x1y2 + x2y3 + ... + xny1) - (y1x2 + y2x3 + ... + yny1)|
    let part_one = steps / 2;

    // Pick's theorem is used to calculate the area of a polygon with points.
    // It's based on the number of interior points (i), our area (A) from the
    // Shoelace formula, and the number of boundary points (b).
    //
    // A = i + b / 2 - 1
    // i = A - b / 2 + 1
    let part_two = area.abs() / 2 - steps / 2 + 1;

    (part_one, part_two)
}

/* Part One
*
* Follow pipes along a grid and find the furthest point from the start.
*
*/
/// Your puzzle answer was
///
/// ```
/// use advent_of_code_2023::day_10::*;
/// let data = include_str!("../input/2023/day10.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 6882);
/// ```
#[aoc(day10, part1)]
pub fn solve_part_01(input: &Input) -> i32 {
    solver(input).0
}

/* Part Two
*
* Find how many points are enclosed by the loop.
*
*/
/// ```
/// use advent_of_code_2023::day_10::*;
/// let data = include_str!("../input/2023/day10.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 491);
/// ```
#[aoc(day10, part2)]
pub fn solve_part_02(input: &Input) -> i32 {
    solver(input).1
}

/// First solution using BFS
///
/// Your puzzle answer was
///
/// ```
/// use advent_of_code_2023::day_10::*;
/// let data = include_str!("../input/2023/day10.txt");
/// assert_eq!(solve_part_01_bfs(&input_generator(data)), 6882);
/// ```
pub fn solve_part_01_bfs(input: &Input) -> u64 {
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
            if !is_valid_start(&grid[new_point], &direction) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        4
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        8
    )]
    fn sample_01(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
        assert_eq!(solve_part_01_bfs(&input_generator(input)), expected as u64);
    }

    #[rstest]
    #[case(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
|-FJL-|7||-",
        4
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        8
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        10
    )]
    fn sample_02(#[case] input: &str, #[case] output: i32) {
        assert_eq!(solve_part_02(&input_generator(input)), output);
    }
}
