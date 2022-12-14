use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};
use std::collections::BTreeSet;

// Day 14 - Regolith Reservoir
//
// Was quick to parse the input using nom. I'm liking it more and more.
// Handling the actual solution was harder, but fun! I kept testing small
// changes, but it got easier after I created the function to draw the current
// state (actually Copilot created a lot of it :O). I work better with visuals.
// I the the solution unit by unit for the example data until it worked.
//
// Will need to clean up later.

type Input = Vec<Vec<(u32, u32)>>;

fn parse_coordinate(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, output) = separated_pair(
        nom::character::complete::u32,
        tag(","),
        nom::character::complete::u32,
    )(input)?;

    Ok((input, output))
}

fn parse_path(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, output) = separated_list1(tag(" -> "), parse_coordinate)(input)?;
    Ok((input, output))
}

fn parse_paths(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    let (input, output) = separated_list1(newline, parse_path)(input)?;

    Ok((input, output))
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Input {
    parse_paths(input).unwrap().1
}

// Keep around to draw the current state
fn _draw_state(rocks: &BTreeSet<(u32, u32)>, sand: &BTreeSet<(u32, u32)>) {
    let min_x = *rocks.iter().map(|(x, _)| x).min().unwrap();
    let max_x = *rocks.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *rocks.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=max_y + 2 {
        for x in min_x..=max_x {
            if rocks.contains(&(x, y)) {
                print!("#");
            } else if sand.contains(&(x, y)) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_14::*;
/// let data = include_str!("../input/2022/day14.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 737);
/// ```
#[aoc(day14, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    let mut rocks: BTreeSet<(u32, u32)> = BTreeSet::new();
    let mut sand: BTreeSet<(u32, u32)> = BTreeSet::new();
    let sand_start = (500, 0);

    for path in input.iter() {
        let mut peekable = path.iter().peekable();

        while let Some((x, y)) = peekable.next() {
            if let Some((x2, y2)) = peekable.peek() {
                if x == x2 {
                    for y in *y.min(y2)..=*y.max(y2) {
                        rocks.insert((*x, y));
                    }
                } else {
                    for x in *x.min(x2)..=*x.max(x2) {
                        rocks.insert((x, *y));
                    }
                }
            }
        }
    }

    let lowest_rock_y = *rocks.iter().map(|(_, y)| y).max().unwrap();
    let lowest_rock_x = *rocks.iter().map(|(x, _)| x).min().unwrap();
    let highest_rock_x = *rocks.iter().map(|(x, _)| x).max().unwrap();
    for f in lowest_rock_x - 50..highest_rock_x + 50 {
        rocks.insert((f, lowest_rock_y + 2));
    }

    'falling: loop {
        let (mut x, mut y) = sand_start;

        'sand: loop {
            if rocks.contains(&(x, y)) {
                if !rocks.contains(&(x - 1, y)) {
                    x -= 1;
                    y += 1;
                    continue 'sand;
                }

                sand.insert((x, y - 1));
                break 'sand;
            }

            let obs_down = sand.contains(&(x, y + 1)) || rocks.contains(&(x, y + 1));
            let obs_left = sand.contains(&(x - 1, y + 1)) || rocks.contains(&(x - 1, y + 1));
            let obs_right = sand.contains(&(x + 1, y + 1)) || rocks.contains(&(x + 1, y + 1));

            if obs_down && !obs_left {
                x -= 1;
                y += 1;
                continue 'sand;
            }

            if obs_down && !obs_right {
                x += 1;
                y += 1;
                continue 'sand;
            }

            if obs_down && obs_left && obs_right {
                sand.insert((x, y));
                break 'sand;
            }

            if y >= lowest_rock_y {
                break 'falling;
            }

            y += 1;
        }
    }

    sand.len()
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_14::*;
/// let data = include_str!("../input/2022/day14.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 28145);
/// ```
#[aoc(day14, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let mut rocks: BTreeSet<(u32, u32)> = BTreeSet::new();
    let mut sand: BTreeSet<(u32, u32)> = BTreeSet::new();
    let sand_start = (500, 0);

    for path in input.iter() {
        let mut peekable = path.iter().peekable();

        while let Some((x, y)) = peekable.next() {
            if let Some((x2, y2)) = peekable.peek() {
                if x == x2 {
                    for y in *y.min(y2)..=*y.max(y2) {
                        rocks.insert((*x, y));
                    }
                } else {
                    for x in *x.min(x2)..=*x.max(x2) {
                        rocks.insert((x, *y));
                    }
                }
            }
        }
    }

    let lowest_rock_y = *rocks.iter().map(|(_, y)| y).max().unwrap();
    let highest_rock_x = *rocks.iter().map(|(x, _)| x).max().unwrap();

    // A bit of trial and error to find the right range
    // There's probably some better way :D
    for f in 0..highest_rock_x + 120 {
        rocks.insert((f, lowest_rock_y + 2));
    }

    'falling: loop {
        let (mut x, mut y) = sand_start;

        // Break when we hit the hole
        if sand.contains(&(500, 0)) {
            break 'falling;
        }

        'sand: loop {
            if rocks.contains(&(x, y)) {
                if !rocks.contains(&(x - 1, y)) {
                    x -= 1;
                    y += 1;
                    continue 'sand;
                }

                sand.insert((x, y - 1));
                break 'sand;
            }

            let obs_down = sand.contains(&(x, y + 1)) || rocks.contains(&(x, y + 1));
            let obs_left = sand.contains(&(x - 1, y + 1)) || rocks.contains(&(x - 1, y + 1));
            let obs_right = sand.contains(&(x + 1, y + 1)) || rocks.contains(&(x + 1, y + 1));

            if obs_down && !obs_left {
                x -= 1;
                y += 1;
                continue 'sand;
            }

            if obs_down && !obs_right {
                x += 1;
                y += 1;
                continue 'sand;
            }

            if obs_down && obs_left && obs_right {
                sand.insert((x, y));
                break 'sand;
            }

            y += 1;
        }
    }

    sand.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 24)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 93)
    }
}
