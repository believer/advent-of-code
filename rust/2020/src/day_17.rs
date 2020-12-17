// Day 17 - Conway Cubes
//
// Brute force with shameless copy-pasta to part 2

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

    pub type Cube = Vec<Vec<CubeState>>;
    pub type HyperCube = Vec<Cube>;

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

#[aoc_generator(day17, part1)]
pub fn input_generator_part_01(input: &str) -> Vec<Cube> {
    let mut cube: Vec<Cube> =
        vec![vec![vec![CubeState::Inactive; CUBE_SIZE]; CUBE_SIZE]; CUBE_SIZE];

    for (i, x) in input.lines().enumerate() {
        for (j, y) in x.chars().enumerate() {
            if y == '#' {
                cube[MID + i][MID + j][MID] = CubeState::Active;
            }
        }
    }

    cube
}

#[aoc_generator(day17, part2)]
pub fn input_generator_part_02(input: &str) -> Vec<HyperCube> {
    let mut cube: Vec<HyperCube> =
        vec![vec![vec![vec![CubeState::Inactive; CUBE_SIZE]; CUBE_SIZE]; CUBE_SIZE]; CUBE_SIZE];

    for (i, x) in input.lines().enumerate() {
        for (j, y) in x.chars().enumerate() {
            if y == '#' {
                cube[MID + i][MID + j][MID][MID] = CubeState::Active;
            }
        }
    }

    cube
}

fn simulate_next_step_3d(cube: &[Cube]) -> Vec<Cube> {
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

fn simulate_next_step_4d(cube: &[HyperCube], deltas: &[(i32, i32, i32, i32)]) -> Vec<HyperCube> {
    let mut next_cube = cube.to_owned();

    for i in 1..CUBE_SIZE - 1 {
        for j in 1..CUBE_SIZE - 1 {
            for k in 1..CUBE_SIZE - 1 {
                for l in 1..CUBE_SIZE - 1 {
                    let alive_neighbor_count = deltas
                        .iter()
                        .filter(|(di, dj, dk, dl)| {
                            let ii = (i as i32 + di) as usize;
                            let jj = (j as i32 + dj) as usize;
                            let kk = (k as i32 + dk) as usize;
                            let ll = (l as i32 + dl) as usize;

                            cube[ii][jj][kk][ll] == CubeState::Active
                        })
                        .count();

                    if cube[i][j][k][l] == CubeState::Active
                        && !(2..=3).contains(&alive_neighbor_count)
                    {
                        next_cube[i][j][k][l] = CubeState::Inactive;
                    } else if cube[i][j][k][l] == CubeState::Inactive && alive_neighbor_count == 3 {
                        next_cube[i][j][k][l] = CubeState::Active;
                    }
                }
            }
        }
    }

    next_cube
}

fn active_cubes_3d(cube: &[Cube]) -> u32 {
    cube.iter().fold(0, |a, i| {
        a + i.iter().fold(0, |b, j| {
            b + j.iter().fold(0, |c, k| match k {
                CubeState::Active => c + 1,
                CubeState::Inactive => c,
            })
        })
    })
}

fn active_cubes_4d(cube: &[HyperCube]) -> u32 {
    cube.iter().fold(0, |a, i| {
        a + i.iter().fold(0, |b, j| {
            b + j.iter().fold(0, |c, k| {
                c + k.iter().fold(0, |c, l| match l {
                    CubeState::Active => c + 1,
                    CubeState::Inactive => c,
                })
            })
        })
    })
}

fn hyper_cube_neighbors() -> Vec<(i32, i32, i32, i32)> {
    let mut result = vec![];

    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                for l in -1..=1 {
                    if (i, j, k, l) == (0, 0, 0, 0) {
                        continue;
                    } else {
                        result.push((i, j, k, l));
                    }
                }
            }
        }
    }

    result
}

