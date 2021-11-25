use std::mem;

// Day 11 - Seating System

static ADJACENT_SEATS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SeatingSystem {
    Empty,
    Floor,
    Occupied,
}

impl std::str::FromStr for SeatingSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => SeatingSystem::Empty,
            "." => SeatingSystem::Floor,
            "#" => SeatingSystem::Occupied,
            _ => unreachable!("Invalid seat"),
        })
    }
}

#[derive(Clone)]
struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<SeatingSystem>>,
}

impl Grid {
    fn occupied_seats(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c == SeatingSystem::Occupied)
            .count()
    }

    fn get(&self, r: usize, c: usize) -> SeatingSystem {
        self.grid[r][c]
    }

    fn set(&mut self, seat: SeatingSystem, r: usize, c: usize) {
        self.grid[r][c] = seat;
    }

    fn new(grid: &[Vec<SeatingSystem>]) -> Grid {
        Grid {
            rows: grid.len(),
            cols: grid[0].len(),
            grid: grid.to_owned(),
        }
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<SeatingSystem>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

fn find_neighbors(
    grid: &[Vec<SeatingSystem>],
    (dr, dc): (isize, isize),
    (r, c): (usize, usize),
) -> Option<SeatingSystem> {
    let (mut r, mut c) = (r as isize, c as isize);

    loop {
        r += dr;
        c += dc;

        match grid.get(r as usize).and_then(|row| row.get(c as usize)) {
            Some(SeatingSystem::Floor) => (),
            Some(&seat) => return Some(seat),
            None => break,
        }
    }

    None
}

fn should_swap_part_1(grid: &[Vec<SeatingSystem>], r: usize, c: usize) -> bool {
    let mut neighbors = ADJACENT_SEATS
        .iter()
        .map(|&(dr, dc)| (r as isize + dr, c as isize + dc))
        .filter_map(|(r, c)| grid.get(r as usize).and_then(|v| v.get(c as usize)));

    match grid[r][c] {
        SeatingSystem::Empty => neighbors.all(|&c| c != SeatingSystem::Occupied),
        SeatingSystem::Occupied => {
            neighbors.filter(|&&c| c == SeatingSystem::Occupied).count() >= 4
        }
        _ => unreachable!("Not Important"),
    }
}

fn should_swap_part_2(grid: &[Vec<SeatingSystem>], r: usize, c: usize) -> bool {
    let mut neighbors = ADJACENT_SEATS
        .iter()
        .filter_map(|&dir| find_neighbors(grid, dir, (r, c)));

    match grid[r][c] {
        SeatingSystem::Empty => neighbors.all(|c| c != SeatingSystem::Occupied),
        SeatingSystem::Occupied => neighbors.filter(|&c| c == SeatingSystem::Occupied).count() >= 5,
        _ => unreachable!("Not important"),
    }
}

fn simulate_seating<F: Fn(&[Vec<SeatingSystem>], usize, usize) -> bool>(
    grid: &[Vec<SeatingSystem>],
    should_swap: F,
) -> usize {
    let mut grid = Grid::new(grid);
    let mut previous_grid = grid.to_owned();

    loop {
        let mut changed = false;

        for r in 0..grid.rows {
            for c in 0..grid.cols {
                if grid.get(r, c) == SeatingSystem::Floor {
                    continue;
                }

                let seat = match (grid.get(r, c), should_swap(&grid.grid, r, c)) {
                    (SeatingSystem::Empty, true) => SeatingSystem::Occupied,
                    (SeatingSystem::Occupied, true) => SeatingSystem::Empty,
                    (seat, _) => seat,
                };

                previous_grid.set(seat, r, c);
                changed |= seat != grid.get(r, c);
            }
        }

        mem::swap(&mut grid, &mut previous_grid);

        if !changed {
            break;
        }
    }

    grid.occupied_seats()
}

/* Part One
 *
 * Your plane lands with plenty of time to spare.
 * The final leg of your journey is a ferry that goes directly to the tropical island
 * where you can finally start your vacation. As you reach the waiting area to board the ferry,
 * you realize you're so early, nobody else has even arrived yet!
 *
 * By modeling the process people use to choose (or abandon) their seat in the waiting area,
 * you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).
 *
 * The seat layout fits neatly on a grid. Each position is either floor (.),
 * an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:
 *
 * L.LL.LL.LL
 * LLLLLLL.LL
 * L.L.L..L..
 * LLLL.LL.LL
 * L.LL.LL.LL
 * L.LLLLL.LL
 * ..L.L.....
 * LLLLLLLLLL
 * L.LLLLLL.L
 * L.LLLLL.LL
 *
 * Now, you just need to model the people who will be arriving shortly.
 * Fortunately, people are entirely predictable and always follow a simple set of rules.
 * All decisions are based on the number of occupied seats adjacent to a given seat
 * (one of the eight positions immediately up, down, left, right, or diagonal from the seat).
 * The following rules are applied to every seat simultaneously:
 *
 * If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
 * If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
 * Otherwise, the seat's state does not change.
 * Floor (.) never changes; seats don't move, and nobody sits on the floor.
 *
 * After one round of these rules, every seat in the example layout becomes occupied:
 *
 * #.##.##.##
 * #######.##
 * #.#.#..#..
 * ####.##.##
 * #.##.##.##
 * #.#####.##
 * ..#.#.....
 * ##########
 * #.######.#
 * #.#####.##
 * After a second round, the seats with four or more occupied adjacent seats become empty again:
 *
 * #.LL.L#.##
 * #LLLLLL.L#
 * L.L.L..L..
 * #LLL.LL.L#
 * #.LL.LL.LL
 * #.LLLL#.##
 * ..L.L.....
 * #LLLLLLLL#
 * #.LLLLLL.L
 * #.#LLLL.##
 * This process continues for three more rounds:
 *
 * #.##.L#.##
 * #L###LL.L#
 * L.#.#..#..
 * #L##.##.L#
 * #.##.LL.LL
 * #.###L#.##
 * ..#.#.....
 * #L######L#
 * #.LL###L.L
 * #.#L###.##
 * #.#L.L#.##
 * #LLL#LL.L#
 * L.L.L..#..
 * #LLL.##.L#
 * #.LL.LL.LL
 * #.LL#L#.##
 * ..L.L.....
 * #L#LLLL#L#
 * #.LLLLLL.L
 * #.#L#L#.##
 * #.#L.L#.##
 * #LLL#LL.L#
 * L.#.L..#..
 * #L##.##.L#
 * #.#L.LL.LL
 * #.#L#L#.##
 * ..L.L.....
 * #L#L##L#L#
 * #.LLLLLL.L
 * #.#L#L#.##
 *
 * At this point, something interesting happens: the chaos stabilizes and further
 * applications of these rules cause no seats to change state!
 * Once people stop moving around, you count 37 occupied seats.
 *
 * Simulate your seating area by applying the seating rules repeatedly until no seats change state.
 * How many seats end up occupied?
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2020::day_11::*;
/// let input = include_str!("../input/2020/day11.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 2183);
#[aoc(day11, part1)]
pub fn solve_part_01(grid: &[Vec<SeatingSystem>]) -> usize {
    simulate_seating(grid, should_swap_part_1)
}

/* Part Two
 *
 * As soon as people start to arrive, you realize your mistake.
 * People don't just care about adjacent seats - they care about the
 * first seat they can see in each of those eight directions!
 *
 * Now, instead of considering just the eight immediately adjacent seats,
 * consider the first seat in each of those eight directions.
 * For example, the empty seat below would see eight occupied seats:
 *
 * .......#.
 * ...#.....
 * .#.......
 * .........
 * ..#L....#
 * ....#....
 * .........
 * #........
 * ...#.....
 *
 * The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:
 *
 * .............
 * .L.L.#.#.#.#.
 * .............
 *
 * The empty seat below would see no occupied seats:
 *
 * .##.##.
 * #.#.#.#
 * ##...##
 * ...L...
 * ##...##
 * #.#.#.#
 * .##.##.
 *
 * Also, people seem to be more tolerant than you expected:
 * it now takes five or more visible occupied seats for an occupied seat to become empty
 * (rather than four or more from the previous rules). The other rules still apply:
 * empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.
 *
 * Given the same starting layout as above, these new rules cause the seating area to shift around as follows:
 *
 * L.LL.LL.LL
 * LLLLLLL.LL
 * L.L.L..L..
 * LLLL.LL.LL
 * L.LL.LL.LL
 * L.LLLLL.LL
 * ..L.L.....
 * LLLLLLLLLL
 * L.LLLLLL.L
 * L.LLLLL.LL
 *
 * #.##.##.##
 * #######.##
 * #.#.#..#..
 * ####.##.##
 * #.##.##.##
 * #.#####.##
 * ..#.#.....
 * ##########
 * #.######.#
 * #.#####.##
 *
 * #.LL.LL.L#
 * #LLLLLL.LL
 * L.L.L..L..
 * LLLL.LL.LL
 * L.LL.LL.LL
 * L.LLLLL.LL
 * ..L.L.....
 * LLLLLLLLL#
 * #.LLLLLL.L
 * #.LLLLL.L#
 *
 * #.L#.##.L#
 * #L#####.LL
 * L.#.#..#..
 * ##L#.##.##
 * #.##.#L.##
 * #.#####.#L
 * ..#.#.....
 * LLL####LL#
 * #.L#####.L
 * #.L####.L#
 *
 * #.L#.L#.L#
 * #LLLLLL.LL
 * L.L.L..#..
 * ##LL.LL.L#
 * L.LL.LL.L#
 * #.LLLLL.LL
 * ..L.L.....
 * LLLLLLLLL#
 * #.LLLLL#.L
 * #.L#LL#.L#
 *
 * #.L#.L#.L#
 * #LLLLLL.LL
 * L.L.L..#..
 * ##L#.#L.L#
 * L.L#.#L.L#
 * #.L####.LL
 * ..#.#.....
 * LLL###LLL#
 * #.LLLLL#.L
 * #.L#LL#.L#
 *
 * #.L#.L#.L#
 * #LLLLLL.LL
 * L.L.L..#..
 * ##L#.#L.L#
 * L.L#.LL.L#
 * #.LLLL#.LL
 * ..#.L.....
 * LLL###LLL#
 * #.LLLLL#.L
 * #.L#LL#.L#
 *
 * Again, at this point, people stop shifting around and the seating area reaches equilibrium.
 * Once this occurs, you count 26 occupied seats.
 *
 * Given the new visibility method and the rule change for occupied seats becoming empty,
 * once equilibrium is reached, how many seats end up occupied?
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2020::day_11::*;
/// let input = include_str!("../input/2020/day11.txt");
/// assert_eq!(solve_part_02(&input_generator(input)), 1990);
/// ```
#[aoc(day11, part2)]
pub fn solve_part_02(grid: &[Vec<SeatingSystem>]) -> usize {
    simulate_seating(grid, should_swap_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn test_example_part_1() {
        let data = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

        assert_eq!(solve_part_01(&input_generator(data)), 37)
    }

    /// Test example data on part 2
    #[test]
    fn test_example_part_2() {
        let data = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

        assert_eq!(solve_part_02(&input_generator(data)), 26)
    }
}
