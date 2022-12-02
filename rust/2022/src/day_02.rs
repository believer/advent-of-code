// Day 2 - Rock Paper Scissors

type Input = Vec<Vec<String>>;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|l| l.to_string()).collect())
        .collect()
}

/* Part One
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_02::*;
/// let data = include_str!("../input/2022/day2.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 13809);
/// ```
#[aoc(day2, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    let rock = 1;
    let paper = 2;
    let scissors = 3;
    let win = 6;
    let draw = 3;
    let mut my_score = 0;
    let mut their_score = 0;

    // a,x = rock
    // b,y = paper
    // c,z = scissors
    input.iter().for_each(|l| {
        match (l.first(), l.get(1)) {
            (Some(a), Some(b)) => match (a.as_str(), b.as_str()) {
                // My win
                ("A", "Y") => {
                    my_score += win + paper;
                    their_score += rock;
                }
                ("B", "Z") => {
                    my_score += win + scissors;
                    their_score += paper;
                }
                ("C", "X") => {
                    my_score += win + rock;
                    their_score += scissors;
                }

                // Their win
                ("A", "Z") => {
                    their_score += win + rock;
                    my_score += scissors;
                }
                ("B", "X") => {
                    their_score += win + paper;
                    my_score += rock;
                }
                ("C", "Y") => {
                    their_score += win + scissors;
                    my_score += paper;
                }

                // Draw
                ("A", "X") => {
                    my_score += draw + rock;
                    their_score += draw + rock;
                }
                ("B", "Y") => {
                    my_score += draw + paper;
                    their_score += draw + paper;
                }
                ("C", "Z") => {
                    my_score += draw + scissors;
                    their_score += draw + scissors;
                }
                _ => (),
            },
            _ => (),
        }
    });

    my_score
}

/* Part Two
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_02::*;
/// let data = include_str!("../input/2022/day2.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 12316);
/// ```
#[aoc(day2, part2)]
pub fn solve_part_02(input: &Input) -> u32 {
    let rock = 1;
    let paper = 2;
    let scissors = 3;
    let win = 6;
    let draw = 3;
    let mut my_score = 0;
    let mut their_score = 0;

    // a = rock
    // b = paper
    // c = scissors
    // x = loss
    // y = draw
    // z = win
    input.iter().for_each(|l| {
        match (l.first(), l.get(1)) {
            (Some(a), Some(b)) => match (a.as_str(), b.as_str()) {
                // My win
                ("A", "Z") => {
                    my_score += win + paper;
                    their_score += rock;
                }
                ("B", "Z") => {
                    my_score += win + scissors;
                    their_score += paper;
                }
                ("C", "Z") => {
                    my_score += win + rock;
                    their_score += scissors;
                }

                // Their win
                ("A", "X") => {
                    their_score += win + rock;
                    my_score += scissors;
                }
                ("B", "X") => {
                    their_score += win + paper;
                    my_score += rock;
                }
                ("C", "X") => {
                    their_score += win + scissors;
                    my_score += paper;
                }

                // Draw
                ("A", "Y") => {
                    my_score += draw + rock;
                    their_score += draw + rock;
                }
                ("B", "Y") => {
                    my_score += draw + paper;
                    their_score += draw + paper;
                }
                ("C", "Y") => {
                    my_score += draw + scissors;
                    their_score += draw + scissors;
                }
                _ => (),
            },
            _ => (),
        }
    });

    my_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "A Y
B X
C Z";

        assert_eq!(solve_part_01(&input_generator(data)), 15)
    }

    #[test]
    fn sample_02() {
        let data = "A Y
B X
C Z";

        assert_eq!(solve_part_02(&input_generator(data)), 12)
    }
}
