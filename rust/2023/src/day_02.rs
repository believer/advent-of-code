use std::collections::HashMap;

// Day 2: Cube Conundrum

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

type Game = Vec<Vec<Cube>>;
type Games = HashMap<u32, Game>;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Games {
    let mut games = HashMap::new();

    for line in input.lines() {
        let (id, game) = line.split_once(':').unwrap();

        let id = id.replace("Game ", "").parse::<u32>().unwrap();
        let game = game.split(';').collect::<Vec<&str>>();

        let d = game
            .iter()
            .map(|x| {
                let x = x.split(", ").collect::<Vec<&str>>();

                let mut cubes = vec![];

                for cube in x {
                    let cube = cube.trim();
                    let (count, color) = cube.split_once(' ').unwrap();
                    let color = color.parse::<Color>().unwrap();
                    let count = count.parse::<u32>().unwrap();

                    cubes.push(Cube { color, count });
                }

                cubes
            })
            .collect::<Vec<Vec<Cube>>>();

        games.insert(id, d);
    }

    games
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

        for round in game {
            for cube in round {
                match cube.color {
                    Color::Red => {
                        if cube.count > min_red {
                            min_red = cube.count
                        }
                    }
                    Color::Green => {
                        if cube.count > min_green {
                            min_green = cube.count
                        }
                    }
                    Color::Blue => {
                        if cube.count > min_blue {
                            min_blue = cube.count
                        }
                    }
                }
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
