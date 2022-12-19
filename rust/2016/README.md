# Advent of Code 2016

Going back in time for more Rust.

## Advent of Code 2022 Story

Santa's sleigh uses a very high-precision clock to guide its movements, and the clock's oscillator is regulated by stars. Unfortunately, the stars have been stolen... by the Easter Bunny. To save Christmas, Santa needs you to retrieve all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                               | #1  | #1 Answer | #2  | #2 Answer |
| ----------------------------------------------------------------------------------------------------------------- | --- | --------: | --- | --------: |
| [Day 1: No Time for a Taxicab](https://github.com/believer/advent-of-code/blob/master/rust/2016/src/day_01.rs)    | 🌟  |   **252** |     |           |
| [Day 3: Squares With Three Sides](https://github.com/believer/advent-of-code/blob/master/rust/2016/src/day_03.rs) | 🌟  |   **862** |     |           |

## Performance

With the help of [cargo-aoc](https://github.com/gobanos/cargo-aoc) I get automatic benchmarking using [Criterion](https://github.com/bheisler/criterion.rs). Computer is a MacBook Pro M1 Pro, 32 GB RAM.

| Day |        #1 |  #2 | Improvement\* | Generator |
| --- | --------: | --: | ------------- | --------: |
| 1   | 267.47 ns |     |               |   3.50 µs |
| 3   |   1.56 µs |     |               | 101.57 µs |

\* compared to first solution

### Previous solutions

| Day |  #1 |  #2 | Improvement | Link |
| --: | --: | --: | ----------: | ---- |
