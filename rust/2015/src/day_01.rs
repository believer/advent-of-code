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
/// Your puzzle answer was
/// ```
/// use advent_of_code_2015::day_01::*;
/// let data = include_str!("../input/2015/day1.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 232);
/// ```
#[aoc(day1, part1)]
pub fn solve_part_01(directions: &Input) -> isize {
    directions.iter().fold(0, |acc, direction| match direction {
        Direction::Up => acc + 1,
        Direction::Down => acc - 1,
    })
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2015::day_01::*;
/// let data = include_str!("../input/2015/day1.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 1783);
/// ```
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
