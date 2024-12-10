use crate::{
    grid::Grid,
    point::{Point, CARDINALS},
};
use std::collections::{HashSet, VecDeque};

pub struct Input {
    grid: Grid<u8>,
    starts: Vec<Point>,
}

// I really enjoy the VecDeque API so I wanted to do this day here as well.
// This together with my Grid/Point helpers from last year makes for some
// really nice code :D
#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    let grid: Grid<u8> = Grid::from(input);
    let trailheads = grid.find_all(b'0');

    Input {
        grid,
        starts: trailheads,
    }
}

fn calculate_trail_score(grid: &Grid<u8>, start: &Point, is_distinct: bool) -> usize {
    let mut trail_score = 0;
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(*start);

    while let Some(location) = queue.pop_front() {
        let current_point = grid[location];

        if visited.contains(&location) && is_distinct {
            continue;
        }

        visited.insert(location);

        if current_point == b'9' {
            trail_score += 1;
            continue;
        }

        for direction in CARDINALS {
            let next = location + direction;

            if grid.contains(next) && grid[next] == current_point + 1 {
                queue.push_back(next);
            }
        }
    }

    trail_score
}

#[aoc(day10, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    let Input { grid, starts } = input;

    starts
        .iter()
        .map(|start| calculate_trail_score(grid, start, true))
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let Input { grid, starts } = input;

    starts
        .iter()
        .map(|start| calculate_trail_score(grid, start, false))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 36)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(DATA)), 81)
    }
}
