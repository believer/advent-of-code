//! Day 19:

#[derive(Debug)]
pub struct Input {
    data: u32,
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    println!("{}", input);

    Input { data: 0 }
}

/* Part One
*
*/
/*
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_19::*;
let data = include_str!("../input/2023/day19.txt");
assert_eq!(solve_part_01(&input_generator(data)), 0);
```"#]
*/
#[aoc(day19, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    input.data
}

/* Part Two
*
*
*/
/*
#[doc = r#"```
use advent_of_code_2023::day_19::*;
let data = include_str!("../input/2023/day19.txt");
assert_eq!(solve_part_02(&input_generator(data)), 0);
```"#]
*/
#[aoc(day19, part2)]
pub fn solve_part_02(_input: &Input) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", 0)]
    fn sample_01(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }
}
