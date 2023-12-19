//! Day 19:

use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    workflows: HashMap<String, Vec<Condition>>,
    ratings: Vec<HashMap<char, u32>>,
}

#[derive(Debug)]
enum Condition {
    LessThan(char, u32, String),
    GreaterThan(char, u32, String),
    Target(String),
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let (workflows, ratings) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            let mut parts = line.split('{');
            let name = parts.next().unwrap();
            let rules = parts
                .next()
                .unwrap()
                .split(',')
                .map(|rule| {
                    let parts = rule.split_once(':');

                    if let Some((condition, result)) = parts {
                        if condition.contains('<') {
                            let (key, value) = condition.split_once('<').unwrap();
                            let key = key.chars().next().unwrap();
                            let value = value.parse::<u32>().unwrap();

                            Condition::LessThan(key, value, result.to_string())
                        } else {
                            let (key, value) = condition.split_once('>').unwrap();
                            let key = key.chars().next().unwrap();
                            let value = value.parse::<u32>().unwrap();

                            Condition::GreaterThan(key, value, result.to_string())
                        }
                    } else {
                        let rule = rule.replace('}', "");

                        Condition::Target(rule)
                    }
                })
                .collect::<Vec<_>>();

            (name.to_string(), rules)
        })
        .collect::<HashMap<String, Vec<_>>>();

    let ratings = ratings
        .lines()
        .map(|line| {
            let clean = &line[1..line.len() - 1];
            let clean = clean.split(',');

            clean
                .map(|l| {
                    let (name, value) = l.split_once('=').unwrap();
                    (name.chars().next().unwrap(), value.parse::<u32>().unwrap())
                })
                .collect::<HashMap<char, u32>>()
        })
        .collect::<Vec<_>>();

    Input { workflows, ratings }
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
    let mut accepted = 0;

    for rating in &input.ratings {
        let mut current_target = "in";

        while current_target != "R" {
            if current_target == "A" {
                accepted += rating.values().sum::<u32>();
                break;
            }

            let current_rule = input.workflows.get(current_target).unwrap();

            for rule in current_rule {
                match rule {
                    Condition::LessThan(key, value, target) => {
                        let r = rating.get(key).unwrap();

                        if r < value {
                            current_target = target;
                            break;
                        }
                    }
                    Condition::GreaterThan(key, value, target) => {
                        let r = rating.get(key).unwrap();

                        if r > value {
                            current_target = target;
                            break;
                        }
                    }
                    Condition::Target(target) => {
                        current_target = target;
                    }
                }
            }
        }
    }

    accepted
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
pub fn solve_part_02(_input: &Input) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        19114
    )]
    fn sample_01(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case(
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        167409079868000
    )]
    fn sample_02(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(solve_part_02(&input_generator(input)), expected);
    }
}
