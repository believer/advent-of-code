use crate::math;

// Day 1 - No Time for a Taxicab

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Left(i32),
    Right(i32),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount = s[1..].parse().unwrap();

        Ok(match &s[..1] {
            "L" => Instruction::Left(amount),
            "R" => Instruction::Right(amount),
            _ => unreachable!("Invalid instruction"),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Cab {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Cab {
    fn drive(&mut self, steps: &Instruction) {
        match (steps, self.direction) {
            (Instruction::Left(v), Direction::South)
            | (Instruction::Right(v), Direction::North) => {
                self.x += v;
                self.direction = Direction::East;
            }
            (Instruction::Left(v), Direction::North)
            | (Instruction::Right(v), Direction::South) => {
                self.x -= v;
                self.direction = Direction::West;
            }
            (Instruction::Left(v), Direction::East) | (Instruction::Right(v), Direction::West) => {
                self.y += v;
                self.direction = Direction::North;
            }
            (Instruction::Left(v), Direction::West) | (Instruction::Right(v), Direction::East) => {
                self.y -= v;
                self.direction = Direction::South;
            }
        }
    }

    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::North,
        }
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .split(',')
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

/* Part One
 */
#[aoc(day1, part1)]
pub fn solve_part_01(instructions: &[Instruction]) -> u32 {
    let mut cab = Cab::new();

    for instruction in instructions {
        cab.drive(instruction);
    }

    math::manhattan_distance(cab.x, cab.y)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test example data on part 1
    #[test]
    fn test_example_part_1() {
        let data = "R5, L5, R5, R3";

        assert_eq!(solve_part_01(&input_generator(data)), 12)
    }
}
