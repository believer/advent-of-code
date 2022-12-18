use std::collections::{HashSet, VecDeque};

// Day 17 - Pyroclastic Flow
//
// The last couple of days have been quite hard. I was stuck on part one for
// a long time. It all came down to a bug that I didn't reset the piece after
// adding it back to the queue. Which meant it was a bit off in position when
// it came around the next time because I'm mutating them.
//
// The second part is a huge number, so we're probably looking for some kind
// of cycle in the rocks. I might come back and find it later. I will at least
// try to clean up my solution for part one.
//
// Updates:
//
// I cleaned up the code a bunch, but it's still a messy. The main difference
// in performance came from changing the Grid from the pathfinding crate to a
// HashSet. It was great to have Grid for debugging purposes as I could easily see
// the current state. But, using a HashSet is > 40% faster.

type Input = VecDeque<Jet>;

#[derive(Debug, Clone, Copy)]
pub enum Jet {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Piece {
    parts: Vec<(usize, usize)>,
    piece_type: PieceType,
}

#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    Line,
    Plus,
    LShape,
    IShape,
    Square,
}

impl Piece {
    fn new(piece_type: PieceType) -> Self {
        let initial_parts = match piece_type {
            PieceType::Line => vec![(2, 0), (3, 0), (4, 0), (5, 0)],
            PieceType::Plus => vec![(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)],
            PieceType::LShape => vec![(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)],
            PieceType::IShape => vec![(2, 0), (2, 1), (2, 2), (2, 3)],
            PieceType::Square => vec![(2, 0), (3, 0), (2, 1), (3, 1)],
        };

        Piece {
            parts: initial_parts,
            piece_type,
        }
    }

    fn move_horizontal(&mut self, direction: Jet, grid: &HashSet<(usize, usize)>) {
        let mut new_parts = Vec::new();
        let max_x = self.parts.iter().map(|(x, _)| x).max().unwrap();
        let min_x = self.parts.iter().map(|(x, _)| x).min().unwrap();
        let has_left = self.parts.iter().any(|(x, y)| {
            let next_x = if *x == 0 { 0 } else { x - 1 };

            grid.contains(&(next_x, *y))
        });
        let has_right = self.parts.iter().any(|(x, y)| grid.contains(&(*x + 1, *y)));

        for (x, y) in &self.parts {
            let next_place = match direction {
                Jet::Left => {
                    if *min_x == 0 || has_left {
                        (*x, *y)
                    } else {
                        (*x - 1, *y)
                    }
                }
                Jet::Right => {
                    if *max_x == 6 || has_right {
                        (*x, *y)
                    } else {
                        (*x + 1, *y)
                    }
                }
            };

            new_parts.push(next_place);
        }

        self.parts = new_parts;
    }

    fn move_down(&mut self, current_stack: &HashSet<(usize, usize)>) -> Option<()> {
        let mut new_parts = Vec::new();

        for (x, y) in &self.parts {
            new_parts.push((*x, *y - 1));
        }

        let hit_rock = self
            .parts
            .iter()
            .any(|(x, y)| current_stack.contains(&(*x, *y - 1)));
        let lowest_y = new_parts.iter().map(|(_, y)| y).min().unwrap();

        // Check if piece is on the ground or if it hit a rock
        if hit_rock || *lowest_y == 0 {
            None
        } else {
            self.parts = new_parts;
            Some(())
        }
    }

    fn adjust_for_stack(&mut self, stack_height: usize) {
        let mut new_parts = Vec::new();

        for (x, y) in &self.parts {
            new_parts.push((*x, *y + stack_height));
        }

        self.parts = new_parts;
    }

    fn reset(&mut self) -> Self {
        Piece::new(self.piece_type)
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Input {
    input
        .trim()
        .split("")
        .filter(|l| !l.is_empty())
        .map(|s| match s {
            ">" => Jet::Right,
            "<" => Jet::Left,
            b => panic!("Unknown jet: {b:?}"),
        })
        .collect()
}

/* Part One
*/
/// Your puzzle answer was
/// ```
/// use advent_of_code_2022::day_17::*;
/// let data = include_str!("../input/2022/day17.txt");
/// assert_eq!(solve_part_01(&input_generator(data)), 3109);
/// ```
#[aoc(day17, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    let mut jets = input.clone();
    let mut pieces = VecDeque::new();
    let mut stack_height = 0;
    let mut rocks: usize = 0;
    // let mut grid = Grid::new(7, 8);
    let mut grid = HashSet::new();

    pieces.push_back(Piece::new(PieceType::Line));
    pieces.push_back(Piece::new(PieceType::Plus));
    pieces.push_back(Piece::new(PieceType::LShape));
    pieces.push_back(Piece::new(PieceType::IShape));
    pieces.push_back(Piece::new(PieceType::Square));

    let mut current_piece = pieces.pop_front().unwrap();
    current_piece.adjust_for_stack(4);

    for round in 0.. {
        if rocks == 2022 {
            break;
        }

        // Jet pushes
        if round % 2 == 0 {
            let jet = jets.pop_front().unwrap();

            current_piece.move_horizontal(jet, &grid);

            // Move jet back to the end of the queue
            jets.push_back(jet);

        // Piece moves down
        } else if current_piece.move_down(&grid).is_none() {
            current_piece.parts.iter().for_each(|(x, y)| {
                grid.insert((*x, *y));
            });

            // Update stack height
            stack_height = grid.iter().map(|(_, y)| *y).max().unwrap();

            let recreate_piece = current_piece.reset();

            // Put the current piece at the back of the queue
            pieces.push_back(recreate_piece);

            // Get a new piece
            current_piece = pieces.pop_front().unwrap();
            current_piece.adjust_for_stack(stack_height + 4);

            // Increment number of rocks that have fallen
            rocks += 1;
        }
    }

    stack_height
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(SAMPLE)), 3068)
    }
}
