//! Day 18: Lavaduct Lagoon
//!
//! I started by creating a Grid and drawing the path on it. It's usually easier
//! for me to visualize the problem this way. Like we learned from day 10, we can
//! use the Shoelace formula to calculate the area of the polygon. Then we can
//! make use of Pick's theorem to find the number of interior points.
//!
//! Drawing the grid was not great since I didn't know how big the grid would be,
//! and it's just a waste of memory. I converted it to a HashSet to keep track of
//! the points instead. It ran in 23 seconds, which is not great. Of course, I then
//! noticed that I was only using the set to see how many points were in it, so I
//! changed it to a simple step counter. Now it runs in microseconds!
//!
//! The solver is the same for both parts, it's just the parsing that is different.

use crate::point::{Point, DOWN, LEFT, RIGHT, UP};

#[derive(Debug)]
pub struct Input {
    dig_plan: Vec<Direction>,
}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[aoc_generator(day18, part1)]
pub fn input_generator(input: &str) -> Input {
    let dig_plan =
        input
            .lines()
            .map(|line| {
                let data = line.split_whitespace().collect::<Vec<_>>();
                let distance = data[1].parse::<i32>().unwrap();

                match data[0] {
                    "U" => Direction::Up(distance),
                    "D" => Direction::Down(distance),
                    "L" => Direction::Left(distance),
                    "R" => Direction::Right(distance),
                    _ => panic!("Invalid direction"),
                }
            })
            .collect::<Vec<_>>();

    Input { dig_plan }
}

#[aoc_generator(day18, part2)]
pub fn input_generator_part2(input: &str) -> Input {
    let dig_plan = input
        .lines()
        .map(|line| {
            let data = line.split_whitespace().collect::<Vec<_>>();
            let hex = data[2].replace(['(', ')', '#'], "");
            let value = &hex[..5];
            let distance = i32::from_str_radix(value, 16).unwrap();

            match &hex[hex.len() - 1..] {
                "3" => Direction::Up(distance),
                "1" => Direction::Down(distance),
                "2" => Direction::Left(distance),
                "0" => Direction::Right(distance),
                _ => panic!("Invalid direction"),
            }
        })
        .collect::<Vec<_>>();

    Input { dig_plan }
}

fn determinant(a: Point, b: Point) -> i64 {
    a.x as i64 * b.y as i64 - a.y as i64 * b.x as i64
}

/// Like day 10 we can use the Shoelace formula to calculate the area
/// of the polygon. Then we can make use of Pick's theorem
/// to find the number of interior points. We can then add the number of boundary
/// points and interior points to find the number of cubic meters that
/// the lava lagoon will hold.
fn cubic_meters(boundary: i64, area: i64) -> i64 {
    // Pick's theorem to find the number of interior points
    let interior_points = area.abs() / 2 - boundary / 2 + 1;

    // Boundary is the number of dug out tiles from following the dig plan
    boundary + interior_points
}

fn dig(dig_plan: &Vec<Direction>) -> i64 {
    let mut position = Point::new(0, 0);
    let mut area = 0;
    let mut steps = 0;

    for dig in dig_plan {
        let (direction, distance) = match dig {
            Direction::Right(distance) => (RIGHT, *distance),
            Direction::Left(distance) => (LEFT, *distance),
            Direction::Up(distance) => (UP, *distance),
            Direction::Down(distance) => (DOWN, *distance),
        };

        let next_position = position + direction * distance;

        area += determinant(position, next_position);
        position = next_position;
        steps += distance as i64;
    }

    cubic_meters(steps, area)
}

/* Part One
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_18::*;
let data = include_str!("../input/2023/day18.txt");
assert_eq!(solve_part_01(&input_generator(data)), 48652);
```"#]
#[aoc(day18, part1)]
pub fn solve_part_01(input: &Input) -> i64 {
    dig(&input.dig_plan)
}

/* Part Two
*
* Find the maximum number of energized tiles after firing the beam
* inwards from all edges of the grid.
*
*/
#[doc = r#"```
use advent_of_code_2023::day_18::*;
let data = include_str!("../input/2023/day18.txt");
assert_eq!(solve_part_02(&input_generator_part2(data)), 45757884535661);
```"#]
#[aoc(day18, part2)]
pub fn solve_part_02(input: &Input) -> i64 {
    dig(&input.dig_plan)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        62
    )]
    fn sample_01(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case(
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        952408144115
    )]
    fn sample_02(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(solve_part_02(&input_generator_part2(input)), expected);
    }
}
