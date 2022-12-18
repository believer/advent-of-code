use nom::{bytes::complete::tag, multi::separated_list1, *};
use std::collections::{HashSet, VecDeque};

// Day 18 - Boiling Boulders
//
// Finally something a bit easier :D First part is checking how many neighbors
// a cube has, i.e., how many surfaces aren't covered.
//
// The second part was a bit
// trickier. I had a solution that worked for the example, but not for the
// actual input. I got some tips and found a bunch of people doing something called
// "flood fill". I looked it up and it was basically creating a box around all
// the points and then filling it with "water". Like the fill tool in Photoshop.
// Then we can count the number of cubes that are neighboring the
// water to get the external surface.

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Cube { x, y, z }
    }

    fn neighbors(&self) -> Vec<Cube> {
        vec![
            // X
            Cube::new(self.x - 1, self.y, self.z),
            Cube::new(self.x + 1, self.y, self.z),
            // Y
            Cube::new(self.x, self.y - 1, self.z),
            Cube::new(self.x, self.y + 1, self.z),
            // Z
            Cube::new(self.x, self.y, self.z - 1),
            Cube::new(self.x, self.y, self.z + 1),
        ]
    }
}

type Input = Vec<Cube>;

fn parse_coordinate(input: &str) -> IResult<&str, Cube> {
    let (input, x) = character::complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = character::complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = character::complete::i32(input)?;

    Ok((input, Cube::new(x, y, z)))
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    separated_list1(character::complete::newline, parse_coordinate)(input)
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Input {
    parse_input(input).unwrap().1
}

/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_18::*;
/// let data = include_str!("../input/2022/day18.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 4244);
/// ```
#[aoc(day18, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    input
        .iter()
        .map(|cube| {
            cube.neighbors()
                .iter()
                .filter(|neighbor| !input.contains(neighbor))
                .count()
        })
        .sum()
}

/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_18::*;
/// let data = include_str!("../input/2022/day18.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 2460);
/// ```
#[aoc(day18, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let min_x = input.iter().map(|cube| cube.x).min().unwrap();
    let max_x = input.iter().map(|cube| cube.x).max().unwrap();
    let min_y = input.iter().map(|cube| cube.y).min().unwrap();
    let max_y = input.iter().map(|cube| cube.y).max().unwrap();
    let min_z = input.iter().map(|cube| cube.z).min().unwrap();
    let max_z = input.iter().map(|cube| cube.z).max().unwrap();
    let x_range = min_x - 1..=max_x + 1;
    let y_range = min_y - 1..=max_y + 1;
    let z_range = min_z - 1..=max_z + 1;

    // Use flood fill to surround the area with water
    // in each direction. Then flood-fill the area with water.
    let mut water = HashSet::new();
    let mut queue = VecDeque::new();

    let start_point = Cube::new(*x_range.start(), *y_range.start(), *z_range.start());

    queue.push_back(start_point);

    while let Some(cube) = queue.pop_front() {
        let Cube { x, y, z } = cube;

        if x_range.contains(&x) && y_range.contains(&y) && z_range.contains(&z) {
            for neighbor in cube.neighbors() {
                if !input.contains(&neighbor) && !water.contains(&neighbor) {
                    queue.push_back(neighbor);
                    water.insert(neighbor);
                }
            }
        }
    }

    // Filter out any cubes that touch the water since they will
    // be the external surface of the lava droplets.
    input
        .iter()
        .map(|cube| {
            cube.neighbors()
                .iter()
                .filter(|x| water.contains(x))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 64)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 58)
    }
}
