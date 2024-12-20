# Advent of Code 2024

I've been enjoying Go this year and thought I might learn more by doing AoC with it too.

## Advent of Code 2024 Story

The Chief Historian is always present for the big Christmas sleigh launch, but nobody has seen him in months! Last anyone heard, he was visiting locations that are historically significant to the North Pole; a group of Senior Historians has asked you to accompany them as they check the places they think he was most likely to visit.

As each location is checked, they will mark it on their list with a star. They figure the Chief Historian must be in one of the first fifty places they'll look, so in order to save Christmas, you need to help them get fifty stars on their list before Santa takes off on December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                                    | #1  |         #1 Answer | #2  |       #2 Answer |
| ---------------------------------------------------------------------------------------------------------------------- | --- | ----------------: | --- | --------------: |
| [Day 1: Historian Hysteria](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day01/main.go)      | 🌟  |           1666427 | 🌟  |        24316233 |
| [Day 2: Red-Nosed Reports](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day02/main.go)       | 🌟  |               564 | 🌟  |             604 |
| [Day 3: Mull It Over](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day03/main.go)            | 🌟  |         161085926 | 🌟  |        82045421 |
| [Day 4: Ceres Search](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day04/main.go)            | 🌟  |              2562 | 🌟  |            1902 |
| [Day 5: Print Queue](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day05/main.go)             | 🌟  |              3608 | 🌟  |            4922 |
| [Day 6: Guard Gallivant](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day06/main.go)         | 🌟  |              4778 | 🌟  |            1618 |
| [Day 7: Bridge Repair](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day07/main.go)           | 🌟  |     1399219271639 | 🌟  | 275791737999003 |
| [Day 8: Resonant Collinearity](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day08/main.go)   | 🌟  |               220 | 🌟  |             813 |
| [Day 9: Disk Fragmenter](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day09/main.go)         | 🌟  |     6384282079460 | 🌟  |   6408966547049 |
| [Day 10: Hoof It](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day10/main.go)                | 🌟  |               652 | 🌟  |            1432 |
| [Day 11: Plutonian Pebbles](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day11/main.go)      | 🌟  |            187738 | 🌟  | 223767210249237 |
| [Day 12: Garden Groups](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day12/main.go)          | 🌟  |           1522850 | 🌟  |          953738 |
| [Day 13: Claw Contraption](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day13/main.go)       | 🌟  |             26299 | 🌟  | 107824497933339 |
| [Day 14: Restroom Redoubt](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day14/main.go)       | 🌟  |         228690000 | 🌟  |            7093 |
| [Day 15: Warehouse Woes](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day15/main.go)         | 🌟  |           1526018 | 🌟  |         1550677 |
| [Day 16: Reindeer Maze](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day16/main.go)          | 🌟  |             78428 | 🌟  |             463 |
| [Day 17: Chronospatial Computer](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day17/main.go) | 🌟  | 2,0,1,3,4,0,2,1,7 | 🌟  | 236580836040301 |
| [Day 18: RAM Run](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day18/main.go)                | 🌟  |               288 | 🌟  |            52,5 |
| [Day 19: Linen Layout](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day19/main.go)           | 🌟  |               209 | 🌟  | 777669668613191 |
| [Day 20: Race Condition](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day20/main.go)         | 🌟  |              1351 | 🌟  |          966130 |

## Benchmarks

Using Go's built-in benchmarking with the [testing](https://pkg.go.dev/testing#hdr-Benchmarks) package. Computer is a 2021 MacBook Pro M1 Pro, 32 GB RAM.

| Day |              #1 |               #2 | Improvement\*       |
| --- | --------------: | ---------------: | ------------------- |
| 1   |    116264 ns/op |     131233 ns/op | `3.53%` / `68.43%`  |
| 2   |    310935 ns/op |     723512 ns/op |                     |
| 3   |    336448 ns/op |     785320 ns/op | - / `36.98%`        |
| 4   |    250981 ns/op |     110358 ns/op | `91.24%` / `62.52%` |
| 5   |    778880 ns/op |    3129873 ns/op | `53.34%` / `81.91%` |
| 6   |    312461 ns/op | 1153391125 ns/op |                     |
| 7   |  16480300 ns/op |  842853000 ns/op | `87.01%` / `91.67%` |
| 8   |     58749 ns/op |     121247 ns/op |                     |
| 9   | 381476181 ns/op |  171042257 ns/op |                     |
| 10  |   1424599 ns/op |    1789071 ns/op | `64.68%` / `64.01%` |
| 11  |    424021 ns/op |   15488584 ns/op |                     |
| 12  |   6677348 ns/op |   12339733 ns/op | `39.21%` / `26.80%` |
| 13  |    698173 ns/op |     702380 ns/op | `75.93%` / -        |
| 14  |     55522 ns/op |   56488050 ns/op | `90.67%` / -        |
| 15  |    477450 ns/op |    3498110 ns/op |                     |
| 16  |  97868160 ns/op |   99166694 ns/op |                     |
| 17  |     13964 ns/op |     561424 ns/op |                     |
| 18  |    755414 ns/op |    1996995 ns/op |                     |
| 19  |  16126963 ns/op |   16206756 ns/op |                     |
| 20  | 762519479 ns/op |  845552729 ns/op | `85.99%` / `63.78%` |

