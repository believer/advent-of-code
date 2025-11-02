use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, HashSet};

// Day 21 - Allergen Assessment

lazy_static! {
    static ref FOOD: Regex =
        Regex::new(r"^(?P<ingredients>.*) \(contains (?P<allergens>.*)\)").unwrap();
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> (Vec<String>, BTreeMap<String, String>) {
    let mut possible_allergens = BTreeMap::new();
    let mut allergens = BTreeMap::new();
    let mut foods = vec![];

    for food in input.lines() {
        let caps = FOOD.captures(food).unwrap();
        let parsed_ingredients = caps.name("ingredients").unwrap().as_str();
        let allergens = caps.name("allergens").unwrap().as_str().split(',');
        let ingredients = parsed_ingredients
            .split_whitespace()
            .collect::<HashSet<_>>();

        foods.push(parsed_ingredients.to_string());

        for allergen in allergens {
            let set = possible_allergens
                .entry(allergen.trim())
                .or_insert_with(|| ingredients.clone());

            *set = set.intersection(&ingredients).copied().collect();
        }
    }

    while let Some((&a, _)) = possible_allergens.iter().find(|(_, s)| s.len() == 1) {
        let &i = possible_allergens[a].iter().next().unwrap();

        allergens.insert(a.to_string(), i.to_string());

        for s in possible_allergens.values_mut() {
            s.remove(&i);
        }
    }

    (foods, allergens)
}

/* Part One
 *
 * You reach the train's last stop and the closest you can get to your vacation island without getting wet.
 * There aren't even any boats here, but nothing can stop you now: you build a raft.
 * You just need a few days' worth of food for your journey.
 *
 * You don't speak the local language, so you can't read any ingredients lists.
 * However, sometimes, allergens are listed in a language you do understand.
 * You should be able to use this information to determine which ingredient contains which
 * allergen and work out which foods are safe to take with you on your trip.
 *
 * You start by compiling a list of foods (your puzzle input), one food per line.
 * Each line includes that food's ingredients list followed by some or all of the allergens the food contains.
 *
 * Each allergen is found in exactly one ingredient. Each ingredient contains zero or one allergen.
 * Allergens aren't always marked; when they're listed (as in (contains nuts, shellfish) after an ingredients list),
 * the ingredient that contains each listed allergen will be somewhere in the corresponding ingredients list.
 * However, even if an allergen isn't listed, the ingredient that contains that allergen could still be present:
 * maybe they forgot to label it, or maybe it was labeled in a language you don't know.
 *
 * For example, consider the following list of foods:
 *
 * mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
 * trh fvjkl sbzzf mxmxvkd (contains dairy)
 * sqjhc fvjkl (contains soy)
 * sqjhc mxmxvkd sbzzf (contains fish)
 *
 * The first food in the list has four ingredients (written in a language you don't understand):
 * mxmxvkd, kfcds, sqjhc, and nhms. While the food might contain other allergens, a few allergens the food definitely contains are listed afterward: dairy and fish.
 *
 * The first step is to determine which ingredients can't possibly contain any of the allergens in any food in your list.
 * In the above example, none of the ingredients kfcds, nhms, sbzzf, or trh can contain an allergen.
 * Counting the number of times any of these ingredients appear in any ingredients list produces 5: they all appear once each except sbzzf, which appears twice.
 *
 * Determine which ingredients cannot possibly contain any of the allergens in your list. How many times do any of those ingredients appear?
*/
#[aoc(day21, part1)]
pub fn solve_part_01((foods, allergens): &(Vec<String>, BTreeMap<String, String>)) -> usize {
    let allergens = allergens
        .values()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();

    foods
        .iter()
        .flat_map(|ingredient| ingredient.split_whitespace())
        .filter(|ingredient| !allergens.contains(*ingredient))
        .count()
}

/* Part Two
 *
 * Now that you've isolated the inert ingredients, you should have enough information
 * to figure out which ingredient contains which allergen.
 *
 * In the above example:
 *
 * mxmxvkd contains dairy.
 * sqjhc contains fish.
 * fvjkl contains soy.
 * Arrange the ingredients alphabetically by their allergen and separate them by commas to produce your canonical dangerous ingredient list.
 * (There should not be any spaces in your canonical dangerous ingredient list.) In the above example, this would be mxmxvkd,sqjhc,fvjkl.
 *
 * Time to stock your raft with supplies. What is your canonical dangerous ingredient list?
 */
#[aoc(day21, part2)]
pub fn solve_part_02((_, allergens): &(Vec<String>, BTreeMap<String, String>)) -> String {
    allergens.values().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        assert_eq!(solve_part_01(&input_generator(data)), 5)
    }

    #[test]
    fn sample_02() {
        let data = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

        assert_eq!(
            solve_part_02(&input_generator(data)),
            "mxmxvkd,sqjhc,fvjkl".to_string()
        )
    }
}
