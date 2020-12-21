# Advent of Code 2020

This year I'm doing the solutions in Rust. I've started liking it a lot during the year and want to see what type of performance I can get.

## Advent of Code 2020 Story

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                               | #1  |          #1 Answer | #2  |                                   #2 Answer |
| ----------------------------------------------------------------------------------------------------------------- | --- | -----------------: | --- | ------------------------------------------: |
| [Day 1: Report Repair](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_01.rs)            | 🌟  |         **898299** | 🌟  |                               **143933922** |
| [Day 2: Password Philosophy](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_02.rs)      | 🌟  |            **524** | 🌟  |                                     **485** |
| [Day 3: Toboggan Trajectory](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_03.rs)      | 🌟  |            **259** | 🌟  |                              **2224913600** |
| [Day 4: Passport Processing](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_04.rs)      | 🌟  |            **200** | 🌟  |                                     **116** |
| [Day 5: Binary Boarding](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_05.rs)          | 🌟  |            **866** | 🌟  |                                     **583** |
| [Day 6: Custom Customs](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_06.rs)           | 🌟  |           **6778** | 🌟  |                                    **3406** |
| [Day 7: Handy Haversacks](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_07.rs)         | 🌟  |            **226** | 🌟  |                                    **9569** |
| [Day 8: Handheld Halting](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_08.rs)         | 🌟  |           **1489** | 🌟  |                                    **1539** |
| [Day 9: Encoding Error](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_09.rs)           | 🌟  |       **21806024** | 🌟  |                                 **2986195** |
| [Day 10: Adapter Array](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_10.rs)           | 🌟  |           **2475** | 🌟  |                         **442136281481216** |
| [Day 11: Seating System](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_11.rs)          | 🌟  |           **2183** | 🌟  |                                    **1990** |
| [Day 12: Rain Risk](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_12.rs)               | 🌟  |           **1294** | 🌟  |                                   **20592** |
| [Day 13: Shuttle Search](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_13.rs)          | 🌟  |           **3246** | 🌟  |                        **1010182346291467** |
| [Day 14: Docking Data](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_14.rs)            | 🌟  |  **5055782549997** |     |                                             |
| [Day 15: Rambunctious Recitation](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_15.rs) | 🌟  |           **1696** | 🌟  |                                   **37385** |
| [Day 16: Ticket Translation](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_16.rs)      | 🌟  |          **26980** |     |                                             |
| [Day 17: Conway Cubes](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_17.rs)            | 🌟  |            **242** | 🌟  |                                    **2292** |
| [Day 18: Operation Order](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_18.rs)         | 🌟  | **29839238838303** | 🌟  |                         **201376568795521** |
| [Day 19: Monster Messages](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_19.rs)        | 🌟  |            **122** | 🌟  |                                     **287** |
| [Day 21: Allergen Assessment](https://github.com/believer/advent-of-code/blob/master/rust/2020/src/day_21.rs)     | 🌟  |           **2517** | 🌟  | **rhvbn,mmcpg,kjf,fvk,lbmt,jgtb,hcbdb,zrb** |

## Performance

With the help of [cargo-aoc](https://github.com/gobanos/cargo-aoc) I get automatic benchmarking using [Criterion](https://github.com/bheisler/criterion.rs). Computer is a MacBook Pro 2,6 GHz 6-Core i7, 32 GB RAM, Radeon 5300M 4 GB.

| Day |        #1 |            #2 | Improvement\*                                                                                                                                         |
| --- | --------: | ------------: | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   |   1.21 µs |      16.70 µs | ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−92.46%` / ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−99.61%` |
| 2   |  16.62 µs |      39.57 µs | ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−96.27%` / ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `-91.57%` |
| 3   |   2.04 µs |       9.51 µs |                                                                                                                                                       |
| 4   | 528.92 ns |     731.94 ns |                                                                                                                                                       |
| 5   | 103.69 µs |     127.14 µs |                                                                                                                                                       |
| 6   | 477.60 µs |     696.92 µs | ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−23.83%` / `0%`                                                                      |
| 7   | 179.43 µs |       1.70 µs |                                                                                                                                                       |
| 8   |  23.66 µs |     49.398 ms |                                                                                                                                                       |
| 9   | 299.88 µs |     363.98 µs | ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−97.78%` / `−97.33%`                                                                 |
| 10  | 646.78 ns |     455.67 µs | ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−75.95%` / `−30.25%`                                                                 |
| 11  |  10.38 ms |      15.29 ms |                                                                                                                                                       |
| 12  |   9.02 µs |      12.72 µs | ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−28.58%` / `0%`                                                                      |
| 13  | 479.50 ns |       1.82 µs | ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `-99.99%` / `0%`                                                                      |
| 14  | 730.08 µs |               |                                                                                                                                                       |
| 15  | 134.16 µs |    2.92 s\*\* | ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−46.89%` / `−72.30%`                                                                 |
| 16  | 381.21 µs |               |                                                                                                                                                       |
| 17  |   8.52 ms | 704.03 ms\*\* |                                                                                                                                                       |
| 18  |  41.72 µs |     779.27 µs |                                                                                                                                                       |
| 19  |  11.42 ms |      74.00 ms |                                                                                                                                                       |
| 21  | 163.07 µs |     901.73 ns |                                                                                                                                                       |

\* compared to first solution

\*\* slow, didn't run through criterion

### Previous solutions

| Day |        #1 |        #2 |                                                                      Improvement | Link                                                                                                                         |
| --: | --------: | --------: | -------------------------------------------------------------------------------: | ---------------------------------------------------------------------------------------------------------------------------- |
|   1 |  16.06 µs |   4.38 ms |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/7e0bc4927db3a9d4f8fd0fd1a0f34feba4f6f3dc/rust-2020/day_01/src/lib.rs) |
|   2 | 446.02 µs | 470.28 µs |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/aee8289c2ffd0c20968c43e573fc5828d88a82a6/rust/2020/src/day_02.rs)     |
|   2 | 446.02 µs |  77.35 µs | `0%` / ![#006b1d](https://via.placeholder.com/15/006b1d/000000?text=+) `−83.55%` | [Link](https://github.com/believer/advent-of-code/blob/0cce6ca175c8d89e43772bc386f152bc6167edbd/rust/2020/src/day_02.rs)     |
|   6 | 627.01 µs | 696.92 µs |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/65cc61f1745db7579b47577769ba00f04f7e5f99/rust/2020/src/day_06.rs)     |
|   9 |  13.50 ms |  13.66 ms |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/cc12a3d53eeb1deebc747ca2daeafaed5a97b3f3/rust/2020/src/day_09.rs)     |
|  10 |   2.69 µs | 653.28 µs |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/7570c1e6123a3acb8687160e4415bf9aeac2d878/rust/2020/src/day_10.rs)     |
|  12 |  12.63 µs |  12.72 µs |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/209b56f57390cc6cc9fdc23a52369d2b0e7fb877/rust/2020/src/day_12.rs)     |
|  13 |  34.44 ms |   1.82 µs |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/ab0d72e1c582699ef888609a7b7555e8e514c5db/rust/2020/src/day_13.rs)     |
|  13 | 246.04 µs |   1.82 µs |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/ebb46bf2ed026f034601bea8434374b19fcc410c/rust/2020/src/day_13.rs)     |
|  15 | 252.60 µs |   10.54 s |                                                                         Baseline | [Link](https://github.com/believer/advent-of-code/blob/88d3b986791fff98db7551acc7a77c8069a1b6f8/rust/2020/src/day_15.rs)     |