\* compared to first solution

### Previous solutions

| Day |              #1 |                #2 |         Improvement | Link                                                                                                                           |
| --: | --------------: | ----------------: | ------------------: | ------------------------------------------------------------------------------------------------------------------------------ |
|   1 |    120513 ns/op |      415683 ns/op |            Baseline | [Link](https://github.com/believer/advent-of-code/blob/47447cc17fffe6994d4b54c4cb815e698b3f5605/go/2024/puzzles/day01/main.go) |
|   1 |    120513 ns/op |      155479 ns/op |        - / `62,59%` | [Link](https://github.com/believer/advent-of-code/blob/ea42592462771b74de87eae6bea9c0ca892a4499/go/2024/puzzles/day01/main.go) |
|   3 |    336448 ns/op |     1246155 ns/op |            Baseline | [Link](https://github.com/believer/advent-of-code/blob/461c2dd40039c27102aa1790c650decb79d4f549/go/2024/puzzles/day03/main.go) |
|   4 |   2864606 ns/op |      294413 ns/op |            Baseline | [Link](https://github.com/believer/advent-of-code/blob/99909bb30f82cda079471134452d886a0eb6266f/go/2024/puzzles/day04/main.go) |
|   4 |    523315 ns/op |      217584 ns/op | `81.73%` / `26.09%` | [Link](https://github.com/believer/advent-of-code/blob/431059e6b64faba3bc67c293b57ae299d3525bb9/go/2024/puzzles/day04/main.go) |
|   5 |   1669175 ns/op |    17299190 ns/op |            Baseline | [Link](https://github.com/believer/advent-of-code/blob/1db858ae3d391319511787d8935c76eecdf6b22f/go/2024/puzzles/day05/main.go) |
|   7 | 126892714 ns/op | 10124683583 ns/op |            Baseline | [Link](https://github.com/believer/advent-of-code/blob/dd735747021ce43ca3a7427c529813139737271e/go/2024/puzzles/day07/main.go) |
|   7 | 110164692 ns/op |  7135839625 ns/op | `13.18%` / `29.52%` | [Link](https://github.com/believer/advent-of-code/blob/640d9604dfefa71f7bfef876750f378bd1a58a8b/go/2024/puzzles/day07/main.go) |
|  10 |   1424599 ns/op |     1789071 ns/op |            Baseline | [Link](https://github.com/believer/advent-of-code/blob/431059e6b64faba3bc67c293b57ae299d3525bb9/go/2024/puzzles/day10/main.go) |
|  12 |  10984420 ns/op |    16856988 ns/op |            Baseline | [Link](https://github.com/believer/advent-of-code/blob/7a220ed0e6deae74d0a293615e6348e6ce1a9a22/go/2024/puzzles/day12/main.go) |
|  13 |   2900453 ns/op |      702380 ns/op |            Baseline | [Link](https://github.com/believer/advent-of-code/blob/0cf31064eb05f384cebe45cbeaf80ba90e0947ce/go/2024/puzzles/day13/main.go) |
|  14 |    594981 ns/op |    56488050 ns/op |            Baseline | [Link](https://github.com/believer/advent-of-code/blob/a3f28eb2691d3e4be60ec56ab7f699332a2b3d31/go/2024/puzzles/day14/main.go) |
|  20 | 762519479 ns/op |   845552729 ns/op |                     | [Link](https://github.com/believer/advent-of-code/blob/305eba9ced6b40ecce606cf19f7cb9fc00e5ed73/go/2024/puzzles/day20/main.go) |

## Running

Run a day

```
make run day=<number>
```

Run test for a day

```
make test day=<number>
```

Create a new day

```
make create-day day=<number>
```
