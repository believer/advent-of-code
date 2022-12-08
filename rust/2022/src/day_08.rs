// Day 8 - Treetop Tree House
//
// A lot of trial and error here, but I got the answers in the end.

type Input = Vec<Vec<u32>>;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|s| s.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_08::*;
/// let data = include_str!("../input/2022/day8.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 1684);
/// ```
#[aoc(day8, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    let outer_rows = input.get(0).unwrap().len() * 2;
    let outer_cols = (input.len() - 2) * 2;

    let mut visible = (outer_rows + outer_cols) as u32;

    let inner_rows = input[1..input.len() - 1].iter();
    let inner = inner_rows
        .clone()
        .map(|row| row[1..row.len() - 1].iter().collect())
        .collect::<Vec<Vec<_>>>();

    for (i, row) in inner.iter().enumerate().map(|(i, row)| (i + 1, row)) {
        for (j, col) in row.iter().enumerate().map(|(j, col)| (j + 1, col)) {
            let value = **col;

            let all_top = (1..=i).all(|v| {
                let top = input[v - 1][j];
                if value > top {
                    true
                } else {
                    false
                }
            });

            let all_bottom = (i..=input.len()).all(|v| {
                if (v + 1) >= input.len() {
                    return true;
                }
                let bottom = input[v + 1][j];
                if value > bottom {
                    true
                } else {
                    false
                }
            });

            let all_left = (1..=j).all(|v| {
                let left = input[i][v - 1];
                if value > left {
                    true
                } else {
                    false
                }
            });

            let all_right = (j..=row.len()).all(|v| {
                let right = input[i][v + 1];
                if value > right {
                    true
                } else {
                    false
                }
            });

            let tree_is_visible = all_top || all_bottom || all_left || all_right;

            // println!(
            //     "{} ({},{}) is visible {} from t/r/b/l ({},{},{},{})",
            //     col, i, j, tree_is_visible, all_top, all_right, all_bottom, all_left
            // );

            if tree_is_visible {
                visible += 1;
            }
        }
    }

    visible
}

/* Part Two
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_08::*;
/// let data = include_str!("../input/2022/day8.txt");
/// assert_eq!(solve_part_02(&input_generator(data)), 486540);
/// ```
#[aoc(day8, part2)]
pub fn solve_part_02(input: &Input) -> u32 {
    let mut scenic_scores: Vec<u32> = vec![];

    let inner_rows = input[1..input.len() - 1].iter();
    let inner = inner_rows
        .clone()
        .map(|row| row[1..row.len() - 1].iter().collect())
        .collect::<Vec<Vec<_>>>();

    for (i, row) in inner.iter().enumerate().map(|(i, row)| (i + 1, row)) {
        for (j, col) in row.iter().enumerate().map(|(j, col)| (j + 1, col)) {
            let value = **col;
            let mut top_score = 0;
            let mut right_score = 0;
            let mut bottom_score = 0;
            let mut left_score = 0;

            for v in (1..=i).rev() {
                let top = input[v - 1][j];

                top_score += 1;

                if top >= value {
                    break;
                }
            }

            for v in i..=input.len() {
                if (v + 1) >= input.len() {
                    break;
                }
                let bottom = input[v + 1][j];

                bottom_score += 1;

                if bottom >= value {
                    break;
                }
            }

            for v in (1..=j).rev() {
                let left = input[i][v - 1];

                left_score += 1;

                if left >= value {
                    break;
                }
            }

            for v in j..=row.len() {
                let right = input[i][v + 1];

                right_score += 1;

                if right >= value {
                    break;
                }
            }

            scenic_scores.push(top_score * right_score * bottom_score * left_score);
        }
    }

    scenic_scores.iter().max().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "30373
25512
65332
33549
35390";

        assert_eq!(solve_part_01(&input_generator(data)), 21)
    }

    #[test]
    fn sample_02() {
        let data = "30373
25512
65332
33549
35390";

        assert_eq!(solve_part_02(&input_generator(data)), 8)
    }
}
