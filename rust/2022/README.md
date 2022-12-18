# Advent of Code 2022

Continuing with Rust this year.

## Advent of Code 2022 Story

Santa's reindeer typically eat regular reindeer food, but they need a lot of [magical energy](https://adventofcode.com/2018/day/25) to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                               | #1  |     #1 Answer | #2  |          #2 Answer |
| ----------------------------------------------------------------------------------------------------------------- | --- | ------------: | --- | -----------------: |
| [Day 1: Calorie Counting](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_01.rs)         | 🌟  |     **69528** | 🌟  |         **206152** |
| [Day 2: Rock Paper Scissors](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_02.rs)      | 🌟  |     **13809** | 🌟  |          **12316** |
| [Day 3: Rucksack Reorganization](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_03.rs)  | 🌟  |      **7831** | 🌟  |           **2683** |
| [Day 4: Camp Cleanup](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_04.rs)             | 🌟  |       **550** | 🌟  |            **931** |
| [Day 5: Supply Stacks](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_05.rs)            | 🌟  | **PSNRGBTFT** | 🌟  |      **BNTZFPMMW** |
| [Day 6: Tuning Trouble](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_06.rs)           | 🌟  |      **1300** | 🌟  |           **3986** |
| [Day 7: No Space Left On Device](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_07.rs)  | 🌟  |   **1444896** | 🌟  |         **404395** |
| [Day 8: Treetop Tree House](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_08.rs)       | 🌟  |      **1684** | 🌟  |         **486540** |
| [Day 9: Rope Bridge](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_09.rs)              | 🌟  |      **5735** | 🌟  |           **2478** |
| [Day 10: Cathode-Ray Tube](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_10.rs)        | 🌟  |     **16880** | 🌟  |       **RKAZAJBR** |
| [Day 11: Monkey in the Middle](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_11.rs)    | 🌟  |     **66124** | 🌟  |    **19309892877** |
| [Day 12: Hill Climbing Algorithm](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_12.rs) | 🌟  |       **420** | 🌟  |            **414** |
| [Day 13: Distress Signal](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_13.rs)         | 🌟  |      **5390** | 🌟  |          **19261** |
| [Day 14: Regolith Reservoir](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_14.rs)      | 🌟  |       **737** | 🌟  |          **28145** |
| [Day 15: Beacon Exclusion Zone](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_15.rs)   | 🌟  |   **5083287** | 🌟  | **13134039205729** |
| Day 16: Proboscidea Volcanium                                                                                     |     |               |     |                    |
| [Day 17: Pyroclastic Flow](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_17.rs)        | 🌟  |      **3109** |     |                    |
| [Day 18: Boiling Boulders](https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_18.rs)        | 🌟  |      **4244** | 🌟  |           **2460** |

## Performance

With the help of [cargo-aoc](https://github.com/gobanos/cargo-aoc) I get automatic benchmarking using [Criterion](https://github.com/bheisler/criterion.rs). Computer is a MacBook Pro M1 Pro, 32 GB RAM.

| Day |        #1 |        #2 | Improvement\*         | Generator |
| --- | --------: | --------: | --------------------- | --------: |
| 1   |   1.56 ns |   2.18 ns | `-99.82%` / `-99.96%` |  49.82 µs |
| 2   |   8.08 µs |   7.71 µs |                       | 183.30 µs |
| 3   | 321.94 µs | 325.65 µs | `+70.24%` / `0%`      |  12.39 µs |
| 4   |   1.63 µs |   1.00 µs |                       | 137.52 µs |
| 5   |  20.21 µs |  34.96 µs | `-91.86%` / `-73.69%` |  65.86 µs |
| 6   |  78.93 µs | 844.10 µs |                       |   2.61 µs |
| 7   | 415.02 ns |   3.88 µs |                       |   7.27 ms |
| 8   | 293.44 µs | 275.96 µs |                       | 192.56 µs |
| 9   | 451.31 µs | 667.39 µs | `-2.51%` / `-23.81%`  |  57.74 µs |
| 10  | 230.74 ns | 806.73 ns | `-73.19%` / `-45.25%` |   2.28 µs |
| 11  |   8.18 µs |   2.79 ms | `-83.77%` / `-89.64%` |  59.21 ns |
| 12  |   1.94 ms | 457.26 ms |                       |  18.22 µs |
| 13  |   2.38 µs |  68.69 µs | `0%` / `72.92%`       | 286.29 µs |
| 14  |   6.94 ms | 384.08 ms |                       | 336.12 µs |
| 15  |  91.81 ms | 137.01 ms |                       |   2.58 µs |
| 17  |  16.94 ms |           | `-42.31%`             |  84.22 µs |
| 18  | 428.92 µs |   3.30 ms | `-97.58%` / `-95.04%` | 165.10 µs |

\* compared to first solution

### Previous solutions

|                                                                                                                      Day |        #1 |        #2 |      Improvement | Link                                                                                                                     |
| -----------------------------------------------------------------------------------------------------------------------: | --------: | --------: | ---------------: | ------------------------------------------------------------------------------------------------------------------------ |
|                                                                                                                        1 | 865.43 ns |   5.51 µs |         Baseline | [Link](https://github.com/believer/advent-of-code/blob/5e1dbfdf07be5916d8d323360cf1f86767009ca2/rust/2022/src/day_01.rs) |
|                                                                                                                        1 |       --- |   3.47 µs | `0%` / `-37.84%` | [Link](https://github.com/believer/advent-of-code/blob/1cf6a750e0e899c25e9cffbc433cc46087d5a3e8/rust/2022/src/day_01.rs) |
|                                                                                                                        2 | 189.11 µs |       --- |                  | [Link](https://github.com/believer/advent-of-code/blob/240d950499b11b8b3d077cc6b1c4b00b9c442235/rust/2022/src/day_03.rs) |
|                                                                                                                        5 | 248.21 µs | 132.88 µs |                  | [Link](https://github.com/believer/advent-of-code/blob/3ad0e790e383a7f558acae64faa5cb5ef73eef0f/rust/2022/src/day_05.rs) |
|                                                                                                                        9 | 467.96 µs | 878.56 µs |                  | [Link](https://github.com/believer/advent-of-code/blob/3c95b9b1bee426c330930a1860f56c29cfd42e52/rust/2022/src/day_09.rs) |
|                                                                                                                       10 | 861.13 ns |   1.47 µs |                  | [Link](https://github.com/believer/advent-of-code/blob/3a3981c320e8695f1161265cba21e2fd49ccb758/rust/2022/src/day_10.rs) |
|                                                                                                                       11 |  50.04 µs |  26.88 ms |                  | [Link](https://github.com/believer/advent-of-code/blob/01b00dd5e64cf5eca405743d87c4b28dbd756f63/rust/2022/src/day_11.rs) |
|                                                                                                                       13 |   2.38 µs | 254.72 µs |                  | [Link](https://github.com/believer/advent-of-code/blob/69491b9867e79634843803c3b60fc45e836b59c2/rust/2022/src/day_13.rs) |
|                                                                                                                       17 |  29.37 ms |           |                  |
| [Link](https://github.com/believer/advent-of-code/blob/fd3d8c628a8291b93835d728fff36ea0961d7fdc/rust/2022/src/day_17.rs) |
|                                                                                                                       18 |  14.25 ms |  66.66 ms |                  |
| [Link](https://github.com/believer/advent-of-code/blob/a6c7954883364be341eb71c558883f185080ca63/rust/2022/src/day_18.rs) |
