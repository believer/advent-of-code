// Day 10 - Cathode-Ray Tube
//
// I understood the first part quickly. The second part was a bit more
// challenging. I had to go through the example sample in parts to be able
// to understand it. I got stuck on how to "change lines" the longest.

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
 *
 * We have a CPU register, x, that starts at 1 and a cycle that
 * increments by one. This value is changed by two types of operation:
 *
 * - noop - no operation, does nothing. Takes one cycle to complete.
 * - addx +/-v - adds v to x. Takes two cycles to complete.
 *
 * If the cycle is one of 20, 60, 100, 140, 180, or 220
 * we should add the cycle value * x to the solution. These cycles
 * can happen in the middle of an operation.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_10::*;
/// let data = include_str!("../input/2022/day10.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), );
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

    solution
}

/* Part Two
 *
 * We are drawing on a CRT monitor of the size 40x6 = 240 pixels.
 * We have a three-pixel sprite that is centered on the current x value.
 * For each cycle, we check if the sprite is currently on that pixel. If
 * it exists, we leave a lit pixel '#' otherwise we leave a dark pixel '.'.
 *
 * The result will be eight capital letters that are spelled out on the CRT.
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_10::*;
/// let data = include_str!("../input/2022/day10.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), "###..#..#..##..####..##....##.###..###..#..#.#.#..#..#....#.#..#....#.#..#.#..#.#..#.##...#..#...#..#..#....#.###..#..#.#.#..#.#..####..#...####....#.#..#.###..#.#..#.#..#..#.#....#..#.#..#.#..#.#.#..#..#.#..#.#..#.####.#..#..##..###..#..#.");
/// ```
#[aoc(day10, part2)]
pub fn solve_part_02(input: &Input) -> String {
    let mut x: isize = 1;
    let mut cycle = 0;
    let mut crt = vec!["."; 240];
    let mut sprite = 0..=2;

    for line in input {
        if line == "noop" {
            for _ in 0..1 {
                if sprite.contains(&(cycle % 40)) {
                    crt[cycle] = "#";
                }
                cycle += 1;
            }
            continue;
        }

        let (_, v) = line.split_once(' ').unwrap();

        for _ in 0..2 {
            if sprite.contains(&(cycle % 40)) {
                crt[cycle] = "#";
            }
            cycle += 1;
        }

        x += v.parse::<isize>().unwrap();

        if x < 1 {
            sprite = 0..=0;
        } else {
            sprite = (x as usize - 1)..=(x as usize + 1);
        }
    }

    crt.chunks(40).for_each(|c| println!("{}", c.join("")));

    println!();

    crt.join("")
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

    #[test]
    fn sample_02() {
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

        assert_eq!(solve_part_02(&input_generator(data)), "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....")
    }
}
