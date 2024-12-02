use crate::math;
use nom::{
    bytes::complete::tag,
    character::*,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

// Day 15 - Beacon Exclusion Zone
//
// For the longest time I couldn't figure out why my solution for the second
// part of this puzzle was wrong. I was using i32, but needed isize or i64.
// I should've figure that the number would be huge when the coordinates were
// in the millions AND I was multiplying them with 4000000. Never trust the
// example data :D

type Coordinate = (isize, isize);
type Sensors = Vec<Sensor>;

#[derive(Debug)]
pub struct Sensor {
    position: Coordinate,
    closest_beacon: Coordinate,
    distance: isize,
}

impl Sensor {
    fn new(position: Coordinate, closest_beacon: Coordinate) -> Self {
        // Calculate Manhattan distance between sensor and beacon
        // manhattan_distance = |x1 - x2| + |y1 - y2|
        let distance = math::manhattan_distance(position, closest_beacon) as isize;

        Sensor {
            position,
            closest_beacon,
            distance,
        }
    }

    // The maximum distance a sensor can reach a beacon to the left
    fn left_limit(&self) -> isize {
        self.position.0 - self.distance
    }

    fn right_limit(&self) -> isize {
        self.position.0 + self.distance
    }

    fn is_inside_range(&self, position: Coordinate) -> bool {
        // Don't include beacons
        if self.closest_beacon == position {
            return false;
        }

        // Check if the position is inside the sensor's range
        self.distance as usize >= math::manhattan_distance(self.position, position)
    }

    fn just_outside_range(&self, min: isize, max: isize) -> impl Iterator<Item = (isize, isize)> {
        let left_bound = (self.position.0 - self.distance - 1).max(min);
        let right_bound = self.position.0.min(max);

        (left_bound..=right_bound).zip(self.position.1..=max)
    }
}

fn parse_sensor(input: &str) -> IResult<&str, Coordinate> {
    let (input, _) = tag("Sensor at ")(input)?;

    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)
    .map(|(input, (x, y))| (input, (x as isize, y as isize)))
}

fn parse_beacon(input: &str) -> IResult<&str, Coordinate> {
    let (input, _) = tag("closest beacon is at ")(input)?;

    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)
    .map(|(input, (x, y))| (input, (x as isize, y as isize)))
}

fn parse_sensor_beacon(input: &str) -> IResult<&str, Sensor> {
    let (input, (sensor, beacon)) = separated_pair(parse_sensor, tag(": "), parse_beacon)(input)?;

    Ok((input, Sensor::new(sensor, beacon)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(complete::newline, parse_sensor_beacon)(input)
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Sensors {
    parse_input(input).unwrap().1
}

fn part1_solver(sensors: &Sensors, y: isize) -> usize {
    // Find the left and right limits of the grid
    let min_x = sensors.iter().map(|s| s.left_limit()).min().unwrap();
    let max_x = sensors.iter().map(|s| s.right_limit()).max().unwrap();

    // Check from left to right for a provided y-coordinate
    // if the sensor is inside the range of a beacon
    (min_x..=max_x)
        .filter(|position| {
            sensors
                .iter()
                .any(|sensor| sensor.is_inside_range((*position, y)))
        })
        .count()
}

fn part2_solver(sensors: &Sensors, max: isize) -> isize {
    // Look just outside of sensor ranges and find the one
    // position that is not in range of any sensor
    // Multiply its x-coordinate by 4000000 and add the y-coordinate
    sensors
        .iter()
        .find_map(|sensor| {
            sensor.just_outside_range(0, max).find_map(|position| {
                sensors
                    .iter()
                    .all(|sensor| !sensor.is_inside_range(position))
                    .then_some(position.0 * 4000000 + position.1)
            })
        })
        .unwrap()
}

/* Part One
*/
#[aoc(day15, part1)]
pub fn solve_part_01(sensors: &Sensors) -> usize {
    part1_solver(sensors, 2000000)
}

/* Part Two
*/
#[aoc(day15, part2)]
pub fn solve_part_02(sensors: &Sensors) -> isize {
    part2_solver(sensors, 4000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn sample_01() {
        assert_eq!(part1_solver(&input_generator(SAMPLE), 10), 26)
    }

    #[test]
    fn sample_02() {
        assert_eq!(part2_solver(&input_generator(SAMPLE), 20), 56000011)
    }
}
