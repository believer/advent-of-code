# Advent of Code 2015

## Advent of Code 2015 Story

Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh out! To save Christmas, he needs you to collect fifty stars by December 25th.

Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                                             | #1  |   #1 Answer | #2  |   #2 Answer |
| ------------------------------------------------------------------------------------------------------------------------------- | --- | ----------: | --- | ----------: |
| [Day 1: Not Quite Lisp](https://github.com/believer/advent-of-code/blob/master/rust/2015/src/day_01.rs)                         | 🌟  |     **232** | 🌟  |    **1783** |
| [Day 2: I Was Told There Would Be No Math](https://github.com/believer/advent-of-code/blob/master/rust/2015/src/day_02.rs)      | 🌟  | **1586300** | 🌟  | **3737498** |
| [Day 3: Perfectly Spherical Houses in a Vacuum](https://github.com/believer/advent-of-code/blob/master/rust/2015/src/day_02.rs) | 🌟  |    **2572** | 🌟  |    **2631** |

## Performance

With the help of [cargo-aoc](https://github.com/gobanos/cargo-aoc) I get automatic benchmarking using [Criterion](https://github.com/bheisler/criterion.rs). Computer is a MacBook Pro M1 Pro, 32 GB RAM.

| Day |        #1 |        #2 | Improvement\*         | Generator |
| --- | --------: | --------: | --------------------- | --------: |
| 1   |   1.39 µs |   1.13 µs |                       |  58.67 µs |
| 2   |   3.89 µs |   6.91 µs |                       |  84.07 µs |
| 3   | 168.90 µs | 191.48 µs | `-52.55%` / `-50.02%` |  80.94 µs |

\* compared to first solution

### Previous solutions

| Day |        #1 |        #2 | Improvement | Link                                                                                                                     |
| --: | --------: | --------: | ----------: | ------------------------------------------------------------------------------------------------------------------------ |
|   3 | 355.96 µs | 383.12 µs |             | [Link](https://github.com/believer/advent-of-code/blob/1ba041a793750d9db089e224c16c800f2525e60b/rust/2015/src/day_03.rs) |
