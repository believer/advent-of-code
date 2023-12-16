//! Day 16

use std::collections::{HashSet, VecDeque};

use crate::{
    grid::Grid,
    point::{Point, DOWN, LEFT, ORIGIN, RIGHT, UP},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Beam {
    head: Point,
    direction: Point,
}

impl Beam {
    fn new(head: Point, direction: Point) -> Self {
        Self { head, direction }
    }

    fn move_forward(&mut self) {
        self.head += self.direction;
    }
}

pub struct Input {
    tiles: Grid<u8>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    Input {
        tiles: Grid::from(input),
    }
}

/* Part One
*
*
*/
// Your puzzle answer was
#[doc = r#"```
use advent_of_code_2023::day_16::*;
let data = include_str!("../input/2023/day16.txt");
assert_eq!(solve_part_01(&input_generator(data)), 7562);
```"#]
#[aoc(day16, part1)]
pub fn solve_part_01(input: &Input) -> usize {
    let mut energized_tiles = input.tiles.clone();
    let mut beams = VecDeque::new();
    let mut seen: HashSet<Beam> = HashSet::new();

    beams.push_back(Beam::new(ORIGIN, RIGHT));

    while let Some(mut beam) = beams.pop_front() {
        if !input.tiles.contains(beam.head) || !seen.insert(beam) {
            continue;
        }

        energized_tiles[beam.head] = b'#';

        match input.tiles[beam.head] {
            b'.' => {
                beam.move_forward();
                beams.push_back(beam);
            }
            b'|' => match beam.direction {
                UP | DOWN => {
                    beam.move_forward();
                    beams.push_back(beam);
                }
                LEFT | RIGHT => {
                    beams.push_back(Beam {
                        head: beam.head + UP,
                        direction: UP,
                    });
                    beams.push_back(Beam {
                        head: beam.head + DOWN,
                        direction: DOWN,
                    });
                }
                _ => (),
            },
            b'-' => match beam.direction {
                LEFT | RIGHT => {
                    beam.move_forward();
                    beams.push_back(beam);
                }
                UP | DOWN => {
                    beams.push_back(Beam {
                        head: beam.head + LEFT,
                        direction: LEFT,
                    });
                    beams.push_back(Beam {
                        head: beam.head + RIGHT,
                        direction: RIGHT,
                    });
                }
                _ => (),
            },
            b'/' => match beam.direction {
                UP => {
                    beams.push_back(Beam {
                        head: beam.head + RIGHT,
                        direction: RIGHT,
                    });
                }
                RIGHT => {
                    beams.push_back(Beam {
                        head: beam.head + UP,
                        direction: UP,
                    });
                }
                DOWN => {
                    beams.push_back(Beam {
                        head: beam.head + LEFT,
                        direction: LEFT,
                    });
                }
                LEFT => {
                    beams.push_back(Beam {
                        head: beam.head + DOWN,
                        direction: DOWN,
                    });
                }
                _ => (),
            },
            b'\\' => match beam.direction {
                UP => {
                    beams.push_back(Beam {
                        head: beam.head + LEFT,
                        direction: LEFT,
                    });
                }
                RIGHT => {
                    beams.push_back(Beam {
                        head: beam.head + DOWN,
                        direction: DOWN,
                    });
                }
                DOWN => {
                    beams.push_back(Beam {
                        head: beam.head + RIGHT,
                        direction: RIGHT,
                    });
                }
                LEFT => {
                    beams.push_back(Beam {
                        head: beam.head + UP,
                        direction: UP,
                    });
                }
                _ => (),
            },
            _ => (),
        }
    }

    energized_tiles.find_all(b'#').len()
}

/* Part Two
*
*
*/
#[doc = r#"```
use advent_of_code_2023::day_16::*;
let data = include_str!("../input/2023/day16.txt");
assert_eq!(solve_part_02(&input_generator(data)), 7793);
```"#]
#[aoc(day16, part2)]
pub fn solve_part_02(input: &Input) -> usize {
    let mut most_energy = 0;

    let mut possible_starts = vec![];

    for x in 0..input.tiles.width {
        possible_starts.push(Beam::new(Point::new(x, 0), DOWN));
        possible_starts.push(Beam::new(Point::new(x, input.tiles.height - 1), UP));
    }

    for y in 0..input.tiles.height {
        possible_starts.push(Beam::new(Point::new(0, y), RIGHT));
        possible_starts.push(Beam::new(Point::new(input.tiles.width - 1, y), LEFT));
    }

    for start in possible_starts {
        let mut energized_tiles = input.tiles.clone();
        let mut beams = VecDeque::new();
        let mut seen: HashSet<Beam> = HashSet::new();

        beams.push_back(start);

        while let Some(mut beam) = beams.pop_front() {
            if !input.tiles.contains(beam.head) || !seen.insert(beam) {
                continue;
            }

            energized_tiles[beam.head] = b'#';

            match input.tiles[beam.head] {
                b'.' => {
                    beam.move_forward();
                    beams.push_back(beam);
                }
                b'|' => match beam.direction {
                    UP | DOWN => {
                        beam.move_forward();
                        beams.push_back(beam);
                    }
                    LEFT | RIGHT => {
                        beams.push_back(Beam {
                            head: beam.head + UP,
                            direction: UP,
                        });
                        beams.push_back(Beam {
                            head: beam.head + DOWN,
                            direction: DOWN,
                        });
                    }
                    _ => (),
                },
                b'-' => match beam.direction {
                    LEFT | RIGHT => {
                        beam.move_forward();
                        beams.push_back(beam);
                    }
                    UP | DOWN => {
                        beams.push_back(Beam {
                            head: beam.head + LEFT,
                            direction: LEFT,
                        });
                        beams.push_back(Beam {
                            head: beam.head + RIGHT,
                            direction: RIGHT,
                        });
                    }
                    _ => (),
                },
                b'/' => match beam.direction {
                    UP => {
                        beams.push_back(Beam {
                            head: beam.head + RIGHT,
                            direction: RIGHT,
                        });
                    }
                    RIGHT => {
                        beams.push_back(Beam {
                            head: beam.head + UP,
                            direction: UP,
                        });
                    }
                    DOWN => {
                        beams.push_back(Beam {
                            head: beam.head + LEFT,
                            direction: LEFT,
                        });
                    }
                    LEFT => {
                        beams.push_back(Beam {
                            head: beam.head + DOWN,
                            direction: DOWN,
                        });
                    }
                    _ => (),
                },
                b'\\' => match beam.direction {
                    UP => {
                        beams.push_back(Beam {
                            head: beam.head + LEFT,
                            direction: LEFT,
                        });
                    }
                    RIGHT => {
                        beams.push_back(Beam {
                            head: beam.head + DOWN,
                            direction: DOWN,
                        });
                    }
                    DOWN => {
                        beams.push_back(Beam {
                            head: beam.head + RIGHT,
                            direction: RIGHT,
                        });
                    }
                    LEFT => {
                        beams.push_back(Beam {
                            head: beam.head + UP,
                            direction: UP,
                        });
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        let energy = energized_tiles.find_all(b'#').len();

        if energy > most_energy {
            most_energy = energy;
        }
    }

    most_energy
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....",
        46
    )]
    fn sample_01(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_01(&input_generator(input)), expected);
    }

    #[rstest]
    #[case(
        ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....",
        51
    )]
    fn sample_02(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part_02(&input_generator(input)), expected);
    }
}
