use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::*, multi::separated_list1, sequence::separated_pair, IResult,
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
// Cleaned it up. It didn't make it any more efficient, but at least it's
// easier to understand.

type Rocks = BTreeSet<(u32, u32)>;

fn parse_coordinate(input: &str) -> IResult<&str, (u32, u32)> {
    // Parse the coordinates (x,y) from the input.
    separated_pair(complete::u32, complete::char(','), complete::u32)(input)
}

fn parse_path(input: &str) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    // Parse each path coordinate, separated by a " -> ".
    let (input, output) = separated_list1(tag(" -> "), parse_coordinate)(input)?;

    let coordinates = output
        .into_iter()
        .tuple_windows()
        .flat_map(|((ax, ay), (bx, by))| {
            let x_range = ax.min(bx)..=ax.max(bx);
            let y_range = ay.min(by)..=ay.max(by);

            // Use Cartesian product (A × B) to get all coordinates between the two
            // points. I remember this from math at KTH, but I didn't
            // recall it until I googled how to get coordinates between two
            // points.
            //
            // Example:
            // x_range = {498,498}, y_range = {4,5,6}
            // {(498,4), (498,5), (498,6)}
            //
            // It will yield some overlapping coordinates on intersections, but that's
            // fine since we will use a set to store them.
            x_range.cartesian_product(y_range)
        });

    Ok((input, coordinates))
}

fn parse_paths(input: &str) -> IResult<&str, Rocks> {
    let (input, output) = separated_list1(complete::newline, parse_path)(input)?;

    let rocks = output.into_iter().flatten().collect::<Rocks>();

    Ok((input, rocks))
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Rocks {
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

#[derive(Debug)]
pub struct Sand {
    position: (u32, u32),
    obstructed_left: bool,
    obstructed_right: bool,
    obstructed_down: bool,
}

impl Sand {
    fn new(x: u32, y: u32) -> Self {
        Self {
            position: (x, y),
            obstructed_left: false,
            obstructed_right: false,
            obstructed_down: false,
        }
    }

    fn find_obstacles(&mut self, rocks: &Rocks, sand: &BTreeSet<(u32, u32)>) {
        let next_down = (self.position.0, self.position.1 + 1);
        let next_left_down = (self.position.0 - 1, self.position.1 + 1);
        let next_right_down = (self.position.0 + 1, self.position.1 + 1);

        self.obstructed_down = sand.contains(&next_down) || rocks.contains(&next_down);
        self.obstructed_left = sand.contains(&next_left_down) || rocks.contains(&next_left_down);
        self.obstructed_right = sand.contains(&next_right_down) || rocks.contains(&next_right_down);
    }

    fn all_obstructed(&self) -> bool {
        self.obstructed_left && self.obstructed_right && self.obstructed_down
    }

    fn move_down(&mut self) {
        self.position.1 += 1;
    }

    fn move_down_and_left(&mut self) {
        self.position.0 -= 1;
        self.move_down()
    }

    fn move_down_and_right(&mut self) {
        self.position.0 += 1;
        self.move_down()
    }

    fn has_rock_to_the_left(&self, rocks: &Rocks) -> bool {
        rocks.contains(&(self.position.0 - 1, self.position.1))
    }

    fn come_to_rest(&self) -> (u32, u32) {
        (self.position.0, self.position.1 - 1)
    }
}

/* Part One
*/
#[aoc(day14, part1)]
pub fn solve_part_01(rocks: &Rocks) -> usize {
    let mut units_of_sand: BTreeSet<(u32, u32)> = BTreeSet::new();
    let lowest_rock_y = *rocks.iter().map(|(_, y)| y).max().unwrap();

    'falling: loop {
        // Sand always starts falling from (500,0)
        let mut sand = Sand::new(500, 0);

        'sand: loop {
            if rocks.contains(&sand.position) {
                // We hit a rock, but there's open space to the left.
                // We need to move left and start falling again.
                if !sand.has_rock_to_the_left(rocks) {
                    sand.move_down_and_left();
                    continue 'sand;
                }

                // We hit a rock and are at rest.
                units_of_sand.insert(sand.come_to_rest());
                break 'sand;
            }

            // Find obstacles down, down-left and down-right
            sand.find_obstacles(rocks, &units_of_sand);

            // Obstructed down, but not left -> move left
            if sand.obstructed_down && !sand.obstructed_left {
                sand.move_down_and_left();
                continue 'sand;
            }

            // Obstructed down and left, but not right -> move right
            if sand.obstructed_down && !sand.obstructed_right {
                sand.move_down_and_right();
                continue 'sand;
            }

            // All obstructed -> we are at rest
            if sand.all_obstructed() {
                units_of_sand.insert(sand.position);
                break 'sand;
            }

            // When we are falling past the lowest rock, we are done
            if sand.position.1 >= lowest_rock_y {
                break 'falling;
            }

            sand.move_down();
        }
    }

    units_of_sand.len()
}

/* Part Two
*/
#[aoc(day14, part2)]
pub fn solve_part_02(rocks: &Rocks) -> usize {
    let mut rocks = rocks.clone();
    let mut units_of_sand: BTreeSet<(u32, u32)> = BTreeSet::new();

    let lowest_rock_y = *rocks.iter().map(|(_, y)| y).max().unwrap();
    let highest_rock_x = *rocks.iter().map(|(x, _)| x).max().unwrap();

    // A bit of trial and error to find the right range
    // There's probably some better way :D
    for f in 0..highest_rock_x + 120 {
        rocks.insert((f, lowest_rock_y + 2));
    }

    'falling: loop {
        let mut sand = Sand::new(500, 0);

        // We are done when we hit the hole where the sand is falling from
        if units_of_sand.contains(&(500, 0)) {
            break 'falling;
        }

        // See commented version in part 1
        'sand: loop {
            if rocks.contains(&sand.position) {
                if !sand.has_rock_to_the_left(&rocks) {
                    sand.move_down_and_left();
                    continue 'sand;
                }

                units_of_sand.insert(sand.come_to_rest());
                break 'sand;
            }

            sand.find_obstacles(&rocks, &units_of_sand);

            if sand.obstructed_down && !sand.obstructed_left {
                sand.move_down_and_left();
                continue 'sand;
            }

            if sand.obstructed_down && !sand.obstructed_right {
                sand.move_down_and_right();
                continue 'sand;
            }

            if sand.all_obstructed() {
                units_of_sand.insert(sand.position);
                break 'sand;
            }

            sand.move_down();
        }
    }

    units_of_sand.len()
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
