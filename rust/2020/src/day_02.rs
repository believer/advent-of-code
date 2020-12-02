use std::ops::RangeInclusive;

// Day 2 - Password Philosophy

pub struct Password {
    password: String,
    policy: String,
    policy_bounds: RangeInclusive<usize>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let password_terms: Vec<_> = l.split_whitespace().collect();
            let password = password_terms[2].trim().to_string();
            let bounds: Vec<usize> = password_terms[0]
                .split("-")
                .map(|x| x.parse().unwrap())
                .collect();
            let policy = password_terms[1][..1].to_string();

            Password {
                password,
                policy,
                policy_bounds: RangeInclusive::new(bounds[0], bounds[1]),
            }
        })
        .collect()
}

/* Part One
 *
 * Your flight departs in a few days from the coastal airport;
 * the easiest way down to the coast from here is via toboggan.
 *
 * The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
 * "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
 *
 * Their password database seems to be a little corrupted:
 * some of the passwords wouldn't have been allowed by the
 * Official Toboggan Corporate Policy that was in effect when they were chosen.
 *
 * To try to debug the problem, they have created a list (your puzzle input)
 * of passwords (according to the corrupted database)
 * and the corporate policy when that password was set.
 *
 * For example, suppose you have the following list:
 *
 * 1-3 a: abcde
 * 1-3 b: cdefg
 * 2-9 c: ccccccccc
 *
 * Each line gives the password policy and then the password.
 * The password policy indicates the lowest and highest number of times
 * a given letter must appear for the password to be valid.
 * For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
 *
 * In the above example, 2 passwords are valid. The middle password, cdefg, is not;
 * it contains no instances of b, but needs at least 1. The first and third passwords are valid:
 * they contain one a or nine c, both within the limits of their respective policies.
 *
 * How many passwords are valid according to their policies?
*/
///Your puzzle answer was 524.
/// ```
/// use advent_of_code_2020::day_02::*;
/// let input = include_str!("../input/2020/day2.txt");
/// assert_eq!(solve_part_01(&input_generator(input)), 524);
/// ```
#[aoc(day2, part1)]
pub fn solve_part_01(input: &Vec<Password>) -> u32 {
    input.iter().fold(0, |acc, password| {
        let matched: Vec<_> = password.password.match_indices(&password.policy).collect();

        match password.policy_bounds.contains(&matched.len()) {
            true => acc + 1,
            false => acc,
        }
    })
}

/* Part Two
 *
 * While it appears you validated the passwords correctly,
 * they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
 *
 * The shopkeeper suddenly realizes that he just accidentally explained the password
 * policy rules from his old job at the sled rental place down the street!
 * The Official Toboggan Corporate Policy actually works a little differently.
 *
 * Each policy actually describes two positions in the password, where 1 means the first character,
 * 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!)
 * Exactly one of these positions must contain the given letter.
 * Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
 *
 * Given the same example list from above:
 *
 * 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
 * 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
 * 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
 *
 * How many passwords are valid according to the new interpretation of the policies?
*/
///Your puzzle answer was 485.
/// ```
/// use advent_of_code_2020::day_02::*;
/// let input = include_str!("../input/2020/day2.txt");
/// assert_eq!(solve_part_02(&input_generator(input)), 485);
/// ```
#[aoc(day2, part2)]
pub fn solve_part_02(input: &Vec<Password>) -> u32 {
    input.iter().fold(0, |acc, password| {
        let (lower, upper) = password.policy_bounds.clone().into_inner();
        let inner =
            password
                .password
                .match_indices(&password.policy)
                .fold(0, |acc, (i, _)| match (i + 1 == lower, i + 1 == upper) {
                    (false, false) => acc,
                    _ => acc + 1,
                });

        if inner == 1 {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test example data on part 1
    #[test]
    fn sample_01() {
        let data = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

        assert_eq!(solve_part_01(&input_generator(data)), 2)
    }

    /// Test example data on part 2
    #[test]
    fn sample_02() {
        let data = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

        assert_eq!(solve_part_02(&input_generator(data)), 1)
    }
}
