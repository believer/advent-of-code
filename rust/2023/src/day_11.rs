//! Day 11: Cosmic Expansion
//!
//! I was creating a new expanded grid and transposing the galaxies to the new grid.
//! This worked fine for part 1, where the expansion rate was 2, but for part 2, it
//! broke down. The grid size became something like 7_000_000 * 9_000_000, which
//! was too big to handle. Of course, I just needed the galaxies, not the whole grid.
//! So I changed the code to just calculate the distance between the galaxies and
//! multiply it by the expansion rate.

use crate::{grid::Grid, point::Point};

pub struct Input {
    galaxies: Vec<Point>,
    empty_rows: Vec<i32>,
    empty_columns: Vec<i32>,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Input {
    let grid: Grid<u8> = Grid::from(input);
    let galaxies =
        grid.data
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| {
                if *tile == b'#' {
                    Some(Point::new((i as i32) % grid.width, (i as i32) / grid.width))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

    // Get outer most galaxy position
    let max_x = galaxies.iter().map(|gp| gp.x).max().unwrap();
    let max_y = galaxies.iter().map(|gp| gp.y).max().unwrap();

    // Find empty columns and rows
    let empty_columns = (0..max_x)
        .filter(|&x| galaxies.iter().all(|gp| gp.x != x))
        .collect::<Vec<_>>();

    let empty_rows = (0..max_y)
        .filter(|&y| galaxies.iter().all(|gp| gp.y != y))
        .collect::<Vec<_>>();

    Input {
        galaxies,
        empty_rows,
        empty_columns,
    }
}

fn should_expand(value: &i32, first: i32, second: i32) -> bool {
    let max = first.max(second);
    let min = first.min(second);

    (min..max).contains(value)
}

fn galaxy_distance(input: &Input, expansion_rate: usize) -> usize {
    let mut path_sum = 0;

    for (i, galaxy) in input.galaxies.iter().enumerate() {
        for other_galaxy in input.galaxies.iter().skip(i + 1) {
            let empty_cols_count = input
                .empty_columns
                .iter()
                .filter(|c| should_expand(c, other_galaxy.x, galaxy.x))
                .count();
            let empty_rows_count = input
                .empty_rows
                .iter()
                .filter(|c| should_expand(c, other_galaxy.y, galaxy.y))
                .count();

            // Calculate how much the galaxies have moved away from each other
            // Expansion rate is decreased because one rows/columns is already
            // empty, i.e., 1 empty row with double expansion rate = 2 empty rows
            let expansion = (empty_cols_count + empty_rows_count) * (expansion_rate - 1);
            let distance = galaxy.manhattan_distance(other_galaxy) as usize + expansion;

            path_sum += distance;
        }
    }

    path_sum
}

/* Part One
*
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_11::*;
let data = include_str!("../input/2023/day11.txt");
assert_eq!(solve_part_01(&input_generator(data)), 6882);
```"#]
#[aoc(day11, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    galaxy_distance(input, 2)
}

/* Part Two
*
*
*/
/*
#[doc = r#"```
use advent_of_code_2023::day_11::*;
let data = include_str!("../input/2023/day11.txt");
assert_eq!(solve_part_02(&input_generator(data)), 0);
```"#]
*/
#[aoc(day11, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    galaxy_distance(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 374);
    }

    #[test]
    fn text_galaxy_distance() {
        let map = input_generator(DATA);

        assert_eq!(galaxy_distance(&map, 2), 374);
        assert_eq!(galaxy_distance(&map, 10), 1030);
        assert_eq!(galaxy_distance(&map, 100), 8410);
    }
}
