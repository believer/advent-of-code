# Advent of Code 2023

Rust is still fun, but I might also do some Go this year. Will try to pair with ChatGPT to expand my knowledge on the algorithms that can be used.

## Advent of Code 2023 Story

Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

## Days

| Day                                                                                                     | #1  | #1 Answer | #2  | #2 Answer |
| ------------------------------------------------------------------------------------------------------- | --- | --------: | --- | --------: |
| [Day 1: Trebuchet?!](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_01.rs)    | 🌟  |     54304 | 🌟  |     54418 |
| [Day 2: Cube Conundrum](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_02.rs) | 🌟  |      2162 | 🌟  |     72513 |
| [Day 3: Gear Ratios](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_03.rs)    | 🌟  |    535235 | 🌟  |  79844424 |
| [Day 4: Scratchcards](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_04.rs)   | 🌟  |     27454 | 🌟  |   6857330 |

## Performance

With the help of [cargo-aoc](https://github.com/gobanos/cargo-aoc) I get automatic benchmarking using [Criterion](https://github.com/bheisler/criterion.rs). Computer is a MacBook Pro M1 Pro, 32 GB RAM.

| Day |        #1 |       #2 | Improvement\*         | Generator |
| --- | --------: | -------: | --------------------- | --------: |
| 1   |  43.18 ns |  1.19 ms |                       | 441.01 ns |
| 2   | 764.68 ns |  1.75 µs |                       |  47.81 µs |
| 3   | 159.61 µs |  7.72 ms | `-8.44%`              |  86.83 µs |
| 4   |  12.85 µs | 13.09 µs | `-48.41%` / `-46.64%` | 230.06 µs |

\* compared to first solution

### Previous solutions

| Day |        #1 |       #2 | Improvement | Link      |
| --: | --------: | -------: | ----------: | --------- |
|   3 | 172.23 µs |        - |             |           |
|   4 |  24.33 µs | 24.32 µs |             | 128.62 µs |
