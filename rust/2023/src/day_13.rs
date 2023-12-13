//! Day 13: Point of Incidence
//!
//! I had a super hard time understanding the problem description. I first thought that there
//! was just line (vertical or horizontal) in each pattern. Had to read and re-read it
//! multiple times before I understood.
//!
//! Haven't cleaned up part 2 yet, but it works. I'll get back to it later.

use crate::grid::Grid;
use crate::point::Point;

#[derive(Debug)]
pub struct Input {
    patterns: Vec<(Grid<u8>, Vec<Point>)>,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    let patterns = input
        .split("\n\n")
        .map(|pattern| {
            let grid: Grid<u8> = Grid::from(pattern);
            let rocks = grid.find_all(b'#');

            (grid, rocks)
        })
        .collect::<Vec<_>>();

    Input { patterns }
}

fn vertical_line(grid: &Grid<u8>, rocks: &[Point]) -> i32 {
    let result = (1..grid.width).find(|&x| {
        rocks.iter().all(|rock| {
            let reflected = Point::new(2 * x - rock.x - 1, rock.y);
            !grid.contains(reflected) || grid[reflected] != b'.'
        })
    });

    result.unwrap_or(0)
}

fn horizontal_line(grid: &Grid<u8>, rocks: &[Point]) -> i32 {
    let result = (1..grid.height).find(|&x| {
        rocks.iter().all(|rock| {
            let reflected = Point::new(rock.x, 2 * x - rock.y - 1);
            !grid.contains(reflected) || grid[reflected] != b'.'
        })
    });

    result.map(|x| x * 100).unwrap_or(0)
}

/* Part One
*
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_13::*;
let data = include_str!("../input/2023/day13.txt");
assert_eq!(solve_part_01(&input_generator(data)), 27300);
```"#]
#[aoc(day13, part1)]
pub fn solve_part_01(input: &Input) -> i32 {
    input
        .patterns
        .iter()
        .map(|(grid, rocks)| vertical_line(grid, rocks) + horizontal_line(grid, rocks))
        .sum()
}

/* Part Two
*
*
*/
#[doc = r#"```
use advent_of_code_2023::day_13::*;
let data = include_str!("../input/2023/day13.txt");
assert_eq!(solve_part_02(&input_generator(data)), 29276);
```"#]
#[aoc(day13, part2)]
pub fn solve_part_02(input: &Input) -> i32 {
    input
        .patterns
        .iter()
        .map(|(grid, rocks)| {
            'vertical: for x in 1..grid.width {
                let mut smudges = 0;
                for rock in rocks {
                    let reflected = Point::new(2 * x - rock.x - 1, rock.y);

                    if grid.contains(reflected) && grid[reflected] == b'.' {
                        smudges += 1;

                        if smudges > 1 {
                            continue 'vertical;
                        }
                    }
                }

                if smudges == 1 {
                    return x;
                }
            }

            // Skip the first row
            'horizontal: for y in 1..grid.height {
                let mut smudges = 0;

                for rock in rocks {
                    let reflected = Point::new(rock.x, 2 * y - rock.y - 1);

                    if grid.contains(reflected) && grid[reflected] == b'.' {
                        smudges += 1;

                        if smudges > 1 {
                            continue 'horizontal;
                        }
                    }
                }

                if smudges == 1 {
                    return y * 100;
                }
            }

            unreachable!("This should never happen. Something blew up.")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        405
    )]
    fn sample_01(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        400
    )]
    fn sample_02(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(solve_part_02(&input_generator(input)), expected);
    }
}
