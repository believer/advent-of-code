// Day 1 - Not Quite Lisp

pub enum Direction {
    Up,
    Down,
}

type Input = Vec<Direction>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    input
        .split("")
        .filter(|s| !s.is_empty())
        .map(|s| match s {
            "(" => Direction::Up,
            ")" => Direction::Down,
            b => panic!("Invalid direction {b}"),
        })
        .collect()
}

/* Part One
*/
#[aoc(day1, part1)]
pub fn solve_part_01(directions: &Input) -> isize {
    directions
        .iter()
        .map(|direction| match direction {
            Direction::Up => 1,
            Direction::Down => -1,
        })
        .sum()
}

/* Part Two
*/
#[aoc(day1, part2)]
pub fn solve_part_02(directions: &Input) -> usize {
    let mut floor = 0;
    let mut ans = 0;

    for (i, direction) in directions.iter().enumerate() {
        floor = match direction {
            Direction::Up => floor + 1,
            Direction::Down => floor - 1,
        };

        if floor == -1 {
            ans = i + 1;
            break;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "(())";
    const SAMPLE_2: &str = "(((";
    const SAMPLE_3: &str = ")((";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 0)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE_2)), 3)
    }

    #[test]
    fn sample_03() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE_3)), 1)
    }
}
