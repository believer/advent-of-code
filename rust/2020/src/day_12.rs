use crate::{common, math};

// Day 12 - Rain Risk

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        let amount = s[1..].parse().unwrap();

        Ok(match &s[..1] {
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

#[derive(Debug, PartialEq, Eq)]
struct Ship {
    x: i32,
    y: i32,
    angle: i32,
}

impl Ship {
    fn north(&mut self, step: &i32) {
        self.y += step;
    }

    fn south(&mut self, step: &i32) {
        self.y -= step;
    }

    fn east(&mut self, step: &i32) {
        self.x += step;
    }

    fn west(&mut self, step: &i32) {
        self.x -= step;
    }

    fn turn(&mut self, angle: &i32) {
        self.angle += angle
    }

    fn forward(&mut self, distance: &i32) {
        let radians = math::degrees_to_radians(self.angle);

        self.x = (radians.cos() * *distance as f32 + self.x as f32).round() as i32;
        self.y = (radians.sin() * *distance as f32 + self.y as f32).round() as i32;
    }

    fn move_by_waypoint(&mut self, waypoint: &Waypoint, value: &i32) {
        self.x += waypoint.x * value;
        self.y += waypoint.y * value;
    }

    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            angle: 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn north(&mut self, step: &i32) {
        self.y += step;
    }

    fn south(&mut self, step: &i32) {
        self.y -= step;
    }

    fn east(&mut self, step: &i32) {
        self.x += step;
    }

    fn west(&mut self, step: &i32) {
        self.x -= step;
    }

    fn rotate(&mut self, angle: &i32) {
        let (nx, ny) = math::rotation_2d(self.x, self.y, *angle);

        self.x = nx;
        self.y = ny;
    }

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    common::input_vec(input)
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
#[aoc(day12, part1)]
pub fn solve_part_01(instructions: &[Instruction]) -> u32 {
    let mut ship = Ship::new();

    for instruction in instructions {
        match instruction {
            Instruction::North(x) => ship.north(x),
            Instruction::South(x) => ship.south(x),
            Instruction::West(x) => ship.west(x),
            Instruction::East(x) => ship.east(x),
            Instruction::Right(x) => ship.turn(&-x),
            Instruction::Left(x) => ship.turn(x),
            Instruction::Forward(x) => ship.forward(x),
        }
    }

    math::manhattan_distance(ship.x, ship.y)
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
#[aoc(day12, part2)]
pub fn solve_part_02(instructions: &[Instruction]) -> u32 {
    let mut waypoint = Waypoint::new(10, 1);
    let mut ship = Ship::new();

    for instruction in instructions {
        match instruction {
            Instruction::Forward(x) => ship.move_by_waypoint(&waypoint, x),
            Instruction::Right(x) => waypoint.rotate(&-x),
            Instruction::Left(x) => waypoint.rotate(x),
            Instruction::North(x) => waypoint.north(x),
            Instruction::South(x) => waypoint.south(x),
            Instruction::East(x) => waypoint.east(x),
            Instruction::West(x) => waypoint.west(x),
        }
    }

    math::manhattan_distance(ship.x, ship.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test example data on part 1
    #[test]
    fn test_example_part_1() {
        let data = "F10
N3
F7
R90
F11";

        assert_eq!(solve_part_01(&input_generator(data)), 25)
    }

    // Test example data on part 2
    #[test]
    fn test_example_part_2() {
        let data = "F10
    N3
    F7
    R90
    F11";

        assert_eq!(solve_part_02(&input_generator(data)), 286)
    }

    #[test]
    fn create_ship() {
        assert_eq!(
            Ship::new(),
            Ship {
                x: 0,
                y: 0,
                angle: 0
            }
        )
    }

    #[test]
    fn move_ship_north() {
        let mut ship = Ship::new();
        ship.north(&1);

        assert_eq!(
            ship,
            Ship {
                x: 0,
                y: 1,
                angle: 0
            }
        )
    }

    #[test]
    fn move_ship_south() {
        let mut ship = Ship::new();
        ship.south(&1);

        assert_eq!(
            ship,
            Ship {
                x: 0,
                y: -1,
                angle: 0
            }
        )
    }

    #[test]
    fn move_ship_east() {
        let mut ship = Ship::new();
        ship.east(&1);

        assert_eq!(
            ship,
            Ship {
                x: 1,
                y: 0,
                angle: 0
            }
        )
    }

    #[test]
    fn move_ship_west() {
        let mut ship = Ship::new();
        ship.west(&1);

        assert_eq!(
            ship,
            Ship {
                x: -1,
                y: 0,
                angle: 0
            }
        )
    }

    #[test]
    fn turn_ship() {
        let mut ship = Ship::new();
        ship.turn(&90);

        assert_eq!(
            ship,
            Ship {
                x: 0,
                y: 0,
                angle: 90
            }
        )
    }

    #[test]
    fn move_ship() {
        let mut ship = Ship::new();
        ship.forward(&10);

        assert_eq!(
            ship,
            Ship {
                x: 10,
                y: 0,
                angle: 0
            }
        )
    }

    #[test]
    fn move_ship_by_waypoint() {
        let wp = Waypoint::new(10, 4);
        let mut ship = Ship::new();
        ship.move_by_waypoint(&wp, &10);

        assert_eq!(
            ship,
            Ship {
                x: 100,
                y: 40,
                angle: 0
            }
        )
    }

    #[test]
    fn create_waypoint() {
        assert_eq!(Waypoint::new(10, 1), Waypoint { x: 10, y: 1 })
    }

    #[test]
    fn move_waypoint_north() {
        let mut wp = Waypoint::new(0, 0);
        wp.north(&1);

        assert_eq!(wp, Waypoint { x: 0, y: 1 })
    }

    #[test]
    fn move_waypoint_south() {
        let mut wp = Waypoint::new(0, 0);
        wp.south(&1);

        assert_eq!(wp, Waypoint { x: 0, y: -1 })
    }

    #[test]
    fn move_waypoint_east() {
        let mut wp = Waypoint::new(0, 0);
        wp.east(&1);

        assert_eq!(wp, Waypoint { x: 1, y: 0 })
    }

    #[test]
    fn move_waypoint_west() {
        let mut wp = Waypoint::new(0, 0);
        wp.west(&1);

        assert_eq!(wp, Waypoint { x: -1, y: 0 })
    }

    #[test]
    fn rotate_waypoint() {
        let mut wp = Waypoint::new(10, 4);
        wp.rotate(&90);

        assert_eq!(wp, Waypoint { x: -4, y: 10 })
    }
}
