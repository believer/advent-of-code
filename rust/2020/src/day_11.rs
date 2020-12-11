use grid::*;

// Day 11 - Seating System

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> (Vec<char>, usize) {
    let cols: Vec<&str> = input.lines().collect();

    (input.replace("\n", "").chars().collect(), cols[0].len())
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
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_11::*;
/// let input = include_str!("../input/2020/day11.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 2183);
#[aoc(day11, part1)]
pub fn solve_part_01(input: &(Vec<char>, usize)) -> usize {
    let mut grid = Grid::from_vec(input.0.to_vec(), input.1);
    let mut previous_grid = Grid::from_vec(input.0.to_vec(), input.1);

    loop {
        let mut update_seats = vec![];
        let adjacent_seats: Vec<(isize, isize)> = vec![
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ];

        for r in 0..grid.rows() {
            for c in 0..grid.cols() {
                let mut occupied_seats = 0;

                // Find how many seats around us that are occupied
                for (x, y) in &adjacent_seats {
                    if let Some('#') =
                        grid.get((r as isize + x) as usize, (c as isize + y) as usize)
                    {
                        occupied_seats += 1;
                    }
                }

                // Add seat arrangement updates
                match grid.get(r, c) {
                    Some('L') if occupied_seats == 0 => update_seats.push((r, c, '#')),
                    Some('#') if occupied_seats >= 4 => update_seats.push((r, c, 'L')),
                    _ => (),
                }
            }
        }

        // Update seat arrangement
        for (r, c, seat) in update_seats {
            grid[r][c] = seat
        }

        // Nothing change since last time, we're done
        if previous_grid == grid {
            break;
        }

        previous_grid = grid.clone();
    }

    // Find number of occupied seats, our solution
    let mut occupied_seats = 0;

    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if let Some('#') = grid.get(r, c) {
                occupied_seats += 1;
            }
        }
    }

    occupied_seats
}

/////your puzzle answer was.
///// ```
///// use advent_of_code_2020::day_11::*;
///// let input = include_str!("../input/2020/day11.txt");
///// assert_eq!(solve_part_02(&input_generator_part_02(input)), 116);
///// ```
//#[aoc(day11, part2)]
//pub fn solve_part_02(input: &[u32]) -> usize {
//    0
//}

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
}
