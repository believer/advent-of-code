// Day 9: Mirage Maintenance
//
// I found it hard to create the data structure for this one. I knew what I wanted
// to do, but struggled with the borrow checker. After some trial and error, I
// got it working. Then it was just a matter of implementing the rules.

#[derive(Debug)]
pub struct Input {
    all_series: Vec<Vec<Vec<i64>>>,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Input {
    let parsed_input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|d| d.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let mut all_series = Vec::with_capacity(parsed_input.len());

    for d in parsed_input.iter() {
        let mut series = vec![d.clone()];
        let sequence = d.clone();
        let mut new_series = sequence.clone();

        loop {
            let mut differences = Vec::new();

            for i in 1..new_series.len() {
                differences.push(new_series[i] - new_series[i - 1]);
            }

            series.append(&mut vec![differences.clone()]);

            if differences.iter().all(|&x| x == 0) {
                break;
            }

            new_series = differences.clone();
        }

        all_series.append(&mut vec![series]);
    }

    Input { all_series }
}

/* Part One
*
* Given a sequence of numbers, find the next value in the sequence. This can
* be done by iterating over the sequence, until every value is 0. Then move
* back up the sequence, and add the last value to the next value.
*
* For example:
*
* 0 3 6 9 12 15 -> 18
* 1 3 6 10 15 21 -> 28
* 10 13 16 21 30 45 -> 68
*
* Add all the last values together, and you get the answer = 114
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_09::*;
let data = include_str!("../input/2023/day9.txt");
assert_eq!(solve_part_01(&input_generator(data)), 1853145119);
```"#]
#[aoc(day9, part1)]
pub fn solve_part_01(input: &Input) -> i64 {
    let mut next_values = 0;

    for series in input.all_series.iter() {
        let mut current_last = 0;
        let mut last_in_series = 0;

        for r in series.windows(2).rev() {
            let left_last = r[0].last().unwrap();

            current_last += left_last;
            last_in_series = current_last;
        }

        next_values += last_in_series;
    }

    next_values
}

/* Part Two
*
* The data is all the same, but we need to find the _previous_ value in the
* sequence. We find the zeroes, like in the first example, but instead of appending
* the differences, we prepend them.
*
* For example:
*
* 0 3 6 9 12 15 -> -3
* 1 3 6 10 15 21 -> 0
* 10 13 16 21 30 45 -> 5
*
* Add all the first values together, and you get the answer = 2
*
*/
#[doc = r#"```
use advent_of_code_2023::day_09::*;
let data = include_str!("../input/2023/day9.txt");
assert_eq!(solve_part_02(&input_generator(data)), 923);
```"#]
#[aoc(day9, part2)]
pub fn solve_part_02(input: &Input) -> i64 {
    let mut next_values = 0;

    for series in input.all_series.iter() {
        let mut current_last = 0;
        let mut last_in_series = 0;

        for r in series.windows(2).rev() {
            let left_first = r[0].first().unwrap();

            current_last = left_first - current_last;
            last_in_series = current_last;
        }

        next_values += last_in_series;
    }

    next_values
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 114);
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(DATA)), 2);
    }
}
