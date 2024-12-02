// Day 2 - Rock Paper Scissors

type Game = Vec<Vec<String>>;

struct Score;

impl Score {
    pub const ROCK: u32 = 1;
    pub const PAPER: u32 = 2;
    pub const SCISSORS: u32 = 3;
    pub const WIN: u32 = 6;
    pub const DRAW: u32 = 3;
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Game {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|l| l.to_string()).collect())
        .collect()
}

/* Part One
 *
 * We are playing a game of rock, paper, scissors. We are provided
 * with a strategy guide, with the opponent's selection first and
 * our selection second. For example:
 *
 * A Y
 * B X
 * C Z
 *
 * From this we suppose that:
 *
 * A = Rock, Y = Paper (Win)
 * B = Paper, X = Rock (Loss)
 * C = Scissors, Z = Scissors (Draw)
 *
 * Each round is scored differently:
 *
 * Win = 6 points + points based on selection
 * Loss = 0 points + points based on selection
 * Draw = 3 points + points based on selection
 *
 * Paper is worth 2 points, Rock is worth 1 point, Scissors is worth 3 points.
 *
 * Following the example that would give us a score of 15 ((6 + 2) + (0 + 1) + (3 + 3))
 * What is our total score if we follow this strategy guide?
*/
#[aoc(day2, part1)]
pub fn solve_part_01(game: &Game) -> u32 {
    game.iter()
        .map(|l| {
            match (l.first(), l.get(1)) {
                (Some(opponent), Some(me)) => match (opponent.as_str(), me.as_str()) {
                    // My win, we get win points and selection points
                    ("A", "Y") => Score::WIN + Score::PAPER,
                    ("B", "Z") => Score::WIN + Score::SCISSORS,
                    ("C", "X") => Score::WIN + Score::ROCK,

                    // Their win, we get selection points
                    ("A", "Z") => Score::SCISSORS,
                    ("B", "X") => Score::ROCK,
                    ("C", "Y") => Score::PAPER,

                    // Draw, split win points and get selection points
                    ("A", "X") => Score::DRAW + Score::ROCK,
                    ("B", "Y") => Score::DRAW + Score::PAPER,
                    ("C", "Z") => Score::DRAW + Score::SCISSORS,

                    _ => 0,
                },
                _ => 0,
            }
        })
        .sum()
}

/* Part Two
 *
 * We learn that the second column in the strategy guide is not our
 * selection, but the desired outcome:
 *
 * X = Loss
 * Y = Draw
 * Z = Win
 *
 * We have to change our strategy to match the desired outcome.
 * What is our total score if we follow this guide?
*/
#[aoc(day2, part2)]
pub fn solve_part_02(game: &Game) -> u32 {
    game.iter()
        .map(|l| {
            match (l.first(), l.get(1)) {
                (Some(a), Some(b)) => match (a.as_str(), b.as_str()) {
                    // My win, we get win points and selection points
                    ("A", "Z") => Score::WIN + Score::PAPER,
                    ("B", "Z") => Score::WIN + Score::SCISSORS,
                    ("C", "Z") => Score::WIN + Score::ROCK,

                    // Their win, we get selection points
                    ("A", "X") => Score::SCISSORS,
                    ("B", "X") => Score::ROCK,
                    ("C", "X") => Score::PAPER,

                    // Draw, split win points and get selection points
                    ("A", "Y") => Score::DRAW + Score::ROCK,
                    ("B", "Y") => Score::DRAW + Score::PAPER,
                    ("C", "Y") => Score::DRAW + Score::SCISSORS,

                    _ => 0,
                },
                _ => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 15)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 12)
    }
}
