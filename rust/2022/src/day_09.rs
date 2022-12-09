use std::{cmp, collections::HashSet};

// Day 9 - Rope Bridge
//
// I had a solution that worked for the example input and also part of the real
// input, but I couldn't figure out why it was failing for the total input. I even created a
// function that could draw the board state, but still couldn't figure out why it was failing.
//
// I ended up looking on Reddit where I found out about the Chebyshev distance (there's always
// some math solution that you'll learn). The result of my solution was 5548 and with
// the Chebyshev distance it was 5735. Close, but not quite.

type Instructions = Vec<String>;

type Knot = (i32, i32);
type Visits = HashSet<Knot>;

#[derive(Debug)]
struct Rope {
    knots: Vec<Knot>,
    visits: Visits,
}

impl Rope {
    fn new(knots: usize) -> Rope {
        Rope {
            knots: vec![(0, 0); knots],
            visits: HashSet::new(),
        }
    }

    // Use the Chebyshev distance to determine the position of the tail
    fn calculate_tail_position(&mut self, head: usize, tail: usize) -> Knot {
        let (head_x, head_y) = self.knots[head];
        let (tail_x, tail_y) = self.knots[tail];
        let dx = head_x - tail_x;
        let dy = head_y - tail_y;

        match cmp::max(dx.abs(), dy.abs()) {
            // Return the current tail position if it is within one step of the head
            1 => self.knots[tail],

            // If the head has moved more than one step away in either direction,
            // then the tail should follow it. The signum method is used to determine
            // the direction of the movement. It returns -1, 0, or 1 depending on the
            // sign of the value.
            _ => (tail_x + dx.signum(), tail_y + dy.signum()),
        }
    }

    fn move_head(&mut self, instruction: &str) {
        let (direction, distance) = instruction.split_once(' ').unwrap();

        // lol CoPilot wrote almost everything
        for _ in 0..distance.parse::<i32>().unwrap() {
            let (x, y) = self.knots[0];

            // Move head
            match direction {
                "U" => self.knots[0] = (x, y + 1),
                "D" => self.knots[0] = (x, y - 1),
                "L" => self.knots[0] = (x - 1, y),
                "R" => self.knots[0] = (x + 1, y),
                _ => unreachable!("Unknown direction"),
            };

            // Move tail
            for i in 0..self.knots.len() - 1 {
                self.knots[i + 1] = self.calculate_tail_position(i, i + 1);
            }

            self.visits.insert(*self.knots.last().unwrap());
        }
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Instructions {
    input.lines().map(|l| l.to_string()).collect()
}

/* Part One
 *
 * Calculate the number of coordinates visited by the _tail_ of the rope.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_09::*;
/// let data = include_str!("../input/2022/day9.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 5735);
/// ```
#[aoc(day9, part1)]
pub fn solve_part_01(instructions: &Instructions) -> usize {
    // Create rope with two knots
    let mut rope = Rope::new(2);

    for instruction in instructions {
        rope.move_head(instruction);
    }

    // Return the number of unique knots visited
    rope.visits.len()
}

/* Part Two
 *
 * Here we need to use 10 knots instead of 2.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_09::*;
/// let data = include_str!("../input/2022/day9.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 2478);
/// ```
#[aoc(day9, part2)]
pub fn solve_part_02(instructions: &Instructions) -> usize {
    // Create rope with ten knots
    let mut rope = Rope::new(10);

    for instruction in instructions {
        rope.move_head(instruction);
    }

    // Return the number of unique knots visited
    rope.visits.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(solve_part_01(&input_generator(data)), 13)
    }

    #[test]
    fn sample_02() {
        let data = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(solve_part_02(&input_generator(data)), 36)
    }
}
