// This is me trying to learn more about the nom parser. I'm using it to parse
// the example data, as I did for my solution, from each day.
//
// Ideally, each parser would be broken down into multiple functions, but I'll
// keep them in one function per day for now.

use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag, character::*, multi::separated_list1, sequence::separated_pair, IResult,
    Parser,
};

fn day_01(example: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(
        tag("\n\n"),
        separated_list1(complete::newline, complete::u32).map(|v| v.into_iter().sum()),
    )(example)
}

fn day_02(example: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(
        complete::newline,
        separated_pair(complete::alpha1, tag(" "), complete::alpha1),
    )(example)
}

fn day_03(example: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(complete::newline, complete::alpha1)(example)
}

fn day_04(example: &str) -> IResult<&str, Vec<Vec<RangeInclusive<u32>>>> {
    separated_list1(
        complete::newline,
        separated_pair(
            separated_pair(complete::u32, complete::char('-'), complete::u32),
            complete::char(','),
            separated_pair(complete::u32, complete::char('-'), complete::u32),
        )
        .map(|((a, b), (c, d))| vec![a..=b, c..=d]),
    )(example)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_day_01() {
        let example = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(
            day_01(example).unwrap().1,
            vec![6000, 4000, 11000, 24000, 10000]
        )
    }

    #[test]
    fn parse_day_02() {
        let example = "A Y
B X
C Z";

        assert_eq!(
            day_02(example).unwrap().1,
            vec![("A", "Y"), ("B", "X"), ("C", "Z")]
        )
    }

    #[test]
    fn parse_day_03() {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(
            day_03(example).unwrap().1,
            vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ]
        )
    }

    #[test]
    fn parse_day_04() {
        let example = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(
            day_04(example).unwrap().1,
            vec![
                [(2..=4), (6..=8)],
                [(2..=3), (4..=5)],
                [(5..=7), (7..=9)],
                [(2..=8), (3..=7)],
                [(6..=6), (4..=6)],
                [(2..=6), (4..=8)],
            ]
        )
    }
}
