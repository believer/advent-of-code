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
//! The answer for part two is the number of interior points. Rearranging Pick's theorem:
//!
//! `A = i + b / 2 - 1`
//! => `i = A - b / 2 + 1`
//!
//! This solution can be used for both parts and is much faster.
//! But, I like the BFS solution for part 1, so I'm keeping it.

use crate::{grid::Grid, point::*};
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

//
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tile::*;

        match self {
            Vertical => write!(f, "|"),
            Horizontal => write!(f, "-"),
            NorthEast => write!(f, "L"),
            NorthWest => write!(f, "J"),
            SouthEast => write!(f, "F"),
            SouthWest => write!(f, "7"),
            Start => write!(f, "S"),
            Ground => write!(f, "."),
        }
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
* Follow pipes along a grid and find the furthest point from the start.
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
* Find how many points are enclosed by the loop.
*
*/
#[aoc(day10, part2)]
pub fn solve_part_02(input: &Input) -> i32 {
    let Input { grid, start } = input;
    let determinant = |a: Point, b: Point| a.x * b.y - a.y * b.x;

    // Find start point and direction
    let mut corner = *start;
    let mut direction = if start.y > 0 {
        match grid[corner + UP] {
            Tile::Vertical | Tile::SouthWest | Tile::SouthEast => UP,
            _ => DOWN,
        }
    } else {
        match grid[corner + DOWN] {
            Tile::Vertical | Tile::NorthWest | Tile::NorthEast => DOWN,
            _ => UP,
        }
    };
    let mut position = corner + direction;
    let mut steps = 1;
    let mut area = 0;

    loop {
        // Following vertical/horizontal paths
        while grid[position] == Tile::Vertical || grid[position] == Tile::Horizontal {
            position += direction;
            steps += 1;
        }

        // Change direction if we hit a corner
        direction = match grid[position] {
            Tile::SouthWest if direction == UP => LEFT,
            Tile::SouthEast if direction == UP => RIGHT,
            Tile::NorthWest if direction == DOWN => LEFT,
            Tile::NorthEast if direction == DOWN => RIGHT,
            Tile::NorthWest | Tile::NorthEast => UP,
            Tile::SouthWest | Tile::SouthEast => DOWN,
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

    area.abs() / 2 - steps / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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
