//! Day 18

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

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Input {
    let dig_plan = input
        .lines()
        .map(|line| {
            let data = line.split_whitespace().collect::<Vec<_>>();
            let distance = data[1].parse::<i32>().unwrap();

            let direction = match data[0] {
                "U" => Direction::Up(distance),
                "D" => Direction::Down(distance),
                "L" => Direction::Left(distance),
                "R" => Direction::Right(distance),
                _ => panic!("Invalid direction"),
            };

            direction
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
assert_eq!(solve_part_01(&input_generator(data)), 1013);
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
assert_eq!(solve_part_02(&input_generator(data)), 1215);
```"#]
#[aoc(day18, part2)]
pub fn solve_part_02(_input: &Input) -> u32 {
    todo!()
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
}
