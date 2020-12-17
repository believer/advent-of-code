// Day 17

mod cube {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum CubeState {
        Active,
        Inactive,
    }

    impl std::str::FromStr for CubeState {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "#" => CubeState::Active,
                "." => CubeState::Inactive,
                _ => unreachable!("Invalid cube state"),
            })
        }
    }

    pub type Cube = Vec<Vec<Vec<CubeState>>>;

    pub const CUBE_SIZE: usize = 27;
    pub const MID: usize = CUBE_SIZE / 2;
}

use cube::*;

const DELTA_NEIGHBORS: [(i32, i32, i32); 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
];

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Cube {
    let mut cube: Cube = vec![vec![vec![CubeState::Inactive; CUBE_SIZE]; CUBE_SIZE]; CUBE_SIZE];

    for (i, x) in input.lines().enumerate() {
        for (j, y) in x.chars().enumerate() {
            if y == '#' {
                cube[MID + i][MID + j][MID] = CubeState::Active;
            }
        }
    }

    cube
}

fn simulate_next_step_3d(cube: &Cube) -> Cube {
    let mut next_cube = cube.to_owned();

    for i in 1..CUBE_SIZE - 1 {
        for j in 1..CUBE_SIZE - 1 {
            for k in 1..CUBE_SIZE - 1 {
                let alive_neighbor_count = DELTA_NEIGHBORS
                    .iter()
                    .filter(|(di, dj, dk)| {
                        let ii = (i as i32 + di) as usize;
                        let jj = (j as i32 + dj) as usize;
                        let kk = (k as i32 + dk) as usize;

                        cube[ii][jj][kk] == CubeState::Active
                    })
                    .count();

                if cube[i][j][k] == CubeState::Active && !(2..=3).contains(&alive_neighbor_count) {
                    next_cube[i][j][k] = CubeState::Inactive;
                } else if cube[i][j][k] == CubeState::Inactive && alive_neighbor_count == 3 {
                    next_cube[i][j][k] = CubeState::Active;
                }
            }
        }
    }

    next_cube
}

fn active_cubes_3d(cube: &Cube) -> u32 {
    cube.iter().fold(0, |a, i| {
        a + i.iter().fold(0, |b, j| {
            b + j.iter().fold(0, |c, k| match k {
                CubeState::Active => c + 1,
                CubeState::Inactive => c,
            })
        })
    })
}

/// Your puzzle answer was
/// ```
/// use advent_of_code_2020::day_17::*;
/// let input = include_str!("../input/2020/day17.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 242);
/// ```
#[aoc(day17, part1)]
pub fn solve_part_01(input: &Cube) -> u32 {
    let mut cube = input.clone();

    for _ in 0..6 {
        cube = simulate_next_step_3d(&cube);
    }

    active_cubes_3d(&cube)
}

// /// Your puzzle answer was
// /// ```
// /// use advent_of_code_2020::day_17::*;
// /// let input = include_str!("../input/2020/day17.txt");
// /// assert_eq!(solve_part_02(&input_generator(input)), 37385);
// /// ```
// #[aoc(day17, part2)]
// pub fn solve_part_02(_input: &[usize]) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    // Test example data on part 1
    #[test]
    fn test_example_part_1() {
        let data = ".#.
..#
###";

        assert_eq!(solve_part_01(&input_generator(data)), 112)
    }
}
