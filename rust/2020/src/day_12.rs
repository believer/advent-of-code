use lazy_static::lazy_static;
use regex::Regex;

// Day 12 - Rain Risk

lazy_static! {
    static ref RE: Regex = Regex::new(r"(N|S|E|W|L|R|F)(\d{1,3})").unwrap();
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

impl Instruction {
    fn as_point(&self) -> (i32, i32) {
        match self {
            Instruction::East => (1, 0),
            Instruction::South => (0, -1),
            Instruction::West => (-1, 0),
            Instruction::North => (0, 1),
            _ => unreachable!(),
        }
    }

    fn new(point: (i32, i32)) -> Instruction {
        match point {
            (1, 0) => Instruction::East,
            (0, -1) => Instruction::South,
            (-1, 0) => Instruction::West,
            (0, 1) => Instruction::North,
            _ => unreachable!(),
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "N" => Instruction::North,
            "E" => Instruction::East,
            "S" => Instruction::South,
            "W" => Instruction::West,
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            "F" => Instruction::Forward,
            _ => unreachable!("Invalid instruction"),
        })
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<(Instruction, i32)> {
    input
        .lines()
        .map(|l| {
            let caps = RE.captures(l).unwrap();
            let instruction = caps.get(1).unwrap().as_str().parse().unwrap();
            let amount = caps.get(2).unwrap().as_str().parse().unwrap();

            (instruction, amount)
        })
        .collect()
}

fn manhattan_distance(x: i32, y: i32) -> u32 {
    (x.abs() + y.abs()) as u32
}

/// 2D rotation of a point
fn rotation_2d(x: i32, y: i32, angle: i32) -> (i32, i32) {
    let radians = angle as f64 * std::f64::consts::PI / 180 as f64;

    let nx = (x as f64 * radians.cos() - y as f64 * radians.sin()).round();
    let ny = (y as f64 * radians.cos() + x as f64 * radians.sin()).round();

    (nx as i32, ny as i32)
}

fn turn(facing: Instruction, angle: i32) -> Instruction {
    let (x, y) = facing.as_point();
    Instruction::new(rotation_2d(x, y, angle))
}

/* Part One
 *
 * Your ferry made decent progress toward the island, but the storm came in faster than anyone expected.
 * The ferry needs to take evasive actions!
 *
 * Unfortunately, the ship's navigation computer seems to be malfunctioning;
 * rather than giving a route directly to safety, it produced extremely circuitous instructions.
 * When the captain uses the PA system to ask if anyone can help, you quickly volunteer.
 *
 * The navigation instructions (your puzzle input) consists of a sequence of single-character actions
 * paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:
 *
 * Action N means to move north by the given value.
 * Action S means to move south by the given value.
 * Action E means to move east by the given value.
 * Action W means to move west by the given value.
 * Action L means to turn left the given number of degrees.
 * Action R means to turn right the given number of degrees.
 * Action F means to move forward by the given value in the direction the ship is currently facing.
 *
 * The ship starts by facing east. Only the L and R actions change the direction the ship is facing.
 * (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units,
 * but would still move east if the following action were F.)
 *
 * For example:
 *
 * F10
 * N3
 * F7
 * R90
 * F11
 *
 * These instructions would be handled as follows:
 *
 * F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
 * N3 would move the ship 3 units north to east 10, north 3.
 * F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
 * R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
 * F11 would move the ship 11 units south to east 17, south 8.
 *
 * At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of
 * its east/west position and its north/south position) from its starting position is 17 + 8 = 25.
 *
 * Figure out where the navigation instructions lead. What is the Manhattan distance
 * between that location and the ship's starting position?
 */
/// Your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_12::*;
/// let input = include_str!("../input/2020/day12.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 1294);
#[aoc(day12, part1)]
pub fn solve_part_01(instructions: &[(Instruction, i32)]) -> u32 {
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut facing = Instruction::East;

    for (instruction, a) in instructions {
        match instruction {
            Instruction::Forward => match facing {
                Instruction::East => x += a,
                Instruction::West => x -= a,
                Instruction::North => y += a,
                Instruction::South => y -= a,
                _ => unreachable!("Not a valid facing direction"),
            },
            Instruction::Right => facing = turn(facing, -a),
            Instruction::Left => facing = turn(facing, *a),
            Instruction::North => y += a,
            Instruction::South => y -= a,
            Instruction::West => x -= a,
            Instruction::East => x += a,
        }
    }

    manhattan_distance(x, y)
}

/* Part Two
 *
 * Before you can give the destination to the captain, you realize that the
 * actual action meanings were printed on the back of the instructions the whole time.
 *
 * Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:
 *
 * Action N means to move the waypoint north by the given value.
 * Action S means to move the waypoint south by the given value.
 * Action E means to move the waypoint east by the given value.
 * Action W means to move the waypoint west by the given value.
 * Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
 * Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
 * Action F means to move forward to the waypoint a number of times equal to the given value.
 *
 * The waypoint starts 10 units east and 1 unit north relative to the ship.
 * The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.
 *
 * For example, using the same instructions as above:
 *
 * F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north),
 * leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
 * N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
 * F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north),
 * leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
 * R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and
 * 10 units south of the ship. The ship remains at east 170, north 38.
 * F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south),
 * leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
 *
 * After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.
 *
 * Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?
*/
/// Your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_12::*;
/// let input = include_str!("../input/2020/day12.txt");
/// assert_eq!(solve_part_02(&input_generator(input)), 20592);
/// ```
#[aoc(day12, part2)]
pub fn solve_part_02(instructions: &[(Instruction, i32)]) -> u32 {
    let (mut wx, mut wy): (i32, i32) = (10, 1);
    let (mut x, mut y): (i32, i32) = (0, 0);

    for (instruction, a) in instructions {
        match instruction {
            Instruction::Forward => {
                x += wx * a;
                y += wy * a;
            }
            Instruction::Right => {
                let (nx, ny) = rotation_2d(wx, wy, -*a);
                wx = nx;
                wy = ny;
            }
            Instruction::Left => {
                let (nx, ny) = rotation_2d(wx, wy, *a);
                wx = nx;
                wy = ny;
            }
            Instruction::North => wy += a,
            Instruction::South => wy -= a,
            Instruction::East => wx += a,
            Instruction::West => wx -= a,
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
F11";

        assert_eq!(solve_part_01(&input_generator(data)), 25)
    }

    /// Test example data on part 2
    #[test]
    fn test_example_part_2() {
        let data = "F10
    N3
    F7
    R90
    F11";

        assert_eq!(solve_part_02(&input_generator(data)), 286)
    }
}
