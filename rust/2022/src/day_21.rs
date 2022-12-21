use std::collections::HashMap;

// Day 21 - Monkey Math
//
// The first part was pretty simple. Parse the input and run through
// the equations until we find the result of "root".
//
// The second part was trickier. I was able to get the correct answer for the
// test, but it didn't work every time. Something was off somewhere. I decided
// to rethink and simplify the solutions from the ground up. Went with the approach
// of going backwards from "root" until we find "humn". The real result was actually
// off by a few digits. Don't know if there was a rounding error somewhere...
//
// The first part even got ~93% faster with the updated solution.

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Multiply,
    Divide,
    Subtract,
    Equals,
}

type Monkey = String;
type Equation = (Operation, Monkey, Monkey);
type AllNumbers = HashMap<Monkey, isize>;
type AllEquations = HashMap<Monkey, Equation>;
type Input = (AllNumbers, AllEquations);

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Input {
    let mut numbers = HashMap::new();
    let mut equations = HashMap::new();

    input.lines().for_each(|line| {
        let monkey = line.split([':', ' ']).collect::<Vec<_>>();

        // Numbers are length 3 - Monkey name, empty space, number
        if monkey.len() == 3 {
            numbers.insert(monkey[0].to_string(), monkey[2].parse().unwrap());
        // All that's left are equations
        } else {
            let operation = match monkey[3] {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                "=" => Operation::Equals,
                _ => panic!("Unknown operation"),
            };

            equations.insert(
                monkey[0].to_string(),
                (operation, monkey[2].to_string(), monkey[4].to_string()),
            );
        }
    });

    (numbers, equations)
}

fn evaluate_monkeys(numbers: &mut AllNumbers, equations: &mut AllEquations) {
    loop {
        let number_of_equations = equations.len();

        equations.retain(|monkey, (operation, lhs, rhs)| {
            let result = match (operation, numbers.get(lhs), numbers.get(rhs)) {
                (Operation::Add, Some(lhs), Some(rhs)) => lhs + rhs,
                (Operation::Subtract, Some(lhs), Some(rhs)) => lhs - rhs,
                (Operation::Multiply, Some(lhs), Some(rhs)) => lhs * rhs,
                (Operation::Divide, Some(lhs), Some(rhs)) => lhs / rhs,
                // Save it for later
                _ => return true,
            };

            numbers.insert(monkey.to_string(), result);

            false
        });

        // Nothing further to evaluate
        if number_of_equations == equations.len() {
            break;
        }
    }
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
pub fn solve_part_01(monkeys: &Input) -> isize {
    let (mut numbers, mut equations) = monkeys.clone();

    evaluate_monkeys(&mut numbers, &mut equations);

    numbers["root"]
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_21::*;
/// let data = include_str!("../input/2022/day21.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 3712643961892);
/// ```
#[aoc(day21, part2)]
pub fn solve_part_02(monkeys: &Input) -> isize {
    let (mut numbers, mut equations) = monkeys.clone();

    // Update root to equals operation
    equations.get_mut("root").unwrap().0 = Operation::Equals;

    // Remove human from numbers as that is what we're looking for
    numbers.remove("humn");

    // Evaluate all possible equations
    evaluate_monkeys(&mut numbers, &mut equations);

    let mut search = "root";
    let mut result = 0;

    // Work backwards from "root" to "humn" to find the result
    while search != "humn" {
        let (op, lhs, rhs) = &equations[search];

        (search, result) = match (op, numbers.get(lhs), numbers.get(rhs)) {
            (Operation::Equals, None, Some(x)) => (lhs, *x),
            (Operation::Equals, Some(x), None) => (rhs, *x),
            (Operation::Add, None, Some(x)) => (lhs, result - x),
            (Operation::Add, Some(x), None) => (rhs, result - x),
            (Operation::Subtract, None, Some(x)) => (lhs, result + x),
            (Operation::Subtract, Some(x), None) => (rhs, x - result),
            (Operation::Multiply, None, Some(x)) => (lhs, result / x),
            (Operation::Multiply, Some(x), None) => (rhs, result / x),
            (Operation::Divide, None, Some(x)) => (lhs, result * x),
            (Operation::Divide, Some(x), None) => (rhs, x / result),
            _ => panic!(),
        };
    }

    result
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

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 301)
    }
}
