// Day 8 - Treetop Tree House
//
// A lot of trial and error here, but I got the answers in the end.

type Input = Vec<Vec<u32>>;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|s| s.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_inner_trees(input: &Input) -> Vec<Vec<&u32>> {
    input[1..input.len() - 1]
        .iter()
        .map(|row| row[1..row.len() - 1].iter().collect())
        .collect::<Vec<Vec<_>>>()
}

/* Part One
 *
 * In this part, we need to find the number of trees that are visible.
 * A visible tree is defined as a tree that is not blocked by a taller tree in
 * at least on direction.
 *
 * The edges of the forest are always visible, so we can count them first.
*/
#[aoc(day8, part1)]
pub fn solve_part_01(input: &Input) -> u32 {
    // We know that the outer most ring of tree are always visible.
    // That's the length of the first and last row plus
    // the length of the first and last column (without the first
    // and the last row).
    let outer_rows = input.first().unwrap().len() * 2;
    let outer_cols = (input.len() - 2) * 2;

    let mut visible = (outer_rows + outer_cols) as u32;

    // Here we create a new vector that has removed the outermost
    // ring of trees as they have already been counted.
    for (i, row) in find_inner_trees(input)
        .iter()
        .enumerate()
        .map(|(i, row)| (i + 1, row))
    {
        for (j, col) in row.iter().enumerate().map(|(j, col)| (j + 1, col)) {
            let tree_size = **col;

            // Find if there are taller tree in all directions
            // If _one_ direction only has smaller trees to the edge
            // the tree is visible.
            let all_bottom = (i..input.len() - 1).all(|v| tree_size > input[v + 1][j]);
            let all_top = (1..=i).all(|v| tree_size > input[v - 1][j]);
            let all_left = (1..=j).all(|v| tree_size > input[i][v - 1]);
            let all_right = (j..=row.len()).all(|v| tree_size > input[i][v + 1]);

            if all_top || all_bottom || all_left || all_right {
                visible += 1;
            }
        }
    }

    visible
}

/* Part Two
 *
 * In this part, we need to find the tree that has the largest scenic score.
 * The scenic score is defined as the number of trees that are visible from
 * the tree. That's counting _from_ the tree outwards until we hit a tree of
 * the same size or taller. The taller tree is counted, the tree itself is not counted.
 *
 * For example:
 *
 * - We see 3 upwards
 * - We see 2 downwards
 * - We see 1 left
 * - We see 2 right
 *
 * The scenic score is 3 * 2 * 1 * 2 = 12
*/
#[aoc(day8, part2)]
pub fn solve_part_02(input: &Input) -> u32 {
    let mut scenic_scores: Vec<u32> = vec![];

    // Again we only need to check the inner trees
    // This tripped me up for a while since I still had the ranges from
    // before. Some of them needed to be reversed to look outwards.
    for (i, row) in find_inner_trees(input)
        .iter()
        .enumerate()
        .map(|(i, row)| (i + 1, row))
    {
        for (j, col) in row.iter().enumerate().map(|(j, col)| (j + 1, col)) {
            let tree_size = **col;
            let mut top_score = 0;
            let mut right_score = 0;
            let mut bottom_score = 0;
            let mut left_score = 0;

            // Look upwards
            for v in (1..=i).rev() {
                top_score += 1;

                if input[v - 1][j] >= tree_size {
                    break;
                }
            }

            // Look downwards
            for v in i..=input.len() {
                if (v + 1) >= input.len() {
                    break;
                }

                bottom_score += 1;

                if input[v + 1][j] >= tree_size {
                    break;
                }
            }

            // Look left
            for v in (1..=j).rev() {
                left_score += 1;

                if input[i][v - 1] >= tree_size {
                    break;
                }
            }

            // Look right
            for v in j..=row.len() {
                right_score += 1;

                if input[i][v + 1] >= tree_size {
                    break;
                }
            }

            // Add the score to the list
            scenic_scores.push(top_score * right_score * bottom_score * left_score);
        }
    }

    // Find the largest scenic score
    *scenic_scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 21)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(SAMPLE)), 8)
    }
}
