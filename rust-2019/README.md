# Advent of Code 2019

Trying some solutions in Rust.

## Advent of Code 2019 Story

Santa has become stranded at the edge of the Solar System while delivering presents to other planets! To accurately calculate his position in space, safely align his warp drive, and return to Earth in time to save Christmas, he needs you to bring him measurements from fifty stars.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                                              | #1  |  #2 | Performance       |
| -------------------------------------------------------------------------------------------------------------------------------- | --- | --- | ----------------- |
| [Day 1: The Tyranny of the Rocket Equation](https://github.com/believer/advent-of-code/blob/master/rust-2019/day_01/src/main.rs) | 🌟  | 🌟  | 3.92 µs / 8.06 µs |
| [Day 2: 1202 Program Alarm](https://github.com/believer/advent-of-code/blob/master/rust-2019/day_02/src/main.rs)                 | 🌟  | 🌟  | 2.13 µs / 3.24 ms |

## Performance

I've tested performance using `std::time::Instant` and taken the fastest time from a couple of runs with release build. This is not the most exact measure and times this fast can probably be wrong, but it's at least some form of measure of how fast Rust is.
