// Day 2 - Rock Paper Scissors

type Game = Vec<Vec<String>>;

#[non_exhaustive]
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
 * The Elves begin to set up camp on the beach.
 * To decide whose tent gets to be closest to the snack storage,
 * a giant Rock Paper Scissors tournament is already in progress.
 *
 * Rock Paper Scissors is a Score between two players.
 * Each Score contains many rounds; in each round, the players each
 * simultaneously choose one of Rock, Paper, or Scissors using a hand shape.
 * Then, a winner for that round is selected: Rock defeats Scissors,
 * Scissors defeats Paper, and Paper defeats Rock.
 * If both players choose the same shape, the round instead ends in a draw.
 *
 * Appreciative of your help yesterday, one Elf gives you an encrypted
 * strategy guide (your puzzle input) that they say will be sure to help you win.
 * "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors.
 * The second column--" Suddenly, the Elf is called away to help with someone's tent.
 *
 * The second column, you reason, must be what you should play in
 * response: X for Rock, Y for Paper, and Z for Scissors.
 * Winning every time would be suspicious, so the responses must have been carefully chosen.
 *
 * The winner of the whole tournament is the player with the highest score.
 * Your total score is the sum of your scores for each round.
 * The score for a single round is the score for the shape you
 * selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for
 * the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
 *
 * Since you can't be sure if the Elf is trying to help you or trick you,
 * you should calculate the score you would get if you were to follow the strategy guide.
 *
 * For example, suppose you were given the following strategy guide:
 *
 * A Y
 * B X
 * C Z
 *
 * This strategy guide predicts and recommends the following:
 *
 * In the first round, your opponent will choose Rock (A), and you
 * should choose Paper (Y). This ends in a win for you with a
 * score of 8 (2 because you chose Paper + 6 because you won).
 *
 * In the second round, your opponent will choose Paper (B),
 * and you should choose Rock (X). This ends in a loss for
 * you with a score of 1 (1 + 0).
 *
 * The third round is a draw with both players choosing
 * Scissors, giving you a score of 3 + 3 = 6.
 *
 * In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
 * What would your total score be if everything goes exactly according to your strategy guide?
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_02::*;
/// let data = include_str!("../input/2022/day2.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 13809);
/// ```
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
 * The Elf finishes helping with the tent and sneaks back over to you.
 * "Anyway, the second column says how the round needs to end:
 * X means you need to lose, Y means you need to end the round in a draw,
 * and Z means you need to win. Good luck!"
 *
 * The total score is still calculated in the same way,
 * but now you need to figure out what shape to choose so
 * the round ends as indicated. The example above now goes like this:
 *
 * In the first round, your opponent will choose Rock (A),
 * and you need the round to end in a draw (Y), so you also choose Rock.
 * This gives you a score of 1 + 3 = 4.
 *
 * In the second round, your opponent will choose Paper (B),
 * and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
 *
 * In the third round, you will defeat your opponent's Scissors
 * with Rock for a score of 1 + 6 = 7.
 *
 * Now that you're correctly decrypting the ultra top secret strategy guide,
 * you would get a total score of 12.
 *
 * Following the Elf's instructions for the second column,
 * what would your total score be if everything goes exactly according to your strategy guide?
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_02::*;
/// let data = include_str!("../input/2022/day2.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 12316);
/// ```
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
