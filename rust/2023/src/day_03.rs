use std::collections::HashSet;

// Day 3: Gear Ratios
//
// I had a hard time wrapping my head around this one. I knew how to find surrounding points,
// but couldn't come up with a good idea for the numbers. During a car ride home from my parents,
// I realized that I could maybe look at it another way. Find all the points where we
// have a symbol, find all the numbers and which points they occupy. Then, for each number,
// check if any of the points around it are symbols.

type Point = (i32, i32);

const UP: Point = (0, -1);
const DOWN: Point = (0, 1);
const LEFT: Point = (-1, 0);
const RIGHT: Point = (1, 0);
const DIAGONALS: [Point; 8] = [UP, (1, -1), RIGHT, (1, 1), DOWN, (-1, 1), LEFT, (-1, -1)];

pub struct Input {
    gears: HashSet<Point>,
    symbols: HashSet<Point>,
    numbers: Vec<(u32, Vec<Point>)>,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    let mut symbols: HashSet<Point> = HashSet::new();
    let mut gears: HashSet<Point> = HashSet::new();
    let mut current_number = 0;
    let mut current_points = vec![];
    let mut numbers = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, value) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;

            // If the value is a number, build it up until we hit a non-number
            if value.is_ascii_digit() {
                current_number = current_number * 10 + (value as u32 - b'0' as u32);
                current_points.push((x, y));

                continue;
            }

            match value {
                // Ignore empty spaces
                '.' => (),
                // * indicates a symbol in part 1 and a potential gear in part 2
                '*' => {
                    symbols.insert((x, y));
                    gears.insert((x, y));
                }
                // Every other character is a symbol
                _ => {
                    symbols.insert((x, y));
                }
            };

            // If we have a number, add it to the list of numbers
            if current_number > 0 {
                numbers.push((current_number, current_points.clone()));
                current_number = 0;
                current_points.clear();
            }
        }
    }

    Input {
        symbols,
        numbers,
        gears,
    }
}

/* Part One
*
* Find all the numbers that have a symbol in one of the 8 directions around them.
* Add all the numbers together.
*
* 467..114..
* ...*......
* ..35..633.
* ......#...
* 617*......
* .....+.58.
* ..592.....
* ......755.
* ...$.*....
* .664.598..
*
* All numbers, except 114 and 58, have an adjacent symbol.
* The sum of these numbers is 467 + 633 + 617 + 592 + 755 + 664 + 598 = 4361
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_03::*;
/// let data = include_str!("../input/2023/day3.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 535235);
/// ```
#[aoc(day3, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    let Input {
        symbols, numbers, ..
    } = input;
    let mut part_numbers = vec![];

    for (number, points) in numbers {
        'points: for (x, y) in points {
            for check_point in DIAGONALS.iter().map(|(dx, dy)| (dx + x, dy + y)) {
                // If the point is in the list of symbols, it's a "part number"
                if symbols.contains(&check_point) {
                    part_numbers.push(*number);

                    // We don't need to check any more points for this number
                    break 'points;
                }
            }
        }
    }

    part_numbers.iter().sum()
}

/* Part Two
*
* The symbol * indicates a gear. Find the two numbers connected to each gear and
* multiply them together. Add all the results together.
*
* If a gear only has one number connected to it, ignore it.
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_03::*;
/// let data = include_str!("../input/2023/day3.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 79844424);
/// ```
#[aoc(day3, part2)]
pub fn solve_part_02(input: &Input) -> u32 {
    let Input { gears, numbers, .. } = input;
    let mut gear_ratios = 0;

    for (x, y) in gears {
        let mut last_found_points = vec![];
        let mut adjacents = vec![];

        for check_point in DIAGONALS.iter().map(|(dx, dy)| (x + dx, y + dy)) {
            for (number, points) in numbers {
                // If the point is in the list of points and we haven't already found
                // this number, add it to the list of adjacents
                if points.contains(&check_point) && !last_found_points.contains(&check_point) {
                    adjacents.push(*number);
                    last_found_points = points.clone();
                }
            }
        }

        if adjacents.len() > 1 {
            gear_ratios += adjacents[0] * adjacents[1];
        }
    }

    gear_ratios
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve_part_01(&input_generator(data)), 4361)
    }

    #[test]
    fn sample_02() {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve_part_02(&input_generator(data)), 467835)
    }
}
