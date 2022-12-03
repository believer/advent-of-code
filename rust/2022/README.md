# Advent of Code 2022

Continuing with Rust this year.

## Advent of Code 2022 Story

Santa's reindeer typically eat regular reindeer food, but they need a lot of [magical energy](https://adventofcode.com/2018/day/25) to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                              | #1  | #1 Answer | #2  |  #2 Answer |
| ---------------------------------------------------------------------------------------------------------------- | --- | --------: | --- | ---------: |
| [Day 1: Calorie Counting](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_01.rs)        | 🌟  | **69528** | 🌟  | **206152** |
| [Day 2: Rock Paper Scissors](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_02.rs)     | 🌟  | **13809** | 🌟  |  **12316** |
| [Day 3: Rucksack Reorganization](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_03.rs) | 🌟  |  **7831** | 🌟  |   **2683** |

## Performance

With the help of [cargo-aoc](https://github.com/gobanos/cargo-aoc) I get automatic benchmarking using [Criterion](https://github.com/bheisler/criterion.rs). Computer is a MacBook Pro M1 Pro, 32 GB RAM.

| Day |        #1 |        #2 | Improvement\*         |
| --- | --------: | --------: | --------------------- |
| 1   |   1.56 ns |   2.18 ns | `-99.82%` / `-99.96%` |
| 2   |   8.08 µs |   7.71 µs |                       |
| 3   | 321.94 µs | 325.65 µs | `+70.24%` / `0%`      |

\* compared to first solution

### Previous solutions

| Day |        #1 |      #2 |      Improvement | Link                                                                                                                     |
| --: | --------: | ------: | ---------------: | ------------------------------------------------------------------------------------------------------------------------ |
|   1 | 865.43 ns | 5.51 µs |         Baseline | [Link](https://github.com/believer/advent-of-code/blob/5e1dbfdf07be5916d8d323360cf1f86767009ca2/rust/2022/src/day_01.rs) |
|   1 |       --- | 3.47 µs | `0%` / `-37.84%` | [Link](https://github.com/believer/advent-of-code/blob/1cf6a750e0e899c25e9cffbc433cc46087d5a3e8/rust/2022/src/day_01.rs) |
|   2 | 189.11 µs |     --- |                  | [Link](https://github.com/believer/advent-of-code/blob/240d950499b11b8b3d077cc6b1c4b00b9c442235/rust/2022/src/day_03.rs) |
