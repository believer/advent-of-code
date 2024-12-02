// Day 6: Wait For It
//
// Today was simple, a nice break from yesterday. I can see multiple ways of improving
// the solution, but I'm happy with it for now.

#[derive(Debug)]
pub struct Input {
    races: Vec<(u64, u64)>,
}

fn parse_numbers(s: &str) -> Vec<u64> {
    s.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_merged_numbers(s: &str) -> u64 {
    s.chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[aoc_generator(day6, part1)]
pub fn input_generator(input: &str) -> Input {
    let (times, distances) = input.split_once('\n').unwrap();

    let times = parse_numbers(times);
    let distances = parse_numbers(distances);

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| (*t, *d))
        .collect::<Vec<_>>();

    Input { races }
}

#[aoc_generator(day6, part2)]
pub fn input_generator_part2(input: &str) -> Input {
    let lines = input.lines().collect::<Vec<_>>();

    let time = parse_merged_numbers(lines[0]);
    let distance = parse_merged_numbers(lines[1]);

    let races = vec![(time, distance)];

    Input { races }
}

fn race(races: &[(u64, u64)]) -> u64 {
    races
        .iter()
        .map(|(time, record)| {
            (1..*time).fold(0, |acc, t| {
                let speed = time - t;
                let remaining_time = time - speed;
                let distance_covered = speed * remaining_time;

                if distance_covered > *record {
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .product::<u64>()
}

/* Part One
*
* We get a table of race times and the record distance. We need to find how many
* ways there are to beat the record. The race starts by holding a button to build
* up speed for a certain amount of time, and then releasing it. The distance
* covered is the speed times the remaining time.
*
*/
#[aoc(day6, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    race(&input.races)
}

/* Part Two
*
* Here we combine all the race times and distance from the previous data into
* two large numbers. This means we only have one race, but since the time
* is longer, we have a lot more ways to beat the record.
*
*/
#[aoc(day6, part2)]
pub fn solve_part_02(input: &Input) -> u64 {
    race(&input.races)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 288)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator_part2(DATA)), 71503)
    }
}
