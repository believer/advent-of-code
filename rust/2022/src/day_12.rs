use std::collections::HashSet;

use pathfinding::prelude::{dijkstra, Matrix};

// Day 12 - Hill Climbing Algorithm
//
// I knew I had to use some sort of pathfinding algorithm. I really didn't
// want to invent the wheel again so I used the 'pathfinding' crate.
// I first thought of A*, but I thought the Dijkstra algorithm looked
// easiest. The included matrix implementation was very helpful
// to get neighbors and valid successors.

type Input = (Matrix<usize>, Position, Position);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position(usize, usize);

impl Position {
    fn successors(&self, network: Matrix<usize>) -> Vec<(Position, usize)> {
        // Current position and cost
        let &Position(r, c) = self;
        let cost = network.get((r, c)).unwrap();

        // Find all neighbors and filter out any neighbours that are not valid
        // successors. A valid successor is a neighbour that is not
        // greater than 1 in cost. Return a vector of positions
        // and costs that the algorithm can use.
        network
            .neighbours((r, c), false)
            .filter(|(r, c)| {
                let neighbour_cost = network[(*r, *c)];
                neighbour_cost as isize - *cost as isize <= 1
            })
            .map(|(r, c)| (Position(r, c), network[(r, c)]))
            .collect()
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Input {
    // Find number of rows and columns
    let data = input.lines().collect::<Vec<&str>>();
    let rows = data.len();
    let cols = data[0].len();

    // Use the lowercase alphabet to convert the strings to weighted costs.
    let alphabet: Vec<char> = ('a'..='z').collect();

    // Setup return matrix and start and end positions
    let mut matrix = Matrix::new(rows, cols, 1);
    let mut start = Position(0, 0);
    let mut end = Position(0, 0);

    for (i, row) in input.lines().enumerate() {
        for (j, col) in row.chars().enumerate() {
            // Update start position
            if col == 'S' {
                // Start point defaults to 'a' (0) in cost
                start = Position(i, j);
                continue;
            }

            // Update end position
            if col == 'E' {
                end = Position(i, j);
                // Set end point to 'z' (26) in cost
                matrix[(i, j)] = 26;
                continue;
            }

            // Set all other points to their cost, i.e., their position in the alphabet
            matrix[(i, j)] = alphabet.iter().position(|&c| c == col).unwrap() + 1;
        }
    }

    (matrix, start, end)
}

/* Part One
 *
 * Find the shortest path from a provided start position to an end position.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_12::*;
/// let data = include_str!("../input/2022/day12.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 0);
/// ```
#[aoc(day12, part1)]
pub fn solve_part_01((matrix, start, end): &Input) -> usize {
    // Find the shortest path from start to end
    let (path, _) =
        dijkstra(start, |p| p.successors(matrix.clone()), |p| p == end).expect("No path found");

    // End point is not included in the steps, so we remove one step
    path.len() - 1
}

/* Part Twot
 *
 * Here we need to consider all paths that start from the lowest cost.
 * First, let's find all position with the lowest cost. Then, we can
 * run Dijkstra's algorithm from each of these positions and save the
 * amount of steps needed. Then we pick the shortest path.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_12::*;
/// let data = include_str!("../input/2022/day12.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 0);
/// ```
#[aoc(day12, part2)]
pub fn solve_part_02((matrix, _start, end): &Input) -> usize {
    // Find all starting points at the lowest cost
    let mut possible_starts = vec![];

    for i in 0..matrix.rows {
        for j in 0..matrix.columns {
            if matrix[(i, j)] == 1 {
                possible_starts.push(Position(i, j));
            }
        }
    }

    // Save all paths that reach the end from any of the possible starts
    let mut paths: HashSet<usize> = HashSet::new();

    for start in possible_starts {
        // Only save completed paths
        if let Some((path, _)) = dijkstra(&start, |p| p.successors(matrix.clone()), |p| p == end) {
            // End point is not included in the steps, so we remove one step
            paths.insert(path.len() - 1);
        }
    }

    // Return the shortest path of all possible starts
    *paths.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 31)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 29)
    }
}
