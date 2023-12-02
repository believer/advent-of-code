use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};
use std::collections::HashMap;

// Day 2: Cube Conundrum
//
// Today was fairly straightforward. I went with a simple parser at
// first, but refactored it to use nom. This made the parser about 34%
// faster.

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

impl std::str::FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Cube {
    color: Color,
    count: u32,
}

type Round = Vec<Cube>;
type Game = Vec<Round>;
type Games = HashMap<u32, Game>;

fn parse_id(input: &str) -> nom::IResult<&str, u32> {
    preceded(tag("Game "), complete::u32)(input)
}

fn parse_cube(input: &str) -> nom::IResult<&str, Cube> {
    let input = input.trim();
    let (input, (count, color)) =
        separated_pair(complete::u32, complete::space1, complete::alpha1)(input)?;

    let color = color.parse::<Color>().unwrap();

    Ok((input, Cube { color, count }))
}

fn parse_round(input: &str) -> nom::IResult<&str, Round> {
    separated_list1(tag(", "), parse_cube)(input)
}

fn parse_game(input: &str) -> nom::IResult<&str, Game> {
    separated_list1(tag("; "), parse_round)(input)
}

fn parse_line(input: &str) -> nom::IResult<&str, (u32, Game)> {
    separated_pair(parse_id, tag(": "), parse_game)(input)
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<(u32, Game)>> {
    separated_list1(complete::line_ending, parse_line)(input)
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Games {
    let (_, games) = parse_input(input).unwrap();

    games.into_iter().collect()
}

/* Part One
*
* How many games are possible if there can be at most:
* 12 red cubes, 13 green cubes, and 14 blue cubes in any round.
*
* Sum up the game IDs of all possible games.
*
* Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
* Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
* Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
* Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
* Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
*
* Games 1, 2, and 5 are possible. The other games have too many cubes of a single color.
* The sum of the game IDs is 1 + 2 + 5 = 8.
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_02::*;
/// let data = include_str!("../input/2023/day2.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 2162);
/// ```
#[aoc(day2, part1)]
pub fn solve_part_01(input: &Games) -> u32 {
    let mut id_sum = 0;
    let max_blue = 14;
    let max_green = 13;
    let max_red = 12;

    for (id, game) in input {
        let is_impossible = game.iter().any(|round| {
            round.iter().any(|cube| match (cube.color, cube.count) {
                (Color::Red, count) => count > max_red,
                (Color::Green, count) => count > max_green,
                (Color::Blue, count) => count > max_blue,
            })
        });

        if !is_impossible {
            id_sum += id;
        }
    }

    id_sum
}

/* Part Two
*
* For each game, find the smallest number of cubes of each color
* that can be used. Multiply these numbers together to get the "power".
* Sum up the powers of all games.
*
*/
// Your puzzle answer was
/// ```
/// use advent_of_code_2023::day_02::*;
/// let data = include_str!("../input/2023/day2.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 72513);
/// ```
#[aoc(day2, part2)]
pub fn solve_part_02(input: &Games) -> u32 {
    let mut power_sum = 0;

    for game in input.values() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for cube in game.iter().flatten() {
            match cube.color {
                Color::Red => min_red = cube.count.max(min_red),
                Color::Green => min_green = cube.count.max(min_green),
                Color::Blue => min_blue = cube.count.max(min_blue),
            }
        }

        power_sum += min_red * min_green * min_blue;
    }

    power_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_part_01(&input_generator(data)), 8)
    }

    #[test]
    fn sample_02() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_part_02(&input_generator(data)), 2286)
    }
}
