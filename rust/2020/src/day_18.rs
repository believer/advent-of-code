use lazy_static::lazy_static;
use regex::Regex;

// Day 18 - Operation Order

lazy_static! {
    static ref PAREN: Regex = Regex::new(r"\(([^\(\)]+)\)").unwrap();
}

fn calculator(exp: &str) -> u64 {
    let mut sum = 0;
    let mut op = "+";

    for value in exp.split_whitespace() {
        match value {
            "+" => op = "+",
            "*" => op = "*",
            v => {
                let number: u64 = v.parse().unwrap();

                match op {
                    "+" => sum += number,
                    "*" => sum *= number,
                    _ => unreachable!(),
                };
            }
        }
    }

    sum
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<String> {
    let mut expressions = Vec::new();

    for line in input.lines() {
        let mut expression = line.to_owned();

        while expression.contains("(") {
            let paren = PAREN.captures(&expression).unwrap();
            let old_exp = paren.get(0).unwrap().as_str();
            let exp = paren.get(1).unwrap().as_str();

            expression = expression.replace(old_exp, &calculator(exp).to_string());
        }

        expressions.push(expression)
    }

    expressions
}

/* Part One
 *
 * As you look out the window and notice a heavily-forested continent slowly
 * appear over the horizon, you are interrupted by the child sitting next to you.
 * They're curious if you could help them with their math homework.

 * Unfortunately, it seems like this "math" follows different rules than you remember.
 *
 * The homework (your puzzle input) consists of a series of expressions that
 * consist of addition (+), multiplication (*), and parentheses ((...)).
 * Just like normal math, parentheses indicate that the expression inside must
 * be evaluated before it can be used by the surrounding expression.
 * Addition still finds the sum of the numbers on both sides of the operator,
 * and multiplication still finds the product.
 *
 * However, the rules of operator precedence have changed.
 * Rather than evaluating multiplication before addition, the operators have the same precedence,
 * and are evaluated left-to-right regardless of the order in which they appear.
 *
 * For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are as follows:
 *
 * 1 + 2 * 3 + 4 * 5 + 6
 *   3   * 3 + 4 * 5 + 6
 *       9   + 4 * 5 + 6
 *          13   * 5 + 6
 *              65   + 6
 *                  71
 *
 * Parentheses can override this order; for example, here is what
 * happens if parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):
 *
 * 1 + (2 * 3) + (4 * (5 + 6))
 * 1 +    6    + (4 * (5 + 6))
 *      7      + (4 * (5 + 6))
 *      7      + (4 *   11   )
 *      7      +     44
 *             51
 *
 * Here are a few more examples:
 *
 * 2 * 3 + (4 * 5) becomes 26.
 * 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
 * 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
 * ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
 *
 * Before you can help with the homework, you need to understand it yourself.
 * Evaluate the expression on each line of the homework; what is the sum of the resulting values?
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2020::day_18::*;
/// let input = include_str!("../input/2020/day18.txt");
/// assert_eq!(solve_part_01(&input_generator_part_01(input)), 29839238838303);
/// ```
#[aoc(day18, part1)]
pub fn solve_part_01(input: &[String]) -> u64 {
    input
        .iter()
        .fold(0, |acc, expression| acc + calculator(expression))
}

// #[aoc(day18, part2)]
// pub fn solve_part_02(input: &[HyperCube]) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    // Test example data on part 1
    #[test]
    fn test_example_part_1() {
        let data = "2 * 3 + (4 * 5)";

        assert_eq!(solve_part_01(&input_generator(data)), 26)
    }
    // Test example data on part 1
    #[test]
    fn test_multiple_lines() {
        let data = "2 * 3 + (4 * 5)
2 * 3 + (4 * 5)";

        assert_eq!(solve_part_01(&input_generator(data)), 52)
    }

    // Test example data on part 1
    #[test]
    fn test_example_part_1_2() {
        let data = "5 + (8 * 3 + 9 + 3 * 4 * 3)";

        assert_eq!(solve_part_01(&input_generator(data)), 437)
    }

    // Test example data on part 1
    #[test]
    fn test_example_part_1_3() {
        let data = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";

        assert_eq!(solve_part_01(&input_generator(data)), 12240)
    }

    // Test example data on part 1
    #[test]
    fn test_example_part_1_4() {
        let data = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

        assert_eq!(solve_part_01(&input_generator(data)), 13632)
    }
}
