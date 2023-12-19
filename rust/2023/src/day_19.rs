//! Day 19: Aplenty
//!
//! The hard part about part 1 was getting the parsing done in a good and usable way.
//! Otherwise, it's just a matter of following the rules until we get to A or R.
//!
//! Part 2 I don't really understand right now, might come back.

use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    workflows: HashMap<String, Vec<Condition>>,
    ratings: Vec<HashMap<char, u64>>,
}

#[derive(Debug)]
enum Condition {
    LessThan(char, u64, String),
    GreaterThan(char, u64, String),
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
                            let value = value.parse::<u64>().unwrap();

                            Condition::LessThan(key, value, result.to_string())
                        } else {
                            let (key, value) = condition.split_once('>').unwrap();
                            let key = key.chars().next().unwrap();
                            let value = value.parse::<u64>().unwrap();

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
                    (name.chars().next().unwrap(), value.parse::<u64>().unwrap())
                })
                .collect::<HashMap<char, u64>>()
        })
        .collect::<Vec<_>>();

    Input { workflows, ratings }
}

/* Part One
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_19::*;
let data = include_str!("../input/2023/day19.txt");
assert_eq!(solve_part_01(&input_generator(data)), 331208);
```"#]
#[aoc(day19, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    let mut accepted = 0;

    for rating in &input.ratings {
        let mut current_target = "in";

        loop {
            // An accepted rating, add the sum of the values
            if current_target == "A" {
                accepted += rating.values().sum::<u64>();
                break;
            }

            // A rejected rating, discard it
            if current_target == "R" {
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

fn count_accepted(
    ranges: HashMap<char, (u64, u64)>,
    workflows: &HashMap<String, Vec<Condition>>,
    name: String,
) -> u64 {
    // Rejected
    if name == "R" {
        return 0;
    }

    // Accepted
    if name == "A" {
        return ranges.values().map(|(lo, hi)| hi - lo + 1).product();
    }

    // Our final total
    let mut total = 0;

    // Clone the ranges so we can modify them
    let mut ranges = ranges.clone();

    // Get the rules for the current workflow
    let rules = workflows.get(&name).unwrap();

    // Iterate over the rules
    // Split the ranges into two, one for the true condition and one for the false condition
    // The true half gets processed while the false half is deferred to the next rule
    for rule in rules {
        let (t, f, key, target) = match rule {
            Condition::LessThan(key, value, target) => {
                let (lo, hi) = ranges.get(key).unwrap();

                // True half is the range from the lower bound to the value - 1
                let t = (*lo, (value - 1).min(*hi));
                // False half is the range from the value to the upper bound
                let f = (*value.max(lo), *hi);

                (t, f, key, target)
            }
            Condition::GreaterThan(key, value, target) => {
                let (lo, hi) = ranges.get(key).unwrap();

                // True half is the range from the value + 1 to the upper bound
                let t = ((value + 1).max(*lo), *hi);
                // False half is the range from the lower bound to the value
                let f = (*lo, *value.min(hi));

                (t, f, key, target)
            }
            Condition::Target(target) => {
                total += count_accepted(ranges.clone(), workflows, target.to_string());
                continue;
            }
        };

        // Optimization: If the true half is empty, we can skip it
        // Otherwise, update the ranges and recurse
        if t.0 <= t.1 {
            ranges.insert(*key, t);

            total += count_accepted(ranges.clone(), workflows, target.to_string());
        }

        // Optimization: If the false half is empty, we can skip it
        // Just update the ranges and continue. This will be passed to
        // Condition::Target and handled there
        if f.0 <= f.1 {
            ranges.insert(*key, f);
        }
    }

    total
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
pub fn solve_part_02(input: &Input) -> u64 {
    let mut ranges: HashMap<char, (u64, u64)> = HashMap::new();

    for ch in "xmas".chars() {
        ranges.insert(ch, (1, 4000));
    }

    count_accepted(ranges, &input.workflows, "in".to_string())
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
    fn sample_01(#[case] input: &str, #[case] expected: u64) {
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
