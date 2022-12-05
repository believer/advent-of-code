use std::collections::{btree_map::Entry, BTreeMap, VecDeque};

// Day 5 - Supply Stacks
//
// Couldn't get a good grip on parsing the stacks that early in the morning. It was
// easier to do it manually since it wasn't that many. Came back and created a parser for it.
//
// Refactored using a BTreeMap to store the stacks. It doesn't make a difference
// in performance, but it's simple to use.
//
// To inspiration from @ankjevel to use VecDeque instead of Vec for the stacks.
// The solution became both simpler and faster. Really beautiful.

pub struct Instruction {
    to: usize,
    from: usize,
    moves: usize,
}

// Instructions look like this:
// move 5 from 1 to 2
//
// Split it by spaces and pick out the numbers.
impl Instruction {
    fn new(input: &str) -> Instruction {
        let parts: Vec<&str> = input.split_whitespace().collect();

        Instruction {
            to: parts[5].parse::<usize>().unwrap() - 1,
            from: parts[3].parse::<usize>().unwrap() - 1,
            moves: parts[1].parse().unwrap(),
        }
    }
}

type Stacks = BTreeMap<usize, VecDeque<String>>;
type Instructions = Vec<Instruction>;
type Input = (Stacks, Instructions);

fn first_in_stacks(stacks: Stacks) -> String {
    stacks
        .values()
        .map(|v| v.front().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let mut stacks: Stacks = BTreeMap::new();

    let stacks_and_instructions: Vec<&str> = input.split("\n\n").collect();

    // Parse the stacks. Remove the last line of column numbers.
    // The example data is:
    //
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3
    for row in stacks_and_instructions[0]
        .lines()
        .collect::<Vec<_>>()
        .split_last()
        .unwrap()
        .1
    {
        for (i, column) in row.chars().collect::<Vec<_>>().chunks(4).enumerate() {
            // Find the name of the crate
            let value: String = column
                .iter()
                .map(|s| s.to_string().trim().replace(['[', ']'], ""))
                .collect::<Vec<String>>()
                .join("");

            // Skip empty columns
            if value.is_empty() {
                continue;
            }

            // Add to or create the stack
            match stacks.entry(i) {
                Entry::Vacant(e) => {
                    e.insert(VecDeque::from_iter([value]));
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push_back(value);
                }
            }
        }
    }

    // Parse the instructions
    let instructions: Vec<Instruction> = stacks_and_instructions[1]
        .lines()
        .filter(|s| !s.is_empty())
        .map(Instruction::new)
        .collect();

    (stacks, instructions)
}

/* Part One
 *
 * Move each crate to the correct stack. One at a time.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_05::*;
/// let data = include_str!("../input/2022/day5.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), "PSNRGBTFT");
/// ```
#[aoc(day5, part1)]
pub fn solve_part_01((stacks, instructions): &Input) -> String {
    let mut stacks = stacks.clone();

    for Instruction { moves, from, to } in instructions {
        // Move the crates, one at a time
        for _ in 0..*moves {
            // Remove the top crate from the source stack
            let crate_in_stack = stacks.get_mut(from).unwrap().pop_front().unwrap();

            // Add it to the destination stack
            stacks.get_mut(to).unwrap().push_front(crate_in_stack);
        }
    }

    // Find the first crate in each stack
    first_in_stacks(stacks)
}

/* Part Two
 *
 * Move each crate to the correct stack. All at once.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_05::*;
/// let data = include_str!("../input/2022/day5.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), "BNTZFPMMW");
/// ```
#[aoc(day5, part2)]
pub fn solve_part_02((stacks, instructions): &Input) -> String {
    let mut stacks = stacks.clone();

    for Instruction { moves, from, to } in instructions {
        // Remove all crates that should be moved from the source stack.
        // Reverse the order since we're draining from the front.
        let move_stack: VecDeque<_> = stacks.get_mut(from).unwrap().drain(..moves).rev().collect();

        // Add the crates to the destination stack
        for crate_in_stack in move_stack {
            stacks.get_mut(to).unwrap().push_front(crate_in_stack);
        }
    }

    // Find the first crate in each stack
    first_in_stacks(stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(solve_part_01(&input_generator(data)), "CMZ")
    }

    #[test]
    fn sample_02() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(solve_part_02(&input_generator(data)), "MCD")
    }
}
