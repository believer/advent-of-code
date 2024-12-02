//! Day 13: Point of Incidence
//!
//! I had a super hard time understanding the problem description. I first thought that there
//! was just line (vertical or horizontal) in each pattern. Had to read and re-read it
//! multiple times before I understood.

use crate::grid::Grid;
use crate::point::Point;

#[derive(Eq, PartialEq)]
enum Line {
    Vertical(i32),
    Horizontal(i32),
}

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

fn find_line(grid: &Grid<u8>, rocks: &[Point], direction: Line) -> i32 {
    // Get the range of the line and the number of smudges
    let (mut range, target) = match direction {
        Line::Vertical(t) => (1..grid.width, t),
        Line::Horizontal(t) => (1..grid.height, t),
    };

    // Find the first line that matches the criteria
    let line = range.find(|&x| {
        let mut smudges = 0;

        rocks.iter().any(|rock| {
            let reflected = match direction {
                Line::Vertical(_) => Point::new(2 * x - rock.x - 1, rock.y),
                Line::Horizontal(_) => Point::new(rock.x, 2 * x - rock.y - 1),
            };

            if grid.contains(reflected) && grid[reflected] == b'.' {
                smudges += 1;
                smudges > target
            } else {
                false
            }
        });

        smudges == target
    });

    // Vertical lines yield the number of columns before the line
    // Horizontal lines yield the number of rows before the line * 100
    line.map(|x| match direction {
        Line::Vertical(_) => x,
        Line::Horizontal(_) => x * 100,
    })
    .unwrap_or(0)
}

fn summarize(input: &Input, smudges: i32) -> i32 {
    input
        .patterns
        .iter()
        .map(|(grid, rocks)| {
            find_line(grid, rocks, Line::Vertical(smudges))
                + find_line(grid, rocks, Line::Horizontal(smudges))
        })
        .sum()
}

/* Part One
*
*/
#[aoc(day13, part1)]
pub fn solve_part_01(input: &Input) -> i32 {
    summarize(input, 0)
}

/* Part Two
*
*/
#[aoc(day13, part2)]
pub fn solve_part_02(input: &Input) -> i32 {
    summarize(input, 1)
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
