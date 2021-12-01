# Advent of Code 2021

Continuing with Rust this year since it was fun last year.

## Advent of Code 2021 Story

You're minding your own business on a ship at sea when the overboard alarm goes off! You rush to see if you can help. Apparently, one of the Elves tripped and accidentally sent the sleigh keys flying into the ocean!

Before you know it, you're inside a submarine the Elves keep ready for situations like this. It's covered in Christmas lights (because of course it is), and it even has an experimental antenna that should be able to track the keys if you can boost its signal strength high enough; there's a little meter that indicates the antenna's signal strength by displaying 0-50 stars.

Your instincts tell you that in order to save Christmas, you'll need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                  | #1  | #1 Answer | #2  | #2 Answer |
| ---------------------------------------------------------------------------------------------------- | --- | --------: | --- | --------: |
| [Day 1: Sonar Sweep](https://github.com/believer/advent-of-code/blob/master/rust/2021/src/day_01.rs) | 🌟  |  **1713** | 🌟  |  **1734** |

## Performance

With the help of [cargo-aoc](https://github.com/gobanos/cargo-aoc) I get automatic benchmarking using [Criterion](https://github.com/bheisler/criterion.rs). Computer is a MacBook Pro 2,6 GHz 6-Core i7, 32 GB RAM, Radeon 5300M 4 GB.

| Day |        #1 |        #2 | Improvement\* |
| --- | --------: | --------: | ------------- |
| 1   | 272.11 ns | 923.48 ns |               |

\* compared to first solution
