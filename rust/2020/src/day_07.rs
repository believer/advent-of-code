use crate::common;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

// Day 7 - Handy Haversacks

lazy_static! {
    static ref COLOR_RE: Regex = Regex::new(r"^(.+?) bags contain").unwrap();
    static ref BAG_RE: Regex = Regex::new(r"(\d+) (.+?) bags?[,.]").unwrap();
}

type BagsCanContain = HashMap<String, HashSet<String>>;
type BagsRequired = HashMap<String, HashMap<String, u32>>;

#[aoc_generator(day7, part1)]
pub fn input_generator_part_01(input: &str) -> Option<BagsCanContain> {
    let data: Vec<String> = common::input_vec(input);

    let mut bag_in: HashMap<String, HashSet<String>> = HashMap::new();

    for bag in data {
        let colors = COLOR_RE.captures(&bag)?;

        for b in BAG_RE.captures_iter(&bag) {
            let color = colors.get(1)?.as_str().to_string();
            let inner_color = b[2].to_string();

            if let Some(inner) = bag_in.get_mut(&inner_color) {
                inner.insert(color);
            } else {
                let mut data = HashSet::new();
                data.insert(color);
                bag_in.insert(inner_color, data);
            }
        }
    }

    Some(bag_in)
}

#[aoc_generator(day7, part2)]
pub fn input_generator_part_02(input: &str) -> Option<BagsRequired> {
    let data: Vec<String> = common::input_vec(input);

    let mut bag_contains: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for bag in data {
        let colors = COLOR_RE.captures(&bag).unwrap();

        for b in BAG_RE.captures_iter(&bag) {
            let color = colors.get(1)?.as_str().to_string();
            let number_of_bags = b[1].to_string().parse().unwrap();
            let inner_color = b[2].to_string();

            if let Some(inner) = bag_contains.get_mut(&color) {
                inner.insert(inner_color, number_of_bags);
            } else {
                let mut data = HashMap::new();
                data.insert(inner_color, number_of_bags);
                bag_contains.insert(color, data);
            }
        }
    }

    Some(bag_contains)
}

fn search(input: &BagsCanContain, holds_gold_bags: &mut HashSet<String>, color: String) {
    if let Some(c) = input.get(&color) {
        for cc in c.iter() {
            holds_gold_bags.insert(cc.to_string());
            search(input, holds_gold_bags, cc.to_string());
        }
    }
}

/* Part One
 *
 * You land at the regional airport in time for your next flight.
 * In fact, it looks like you'll even have time to grab some food:
 * all flights are currently delayed due to issues in luggage processing.
 *
 * Due to recent aviation regulations, many rules (your puzzle input)
 * are being enforced about bags and their contents; bags must be color-coded and
 * must contain specific quantities of other color-coded bags.
 * Apparently, nobody responsible for these regulations considered how long they would take to enforce!
 *
 * For example, consider the following rules:
 *
 * light red bags contain 1 bright white bag, 2 muted yellow bags.
 * dark orange bags contain 3 bright white bags, 4 muted yellow bags.
 * bright white bags contain 1 shiny gold bag.
 * muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
 * shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
 * dark olive bags contain 3 faded blue bags, 4 dotted black bags.
 * vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
 * faded blue bags contain no other bags.
 * dotted black bags contain no other bags.
 * These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.
 *
 * You have a shiny gold bag. If you wanted to carry it in at least one other bag,
 * how many different bag colors would be valid for the outermost bag?
 * (In other words: how many colors can, eventually, contain at least one shiny gold bag?)
 *
 * In the above rules, the following options would be available to you:
 *
 * A bright white bag, which can hold your shiny gold bag directly.
 * A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
 * A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
 * A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
 * So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.
 *
 * How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
 */
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_07::*;
/// let input = include_str!("../input/2020/day7.txt");
/// assert_eq!(solve_part_01(&input_generator_part_01(input).unwrap()), 226);
/// ```
#[aoc(day7, part1)]
pub fn solve_part_01(input: &BagsCanContain) -> usize {
    let mut holds_gold_bags: HashSet<String> = HashSet::new();
    search(input, &mut holds_gold_bags, "shiny gold".to_string());

    holds_gold_bags.len()
}

fn calculate_cost(input: &BagsRequired, color: &str) -> u32 {
    let mut total = 0;

    if let Some(c) = input.get(color) {
        for (c, ct) in c {
            total += ct;
            total += ct * calculate_cost(input, c);
        }
    }

    total
}

/* Part Two
 *
 * It's getting pretty expensive to fly these days - not because of ticket prices,
 * but because of the ridiculous number of bags you need to buy!
 *
 * Consider again your shiny gold bag and the rules from the above example:
 *
 * faded blue bags contain 0 other bags.
 * dotted black bags contain 0 other bags.
 * vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
 * dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
 * So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it)
 * plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
 *
 * Of course, the actual rules have a small chance of going several levels deeper than this example;
 * be sure to count all of the bags, even if the nesting becomes topologically impractical!
 *
 * Here's another example:
 *
 * shiny gold bags contain 2 dark red bags.
 * dark red bags contain 2 dark orange bags.
 * dark orange bags contain 2 dark yellow bags.
 * dark yellow bags contain 2 dark green bags.
 * dark green bags contain 2 dark blue bags.
 * dark blue bags contain 2 dark violet bags.
 * dark violet bags contain no other bags.
 * In this example, a single shiny gold bag must contain 126 other bags.
 *
 * How many individual bags are required inside your single shiny gold bag?
 */
///your puzzle answer was.
/// ```
/// use advent_of_code_2020::day_07::*;
/// let input = include_str!("../input/2020/day7.txt");
/// assert_eq!(solve_part_02(&input_generator_part_02(input).unwrap()), 9569);
/// ```
#[aoc(day7, part2)]
pub fn solve_part_02(input: &BagsRequired) -> u32 {
    calculate_cost(input, "shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn sample_01() {
        let data = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

        assert_eq!(solve_part_01(&input_generator_part_01(data).unwrap()), 4)
    }

    /// Test example data on part 2
    #[test]
    fn sample_02() {
        let data = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

        assert_eq!(solve_part_02(&input_generator_part_02(data).unwrap()), 32)
    }
}
