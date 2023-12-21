# Advent of Code 2023

Rust is still fun, but I might also do some [Go](https://github.com/believer/advent-of-code/tree/master/go/2023) and also [TypeScript with Bun](https://github.com/believer/advent-of-code/tree/master/typescript/2023) this year. Will try to pair with ChatGPT to expand my knowledge on the algorithms that can be used.

## Advent of Code 2023 Story

Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

## Days

| Day                                                                                                                      | #1  |  #1 Answer | #2  |       #2 Answer |
| ------------------------------------------------------------------------------------------------------------------------ | --- | ---------: | --- | --------------: |
| [Day 1: Trebuchet?!](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_01.rs)                     | 🌟  |      54304 | 🌟  |           54418 |
| [Day 2: Cube Conundrum](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_02.rs)                  | 🌟  |       2162 | 🌟  |           72513 |
| [Day 3: Gear Ratios](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_03.rs)                     | 🌟  |     535235 | 🌟  |        79844424 |
| [Day 4: Scratchcards](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_04.rs)                    | 🌟  |      27454 | 🌟  |         6857330 |
| [Day 5: If You Give A Seed A Fertilizer](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_05.rs) | 🌟  |  993500720 | 🌟  |         4917124 |
| [Day 6: Wait For It](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_06.rs)                     | 🌟  |     861300 | 🌟  |        28101347 |
| [Day 7: Camel Cards](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_07.rs)                     | 🌟  |  250474325 | 🌟  |       248909434 |
| [Day 8: Haunted Wasteland](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_08.rs)               | 🌟  |      22411 | 🌟  |  11188774513823 |
| [Day 9: Mirage Maintenance](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_09.rs)              | 🌟  | 1853145119 | 🌟  |             923 |
| [Day 10: Pipe Maze](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_10.rs)                      | 🌟  |       6882 | 🌟  |             491 |
| [Day 11: Cosmic Expansion](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_11.rs)               | 🌟  |    9795148 | 🌟  |    650672493820 |
| [Day 12: Hot Springs](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_12.rs)                    | 🌟  |       8270 | 🌟  | 204640299929836 |
| [Day 13: Point of Incidence](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_13.rs)             | 🌟  |      27300 | 🌟  |           29276 |
| [Day 14: Parabolic Reflector Dish](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_14.rs)       | 🌟  |     108614 | 🌟  |           96447 |
| [Day 15: Lens Library](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_15.rs)                   | 🌟  |     516070 | 🌟  |          244981 |
| [Day 16: The Floor Will Be Lava](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_16.rs)         | 🌟  |       7562 | 🌟  |            7793 |
| [Day 17: Clumsy Crucible](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_17.rs)                | 🌟  |       1013 | 🌟  |            1215 |
| [Day 18: Lavaduct Lagoon](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_18.rs)                | 🌟  |      48652 | 🌟  |  45757884535661 |
| [Day 19: Aplenty](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_19.rs)                        | 🌟  |     331208 | 🌟  | 121464316215623 |
| [Day 20: Pulse Propagation](https://github.com/believer/advent-of-code/blob/master/rust/2023/src/day_19.rs)              | 🌟  |  812609846 | 🌟  | 245114020323037 |

## Performance

With the help of [cargo-aoc](https://github.com/gobanos/cargo-aoc) I get automatic benchmarking using [Criterion](https://github.com/bheisler/criterion.rs). Computer is a MacBook Pro M1 Pro, 32 GB RAM.

| Day |        #1 |            #2 | Improvement\*         |             Generator |
| --- | --------: | ------------: | --------------------- | --------------------: |
| 1   |  40.71 µs |     834.08 µs | - / `-31.03%`         |             441.01 ns |
| 2   | 764.68 ns |       1.75 µs |                       |              47.81 µs |
| 3   | 159.61 µs |       7.72 ms | `-8.44%` / -          |              86.83 µs |
| 4   |  12.85 µs |      13.09 µs | `-48.41%` / `-46.64%` |             230.06 µs |
| 5   |   1.27 µs | 238.45 ms\*\* | `-16.72%` / `-98.87%` |              16.85 µs |
| 6   | 101.59 ns |      15.04 ms |                       | 257.13 ns / 204.78 ns |
| 7   | 364.18 µs |     359.22 µs |                       | 318.23 µs / 324.96 µs |
| 8   | 926.09 µs |       4.47 ms | - / `-70.69%`         |             137.33 µs |
| 9   |   2.44 µs |       2.55 µs |                       |             434.07 µs |
| 10  |  65.17 µs |      65.01 µs | `-96.02%` / -         |              50.84 µs |
| 11  | 846.06 µs |     844.65 µs |                       |              75.21 µs |
| 12  |   2.65 ms |     149.26 ms |                       |             113.10 µs |
| 13  |  24.87 µs |      17.61 µs |                       |             169.28 µs |
| 14  |  19.30 µs |      20.22 µs |                       |              23.69 µs |
| 15  |  19.99 µs |     210.88 µs | - / `-28.20%`         |             241.12 µs |
| 16  |   1.41 ms |      49.39 ms |                       |              30.33 µs |
| 17  |  64.21 ms |     173.37 ms |                       |              47.36 µs |
| 18  | 837.59 ns |     844.43 ns |                       |   41.30 µs / 73.29 µs |
| 19  |  55.27 µs |     160.90 µs |                       |             213.26 µs |
| 20  |   5.22 ms |      24.31 ms |                       |              24.13 µs |

\* compared to first solution<br/>
\*\* slow, didn't benchmark. Value comes from running the solver.

### Previous solutions

| Day |         #1 |        #2 | Improvement | Link                                                                                                                     |
| --: | ---------: | --------: | ----------: | ------------------------------------------------------------------------------------------------------------------------ |
|   1 |          - |   1.19 ms |             | [Link](https://github.com/believer/advent-of-code/blob/8742cd4d71869a886595bfb363895ca4e9cd84f1/rust/2023/src/day_01.rs) |
|   3 |  172.23 µs |         - |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/75a83e31024bbac99a0664f81fce4e13ec1e94af/rust/2023/src/day_03.rs) |
|   4 |   24.33 µs |  24.32 µs |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/c970c6322d3904048bcf3f30b1052e2916476d73/rust/2023/src/day_04.rs) |
|   5 |    1.50 µs |   21.22 s |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/39b0904c4921f4ae79963a6df49bb3502ef6b3be/rust/2023/src/day_05.rs) |
|   8 |          - |  15.25 ms |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/912d70c6e04ffd97f766c79b90764c105fe2f6ce/rust/2023/src/day_08.rs) |
|  10 | 1.70 ms \* |         - |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/ebbbbb8cb26e0fa4858cc48cf1a00304b4eee3a7/rust/2023/src/day_10.rs) |
|  15 |          - | 292.04 µs |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/5040f0fb6d9888b4a4d28f976e44ed077e0337b2/rust/2023/src/day_15.rs) |

\* BFS solution. I liked it, so it's still in the file.