/* Part One
 *
 * As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau
 * at the North Pole contact you. They'd like some help debugging a malfunctioning experimental
 * energy source aboard one of their super-secret imaging satellites.
 *
 * The experimental energy source is based on cutting-edge technology:
 * a set of Conway Cubes contained in a pocket dimension!
 * When you hear it's having problems, you can't help but agree to take a look.
 *
 * The pocket dimension contains an infinite 3-dimensional grid.
 * At every integer 3-dimensional coordinate (x,y,z),
 * there exists a single cube which is either active or inactive.
 *
 * In the initial state of the pocket dimension, almost all cubes start inactive.
 * The only exception to this is a small flat region of cubes (your puzzle input);
 * the cubes in this region start in the specified active (#) or inactive (.) state.
 *
 * The energy source then proceeds to boot up by executing six cycles.
 *
 * Each cube only ever considers its neighbors: any of the 26 other cubes where any of
 * their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,
 * its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.
 *
 * During a cycle, all cubes simultaneously change their state according to the following rules:
 *
 * If a cube is active and exactly 2 or 3 of its neighbors are also active,
 * the cube remains active. Otherwise, the cube becomes inactive.
 * If a cube is inactive but exactly 3 of its neighbors are active,
 * the cube becomes active. Otherwise, the cube remains inactive.
 * The engineers responsible for this experimental energy source would like you
 * to simulate the pocket dimension and determine what the configuration of
 * cubes should be at the end of the six-cycle boot process.
 *
 * For example, consider the following initial state:
 *
 * .#.
 * ..#
 * ###
 *
 * Even though the pocket dimension is 3-dimensional, this initial state represents
 * a small 2-dimensional slice of it. (In particular, this initial state defines
 * a 3x3x1 region of the 3-dimensional space.)
 *
 * Simulating a few cycles from this initial state produces the following configurations,
 * where the result of each cycle is shown layer-by-layer at each given z
 * coordinate (and the frame of view follows the active cells in each cycle):
 *
 * REMOVED LONG LIST OF CYCLES
 *
 * After the full six-cycle boot process completes, 112 cubes are left in the active state.
 *
 * Starting with your given initial configuration, simulate six cycles.
 * How many cubes are left in the active state after the sixth cycle?
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2020::day_17::*;
/// let input = include_str!("../input/2020/day17.txt");
/// assert_eq!(solve_part_01(&input_generator_part_01(input)), 242);
/// ```
#[aoc(day17, part1)]
pub fn solve_part_01(input: &[Cube]) -> u32 {
    let mut cube = input.to_owned();

    for _ in 0..6 {
        cube = simulate_next_step_3d(&cube);
    }

    active_cubes_3d(&cube)
}

/* Part Two
 *
 * For some reason, your simulated results don't match what the experimental energy
 * source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.
 *
 * The pocket dimension contains an infinite 4-dimensional grid.
 * At every integer 4-dimensional coordinate (x,y,z,w), there exists a
 * single cube (really, a hypercube) which is still either active or inactive.
 *
 * Each cube only ever considers its neighbors: any of the 80 other cubes where any
 * of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4,
 * its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.
 *
 * The initial state of the pocket dimension still consists of a small flat region of cubes.
 * Furthermore, the same rules for cycle updating still apply: during each cycle,
 * consider the number of active neighbors of each cube.
 *
 * For example, consider the same initial state as in the example above.
 * Even though the pocket dimension is 4-dimensional, this initial state represents a
 * small 2-dimensional slice of it. (In particular, this initial state
 * defines a 3x3x1x1 region of the 4-dimensional space.)
 *
 * Simulating a few cycles from this initial state produces the following configurations,
 * where the result of each cycle is shown layer-by-layer at each given z and w coordinate:
 *
 * REMOVED LONG LIST OF CYCLES
 *
 * After the full six-cycle boot process completes, 848 cubes are left in the active state.
 *
 * Starting with your given initial configuration, simulate six cycles in a 4-dimensional space.
 * How many cubes are left in the active state after the sixth cycle?
 *
 * NO DOC TEST because slow
*/
#[aoc(day17, part2)]
pub fn solve_part_02(input: &[HyperCube]) -> u32 {
    let mut cube = input.to_owned();
    let neighbors = hyper_cube_neighbors();

    for _ in 0..6 {
        cube = simulate_next_step_4d(&cube, &neighbors);
    }

    active_cubes_4d(&cube)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test example data on part 1
    #[test]
    fn test_example_part_1() {
        let data = ".#.
..#
###";

        assert_eq!(solve_part_01(&input_generator_part_01(data)), 112)
    }

    // Test example data on part 2
    // This is slow. To run, use cargo test -- --ignored
    #[test]
    #[ignore]
    fn test_example_part_2() {
        let data = ".#.
..#
###";

        assert_eq!(solve_part_02(&input_generator_part_02(data)), 848)
    }
}
