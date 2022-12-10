// Day 10 - Part 1

type Input = Vec<String>;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().to_string())
        .collect()
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_10::*;
/// let data = include_str!("../input/2022/day10.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 0);
/// ```
#[aoc(day10, part1)]
pub fn solve_part_01(input: &Input) -> i32 {
    let mut x = 1;
    let mut cycle = 0;
    let mut solution = 0;

    let cycles = vec![20, 60, 100, 140, 180, 220];

    for line in input {
        if line == "noop" {
            for _ in 0..1 {
                cycle += 1;
                if cycles.contains(&cycle) {
                    solution += x * cycle;
                }
            }
            continue;
        }

        let (_, v) = line.split_once(' ').unwrap();

        for _ in 0..2 {
            cycle += 1;
            if cycles.contains(&cycle) {
                solution += x * cycle;
            }
        }

        x += v.parse::<i32>().unwrap();
    }

    // println!("{cycle} {x:?}");
    // println!("{:?}", solution);

    solution
}

/* Part Two
*/
// /// Your puzzle answer was
// /// ```
// /// use advent_of_code_2022::day_10::*;
// /// let data = include_str!("../input/2022/day10.txt");
// /// assert_eq!(solve_part_02(&input_generator(data)), 0);
// /// ```
#[aoc(day10, part2)]
pub fn solve_part_02(_input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        assert_eq!(solve_part_01(&input_generator(data)), 13140)
    }
}
