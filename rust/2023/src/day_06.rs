// Day 6: Wait For It
//
// Today was simple, a nice break from yesterday. I can see multiple ways of improving
// the solution, but I'm happy with it for now.

#[derive(Debug)]
pub struct Input {
    races: Vec<(u64, u64)>,
}

#[aoc_generator(day6, part1)]
pub fn input_generator(input: &str) -> Input {
    let (times, distances) = input.split_once('\n').unwrap();

    let times = times
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let distances = distances
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| (*t, *d))
        .collect::<Vec<_>>();

    Input { races }
}

#[aoc_generator(day6, part2)]
pub fn input_generator_part2(input: &str) -> Input {
    let (times, distances) = input.split_once('\n').unwrap();

    let time = times
        .split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let distance = distances
        .split(':')
        .nth(1)
        .unwrap()
        .replace([' ', '\n'], "")
        .parse::<u64>()
        .unwrap();

    let races = vec![(time, distance)];

    Input { races }
}

/* Part One
*
*/
// Your puzzle answer was
/*
/// ```
/// use advent_of_code_2023::day_06::*;
/// let data = include_str!("../input/2023/day6.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 861300);
/// ```
*/
#[aoc(day6, part1)]
pub fn solve_part_01(input: &Input) -> u64 {
    let Input { races } = input;
    let mut product = 1;

    for (time, record) in races {
        let mut ways_to_beat_record = 0;

        for t in 1..*time {
            let speed = time - t;
            let remaining_time = time - speed;
            let distance_covered = speed * remaining_time;

            if distance_covered > *record {
                ways_to_beat_record += 1;
            }
        }

        product *= ways_to_beat_record;
    }

    product
}

/* Part Two
*
*/
/*
/// ```
/// use advent_of_code_2023::day_06::*;
/// let data = include_str!("../input/2023/day6.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 28101347);
/// ```
*/
#[aoc(day6, part2)]
pub fn solve_part_02(input: &Input) -> u64 {
    let Input { races } = input;
    let mut product = 1;

    for (time, record) in races {
        let mut ways_to_beat_record = 0;

        for t in 1..*time {
            let speed = time - t;
            let remaining_time = time - speed;
            let distance_covered = speed * remaining_time;

            if distance_covered > *record {
                ways_to_beat_record += 1;
            }
        }

        product *= ways_to_beat_record;
    }

    product
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
