// Day 1 - Calorie Counting

type Fuel = Vec<i32>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Fuel {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

fn calculate_fuel(mass: &i32) -> i32 {
    mass / 3 - 2
}

fn calculate_fuel_with_extra(mass: &i32, total_fuel: i32) -> i32 {
    match calculate_fuel(mass) {
        fuel if fuel <= 0 => total_fuel,
        fuel => calculate_fuel_with_extra(&fuel, total_fuel + fuel),
    }
}

/* Part One
 */
#[aoc(day1, part1)]
pub fn solve_part_01(fuel: &Fuel) -> i32 {
    fuel.iter().map(calculate_fuel).sum()
}

/* Part Two
*/
#[aoc(day1, part2)]
pub fn solve_part_02(fuel: &Fuel) -> i32 {
    fuel.iter().map(|f| calculate_fuel_with_extra(f, 0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "12
14
1969
100756";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 34241)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 51316)
    }
}
