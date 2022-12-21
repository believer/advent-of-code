use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair, *,
};
use std::collections::HashMap;

// Day 21 - Monkey Math

type Input = Vec<Monkey>;

#[derive(Debug)]
pub enum Operation {
    Add((String, String)),
    Multiply((String, String)),
    Divide((String, String)),
    Subtract((String, String)),
}

#[derive(Debug)]
pub enum MonkeyYell {
    Number(u64),
    Operation(Operation),
}

#[derive(Debug)]
pub struct Monkey {
    name: String,
    yell: MonkeyYell,
}

impl Monkey {
    fn new(name: String, yell: MonkeyYell) -> Self {
        Monkey { name, yell }
    }
}

fn parse_operation(input: &str) -> IResult<&str, Monkey> {
    let (input, name) = complete::alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, left) = complete::alpha1(input)?;
    let (input, _) = complete::space0(input)?;
    let (input, op) = complete::one_of("+-*/")(input)?;
    let (input, _) = complete::space0(input)?;
    let (input, right) = complete::alpha1(input)?;

    let operation = match op {
        '+' => Operation::Add((left.to_string(), right.to_string())),
        '-' => Operation::Subtract((left.to_string(), right.to_string())),
        '*' => Operation::Multiply((left.to_string(), right.to_string())),
        '/' => Operation::Divide((left.to_string(), right.to_string())),
        _ => unreachable!(),
    };

    Ok((
        input,
        Monkey::new(name.to_string(), MonkeyYell::Operation(operation)),
    ))
}

// abcd: 1234
fn parse_number(input: &str) -> IResult<&str, Monkey> {
    let (input, output) = separated_pair(complete::alpha1, tag(": "), complete::u64)(input)?;

    Ok((
        input,
        Monkey::new(output.0.to_string(), MonkeyYell::Number(output.1)),
    ))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    branch::alt((parse_number, parse_operation))(input)
}

fn parse_monkeys(input: &str) -> IResult<&str, Input> {
    separated_list1(complete::newline, parse_monkey)(input)
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Input {
    parse_monkeys(input).unwrap().1
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_21::*;
/// let data = include_str!("../input/2022/day21.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 170237589447588);
/// ```
#[aoc(day21, part1)]
pub fn solve_part_01(monkeys: &Input) -> u64 {
    let mut monkey_math: HashMap<String, u64> = HashMap::new();

    loop {
        if monkey_math.get(&"root".to_string()).is_some() {
            break;
        }

        for monkey in monkeys {
            match &monkey.yell {
                MonkeyYell::Number(number) => {
                    monkey_math.insert(monkey.name.clone(), *number);
                }
                MonkeyYell::Operation(operation) => {
                    let (left, right) = match operation {
                        Operation::Add((left, right)) => (left, right),
                        Operation::Subtract((left, right)) => (left, right),
                        Operation::Multiply((left, right)) => (left, right),
                        Operation::Divide((left, right)) => (left, right),
                    };

                    let left = monkey_math.get(left);
                    let right = monkey_math.get(right);

                    if left.is_none() || right.is_none() {
                        continue;
                    }

                    let left = *left.unwrap();
                    let right = *right.unwrap();

                    let result = match operation {
                        Operation::Add(_) => left + right,
                        Operation::Subtract(_) => left - right,
                        Operation::Multiply(_) => left * right,
                        Operation::Divide(_) => left / right,
                    };

                    monkey_math.insert(monkey.name.clone(), result);
                }
            }
        }
    }

    *monkey_math.get(&"root".to_string()).unwrap()
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_21::*;
/// let data = include_str!("../input/2022/day21.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 0);
/// ```
#[aoc(day21, part2)]
pub fn solve_part_02(_input: &Input) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 152)
    }
}
