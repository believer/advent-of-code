// Day 3 - Squares With Three Sides
//
// Sneaky whitespace in the beginning of each line.

type Input = Vec<Vec<u32>>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2016::day_03::*;
/// let data = include_str!("../input/2016/day3.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 862);
/// ```
#[aoc(day3, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    input
        .iter()
        .filter(|sides| {
            let a = sides[0];
            let b = sides[1];
            let c = sides[2];

            a + b > c && a + c > b && b + c > a
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = " 5 10 25";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 0)
    }

    #[test]
    fn sample_02() {
        let test = "  785  516  744
  272  511  358
  801  791  693";

        assert_eq!(solve_part_01(&input_generator(test)), 3)
    }
}
