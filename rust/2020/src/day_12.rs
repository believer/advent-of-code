use crate::common;
use lazy_static::lazy_static;
use regex::Regex;

// Day 12

lazy_static! {
    static ref RE: Regex = Regex::new(r"(N|S|E|W|L|R|F)(\d{1,3})").unwrap();
}

#[derive(Debug)]
pub enum Instruction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = RE.captures(s).unwrap();
        let amount = caps.get(2).unwrap().as_str().parse().unwrap();

        Ok(match caps.get(1).unwrap().as_str() {
            "N" => Instruction::North(amount),
            "E" => Instruction::East(amount),
            "S" => Instruction::South(amount),
            "W" => Instruction::West(amount),
            "L" => Instruction::Left(amount),
            "R" => Instruction::Right(amount),
            "F" => Instruction::Forward(amount),
            _ => unreachable!("Invalid instruction"),
        })
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    common::input_vec(input)
}

fn manhattan_distance(x: i32, y: i32) -> u32 {
    (x.abs() + y.abs()) as u32
}

/// Your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_12::*;
/// let input = include_str!("../input/2020/day12.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 1294);
#[aoc(day12, part1)]
pub fn solve_part_01(instructions: &[Instruction]) -> u32 {
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut facing = Instruction::East(0);

    for instruction in instructions {
        match instruction {
            Instruction::Forward(a) => match facing {
                Instruction::East(_) => x += a,
                Instruction::West(_) => x -= a,
                Instruction::North(_) => y += a,
                Instruction::South(_) => y -= a,
                _ => unreachable!("Not a valid facing direction"),
            },
            Instruction::Right(r) => match facing {
                Instruction::East(_) => {
                    facing = match r {
                        90 => Instruction::South(0),
                        180 => Instruction::West(0),
                        270 => Instruction::North(0),
                        360 => Instruction::East(0),
                        _ => todo!(),
                    }
                }
                Instruction::South(_) => {
                    facing = match r {
                        90 => Instruction::West(0),
                        180 => Instruction::North(0),
                        270 => Instruction::East(0),
                        360 => Instruction::South(0),
                        _ => todo!(),
                    }
                }
                Instruction::North(_) => {
                    facing = match r {
                        90 => Instruction::East(0),
                        180 => Instruction::South(0),
                        270 => Instruction::West(0),
                        360 => Instruction::North(0),
                        _ => todo!(),
                    }
                }
                Instruction::West(_) => {
                    facing = match r {
                        90 => Instruction::North(0),
                        180 => Instruction::East(0),
                        270 => Instruction::South(0),
                        360 => Instruction::West(0),
                        _ => todo!(),
                    }
                }
                _ => unreachable!("Invalid facing"),
            },
            Instruction::Left(r) => match facing {
                Instruction::East(_) => {
                    facing = match r {
                        90 => Instruction::North(0),
                        180 => Instruction::West(0),
                        270 => Instruction::South(0),
                        360 => Instruction::East(0),
                        _ => todo!(),
                    }
                }
                Instruction::South(_) => {
                    facing = match r {
                        90 => Instruction::East(0),
                        180 => Instruction::North(0),
                        270 => Instruction::West(0),
                        360 => Instruction::South(0),
                        _ => todo!(),
                    }
                }
                Instruction::North(_) => {
                    facing = match r {
                        90 => Instruction::West(0),
                        180 => Instruction::South(0),
                        270 => Instruction::East(0),
                        360 => Instruction::North(0),
                        _ => todo!(),
                    }
                }
                Instruction::West(_) => {
                    facing = match r {
                        90 => Instruction::South(0),
                        180 => Instruction::East(0),
                        270 => Instruction::North(0),
                        360 => Instruction::West(0),
                        _ => todo!(),
                    }
                }
                _ => unreachable!("Invalid facing"),
            },
            Instruction::North(a) => y += a,
            Instruction::South(a) => y -= a,
            Instruction::West(a) => x -= a,
            Instruction::East(a) => x += a,
        }
    }

    manhattan_distance(x, y)
}

/// 2D rotation of a point
fn rotation_2d(x: i32, y: i32, angle: i32) -> (i32, i32) {
    let radians = angle as f64 * std::f64::consts::PI / 180 as f64;

    let nx = (x as f64 * radians.cos() - y as f64 * radians.sin()).round();
    let ny = (y as f64 * radians.cos() + x as f64 * radians.sin()).round();

    (nx as i32, ny as i32)
}

/// ```
/// use advent_of_code_2020::day_12::*;
/// let input = include_str!("../input/2020/day12.txt");
/// assert_eq!(solve_part_02(&input_generator(input)), 20592);
/// ```
#[aoc(day12, part2)]
pub fn solve_part_02(instructions: &[Instruction]) -> u32 {
    let (mut wx, mut wy): (i32, i32) = (10, 1);
    let (mut x, mut y): (i32, i32) = (0, 0);

    for instruction in instructions {
        match instruction {
            Instruction::Forward(a) => {
                x += wx * a;
                y += wy * a;
            }
            Instruction::Right(angle) => {
                let (nx, ny) = rotation_2d(wx, wy, -*angle);
                wx = nx;
                wy = ny;
            }
            Instruction::Left(angle) => {
                let (nx, ny) = rotation_2d(wx, wy, *angle);
                wx = nx;
                wy = ny;
            }
            Instruction::North(i) => wy += i,
            Instruction::South(i) => wy -= i,
            Instruction::East(i) => wx += i,
            Instruction::West(i) => wx -= i,
        }
    }

    manhattan_distance(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn test_example_part_1() {
        let data = "F10
N3
F7
R90
F11
";

        assert_eq!(solve_part_01(&input_generator(data)), 25)
    }

    /// Test example data on part 2
    #[test]
    fn test_example_part_2() {
        let data = "F10
N3
F7
R90
F11
";

        assert_eq!(solve_part_02(&input_generator(data)), 286)
    }
}
