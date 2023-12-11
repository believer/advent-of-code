//! Day 11:

use std::collections::HashMap;

use crate::{grid::Grid, point::Point};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Tile::Empty,
            b'#' => Tile::Galaxy,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tile::*;

        match self {
            Galaxy => write!(f, "#"),
            Empty => write!(f, "."),
        }
    }
}

pub struct Input {
    grid: Grid<Tile>,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Input {
    let grid: Grid<Tile> = Grid::from(input);
    let empty_rows = grid
        .data
        .chunks(grid.width as usize)
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|&x| x == Tile::Empty) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut cols = vec![];

    for x in 0..grid.width {
        for y in 0..grid.height {
            let tile = grid[Point::new(x, y)];

            cols.push(tile);
        }
    }

    let empty_cols = cols
        .chunks(grid.height as usize)
        .enumerate()
        .filter_map(|(i, col)| {
            if col.iter().all(|&x| x == Tile::Empty) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let galaxy_positions =
        grid.data
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| {
                if *tile == Tile::Galaxy {
                    Some(Point::new((i as i32) % grid.width, (i as i32) / grid.width))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

    let mut expanded_grid: Grid<Tile> = Grid {
        width: grid.width + empty_cols.len() as i32,
        height: grid.height + empty_rows.len() as i32,
        data: vec![],
    };

    for _ in 0..expanded_grid.width {
        for _ in 0..expanded_grid.height {
            expanded_grid.data.push(Tile::Empty);
        }
    }

    // Translate galaxy positions to new grid

    for point in galaxy_positions {
        let empty_cols_below = empty_cols.iter().filter(|&&x| x < point.x as usize).count() as i32;
        let empty_rows_below = empty_rows.iter().filter(|&&y| y < point.y as usize).count() as i32;

        let y = point.y + empty_rows_below;
        let x = point.x + empty_cols_below;

        expanded_grid[Point::new(x, y)] = Tile::Galaxy;
    }

    Input {
        grid: expanded_grid,
    }
}

/* Part One
*
* Follow pipes along a grid and find the furthest point from the start.
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_11::*;
let data = include_str!("../input/2023/day11.txt");
assert_eq!(solve_part_01(&input_generator(data)), 6882);
```"#]
#[aoc(day11, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    let mut paths: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let galaxies = input
        .grid
        .data
        .iter()
        .enumerate()
        .filter_map(|(i, tile)| {
            if *tile == Tile::Galaxy {
                Some(Point::new(
                    (i as i32) % input.grid.width,
                    (i as i32) / input.grid.width,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for (g, galaxy) in galaxies.iter().enumerate() {
        for (og, other_galaxy) in galaxies.iter().enumerate() {
            if galaxy == other_galaxy {
                continue;
            }

            let pair = (g.min(og) + 1, g.max(og) + 1);

            if paths.contains_key(&pair) {
                continue;
            }

            let distance = galaxy.manhattan_distance(other_galaxy);

            paths.entry(pair).or_default().push(distance);
        }
    }

    paths.values().map(|x| x.last().unwrap()).sum()
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
pub fn solve_part_02(input: &Input) -> u64 {
    0
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
}
