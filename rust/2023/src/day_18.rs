//! Day 18

use std::collections::HashSet;

use crate::{
    grid::Grid,
    point::{Point, DOWN, LEFT, RIGHT, UP},
};

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
pub fn solve_part_01(input: &Input) -> i32 {
    let mut grid = Grid::new(1000, 1000, vec![b'.'; 1000 * 1000]);
    let mut position = Point::new(grid.width / 2, grid.height / 2);
    let mut area = 0;
    let determinant = |a: Point, b: Point| a.x * b.y - a.y * b.x;

    for direction in &input.dig_plan {
        match direction {
            Direction::Right(distance) => {
                for x in position.x + 1..=position.x + *distance {
                    grid.data[(x + position.y * grid.width) as usize] = b'#';
                }

                let next_position = position + RIGHT * *distance;
                area += determinant(position, next_position);
                position = next_position;
            }
            Direction::Left(distance) => {
                for x in (position.x - *distance..=position.x).rev() {
                    grid.data[(x + position.y * grid.width) as usize] = b'#';
                }

                let next_position = position + LEFT * *distance;
                area += determinant(position, next_position);
                position = next_position;
            }
            Direction::Up(distance) => {
                for y in (position.y - *distance..=position.y).rev() {
                    grid.data[(position.x + y * grid.width) as usize] = b'#';
                }

                let next_position = position + UP * *distance;
                area += determinant(position, next_position);
                position = next_position;
            }
            Direction::Down(distance) => {
                for y in position.y..=position.y + *distance {
                    grid.data[(position.x + y * grid.width) as usize] = b'#';
                }

                let next_position = position + DOWN * *distance;
                area += determinant(position, next_position);
                position = next_position;
            }
        }
    }

    // Find the number of boundary points
    let boundary = grid.find_all(b'#').len() as i32;
    // Similar to day 10
    // The Shoelace formula is used to calculate the area of a polygon with points.
    // Make use of Pick's theorem to find the number of interior points
    let interior_points = area.abs() / 2 - boundary / 2 + 1;

    boundary + interior_points
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
assert_eq!(solve_part_02(&input_generator_part2(data)), 1215);
```"#]
#[aoc(day18, part2)]
pub fn solve_part_02(input: &Input) -> i64 {
    let mut points: HashSet<Point> = HashSet::new();
    let mut position = Point::new(0, 0);
    let mut area = 0;
    let determinant = |a: Point, b: Point| a.x as i64 * b.y as i64 - a.y as i64 * b.x as i64;

    for direction in &input.dig_plan {
        match direction {
            Direction::Right(distance) => {
                for x in position.x + 1..=position.x + *distance {
                    points.insert(Point::new(x, position.y));
                }

                let next_position = position + RIGHT * *distance;
                area += determinant(position, next_position);
                position = next_position;
            }
            Direction::Left(distance) => {
                for x in (position.x - *distance..=position.x).rev() {
                    points.insert(Point::new(x, position.y));
                }

                let next_position = position + LEFT * *distance;
                area += determinant(position, next_position);
                position = next_position;
            }
            Direction::Up(distance) => {
                for y in (position.y - *distance..=position.y).rev() {
                    points.insert(Point::new(position.x, y));
                }

                let next_position = position + UP * *distance;
                area += determinant(position, next_position);
                position = next_position;
            }
            Direction::Down(distance) => {
                for y in position.y..=position.y + *distance {
                    points.insert(Point::new(position.x, y));
                }

                let next_position = position + DOWN * *distance;
                area += determinant(position, next_position);
                position = next_position;
            }
        }
    }

    // Find the number of boundary points
    let boundary = points.len() as i64;
    // Similar to day 10
    // The Shoelace formula is used to calculate the area of a polygon with points.
    // Make use of Pick's theorem to find the number of interior points
    let interior_points = area.abs() / 2 - boundary / 2 + 1;

    boundary + interior_points
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
    fn sample_01(#[case] input: &str, #[case] expected: i32) {
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
