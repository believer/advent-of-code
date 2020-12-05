# Advent of Code 2020

This year I'm doing the solutions in Rust. I've started liking it a lot during the year and want to see what type of performance I can get.

## Advent of Code 2020 Story

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                          | #1  |  #1 Answer | #2  |      #2 Answer |
| ------------------------------------------------------------------------------------------------------------ | --- | ---------: | --- | -------------: |
| [Day 1: Report Repair](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_01.rs)       | 🌟  | **898299** | 🌟  |  **143933922** |
| [Day 2: Password Philosophy](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_02.rs) | 🌟  |    **524** | 🌟  |        **485** |
| [Day 3: Toboggan Trajectory](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_03.rs) | 🌟  |    **259** | 🌟  | **2224913600** |
| [Day 4: Passport Processing](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_04.rs) | 🌟  |    **200** | 🌟  |        **116** |
| [Day 5: Binary Boarding](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_05.rs)     | 🌟  |    **866** | 🌟  |        **583** |

## Performance

With the help of [cargo-aoc](https://github.com/gobanos/cargo-aoc) I get automatic benchmarking using [Criterion](https://github.com/bheisler/criterion.rs). Computer is a MacBook Pro 2,6 GHz 6-Core i7, 32 GB RAM, Radeon 5300M 4 GB.

| Day |        #1 |        #2 |                                                                                                                                         Improvement\* |
| --- | --------: | --------: | ----------------------------------------------------------------------------------------------------------------------------------------------------: |
| 1   |   1.21 µs |  16.70 µs | ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−92.46%` / ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−99.61%` |
| 2   |  16.62 µs |  39.57 µs | ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−96.27%` / ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `-91.57%` |
| 3   |   2.04 µs |   9.51 µs |                                                                                                                                                       |
| 4   | 528.92 ns | 731.94 ns |                                                                                                                                                       |
| 5   | 103.69 µs | 127.14 µs |                                                                                                                                                       |

\* compared to first solution

### Previous solutions

| Day | Solution # |        #1 |        #2 |                                                                      Improvement | Link                                                                                                                         |
| --- | ---------: | --------: | --------: | -------------------------------------------------------------------------------: | ---------------------------------------------------------------------------------------------------------------------------- |
| 1   |          1 |  16.06 µs |   4.38 ms |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/7e0bc4927db3a9d4f8fd0fd1a0f34feba4f6f3dc/rust-2020/day_01/src/lib.rs) |
| 2   |          1 | 446.02 µs | 470.28 µs |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/aee8289c2ffd0c20968c43e573fc5828d88a82a6/rust/2020/src/day_02.rs)     |
| 2   |          2 | 446.02 µs |  77.35 µs | `0%` / ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−83.55%` | [Link](https://github.com/believer/advent-of-code/blob/0cce6ca175c8d89e43772bc386f152bc6167edbd/rust/2020/src/day_02.rs)     |
