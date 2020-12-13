use std::collections::BTreeMap;

// Day 13

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (u32, Vec<u32>) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|&b| b != "x")
        .map(|b| b.parse().unwrap())
        .collect::<Vec<u32>>();

    (timestamp, buses)
}

/// Your puzzle answer was
/// ```
/// use advent_of_code_2020::day_13::*;
/// let input = include_str!("../input/2020/day13.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 3246);
#[aoc(day13, part1)]
pub fn solve_part_01((timestamp, buses): &(u32, Vec<u32>)) -> u32 {
    let mut timetable: BTreeMap<u32, u32> = BTreeMap::new();

    for bus in buses {
        let mut time = 0;

        while time <= *timestamp + bus {
            timetable.insert(time, *bus);
            time += bus;
        }
    }

    let (time, bus) = timetable.range(timestamp..).next().unwrap();
    bus * (time - timestamp)
}

// /// Your puzzle answer was
// /// ```
// /// use advent_of_code_2020::day_13::*;
// /// let input = include_str!("../input/2020/day13.txt");
// /// assert_eq!(solve_part_02(&input_generator(input)), 20592);
// /// ```
// #[aoc(day13, part2)]
// pub fn solve_part_02(_input: &[String]) -> u32 {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    // Test example data on part 1
    #[test]
    fn test_example_part_1() {
        let data = "939
7,13,x,x,59,x,31,19";

        assert_eq!(solve_part_01(&input_generator(data)), 295)
    }
}
