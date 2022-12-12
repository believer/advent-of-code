use std::collections::VecDeque;

// Day 11 - Monkey in the Middle
//
// Welcome aboard the struggle bus against the borrow checker. I had to fight and
// refactor until it was working. It's not pretty, but it works.
//
// For the second part I wasn't sure how to handle the divisor. I ended up
// getting some tips from YouTube which led me to the solution of combining
// all the divisors.
//
// Otherwise the two parts were similar. I might come back and refactor, but
// at this point I'm just tired :D.
//
// Update 2022-12-12: I watched Chris Biscardi's video on this problem and found
// that I needed to move the inspecting, testing, and item handling to the Monkey.
// Of course, it's obvious now that I think about it. Using this I could
// refactor a lot and made the solutions > 80% faster.

#[derive(Debug)]
pub enum Operation {
    Multiply(Option<u64>),
    Add(Option<u64>),
}

#[derive(Debug)]
pub struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    handled_items: u64,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0,
            items: VecDeque::new(),
            operation: Operation::Add(None),
            test: 0,
            if_true: 0,
            if_false: 0,
            handled_items: 0,
        }
    }

    fn set_items(&mut self, line: &str) {
        let (_, data) = line.split_once(':').unwrap();
        let items: VecDeque<u64> = data.split(',').map(|i| i.trim().parse().unwrap()).collect();

        self.items = items;
    }

    fn set_operation(&mut self, line: &str) {
        let mut data = line.split_whitespace();
        let number = data.next_back().unwrap();
        let operation = data.next_back().unwrap();

        let operation = match (operation, number) {
            ("+", "old") => Operation::Add(None),
            ("*", "old") => Operation::Multiply(None),
            ("+", n) => Operation::Add(Some(n.parse::<u64>().unwrap())),
            ("*", n) => Operation::Multiply(Some(n.parse::<u64>().unwrap())),
            _ => panic!("Unknown operation"),
        };

        self.operation = operation;
    }

    fn set_test(&mut self, line: &str) {
        let data = line.split_whitespace().last().unwrap();
        let test = data.parse::<u64>().unwrap();

        self.test = test;
    }

    fn set_if_true(&mut self, line: &str) {
        let test_result = line.split_whitespace().last().unwrap();
        self.if_true = test_result.parse().unwrap();
    }

    fn set_if_false(&mut self, line: &str) {
        let test_result = line.split_whitespace().last().unwrap();
        self.if_false = test_result.parse().unwrap();
    }

    fn set_id(&mut self, line: &str) {
        let id = line.split_whitespace().last().unwrap();
        self.id = id.replace(':', "").parse().unwrap();
    }

    fn inspect_item(&mut self, divisor: Option<u64>) -> u64 {
        self.handled_items += 1;

        let item = self.items.pop_front().unwrap();

        let worry_level = match self.operation {
            Operation::Add(None) => item + item,
            Operation::Multiply(None) => item * item,
            Operation::Add(Some(n)) => n + item,
            Operation::Multiply(Some(n)) => n * item,
        };

        match divisor {
            Some(n) => worry_level % n,
            None => worry_level / 3,
        }
    }

    fn test_item(&mut self, item: u64) -> usize {
        if item % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

fn create_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];

    for m in input.split("\n\n") {
        let mut monkey = Monkey::new();

        for (i, line) in m.lines().enumerate() {
            match i {
                0 => monkey.set_id(line),        // ID
                1 => monkey.set_items(line),     // Items
                2 => monkey.set_operation(line), // Operation
                3 => monkey.set_test(line),      // Test
                4 => monkey.set_if_true(line),   // Test result true
                5 => monkey.set_if_false(line),  // Test result false
                _ => break,
            }
        }

        monkeys.push(monkey);
    }

    monkeys
}

fn monkey_in_the_middle(monkeys: &mut [Monkey], rounds: u32, divisor: Option<u64>) -> u64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let monkey = monkeys.get_mut(i).unwrap();

                let item = monkey.inspect_item(divisor);
                let pass_to_monkey = monkey.test_item(item);

                monkeys
                    .get_mut(pass_to_monkey)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.handled_items);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.handled_items)
        .product()
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_11::*;
/// let data = include_str!("../input/2022/day11.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 66124);
/// ```
#[aoc(day11, part1)]
pub fn solve_part_01(input: &str) -> u64 {
    let mut monkeys = create_monkeys(input);

    monkey_in_the_middle(&mut monkeys, 20, None)
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_11::*;
/// let data = include_str!("../input/2022/day11.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 19309892877);
/// ```
#[aoc(day11, part2)]
pub fn solve_part_02(input: &str) -> u64 {
    let mut monkeys = create_monkeys(input);
    let divisor: u64 = monkeys.iter().map(|m| m.test).product();

    monkey_in_the_middle(&mut monkeys, 10000, Some(divisor))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 10605)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 2713310158)
    }
}
