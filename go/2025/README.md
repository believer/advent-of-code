# Advent of Code 2025

Go again like last year for main solutions.

## Advent of Code 2025 Story

The Elves have good news and bad news.

The good news is that they've discovered project management! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.

The bad news is that they've realized they have a different emergency: according to their resource planning, none of them have any time left to decorate the North Pole!

To save Christmas, the Elves need you to finish decorating the North Pole by December 12th.

Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                                 | #1  |     #1 Answer | #2  |       #2 Answer |
| ------------------------------------------------------------------------------------------------------------------- | --- | ------------: | --- | --------------: |
| [Day 1: Secret Entrance](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day01/main.go)      | 🌟  |           997 | 🌟  |            5978 |
| [Day 2: Gift Shop](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day02/main.go)            | 🌟  |    9188031749 | 🌟  |     11323661261 |
| [Day 3: Lobby](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day03/main.go)                | 🌟  |         16993 | 🌟  | 168617068915447 |
| [Day 4: Printing Department](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day04/main.go)  | 🌟  |          1320 | 🌟  |            8354 |
| [Day 5: Cafeteria](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day05/main.go)            | 🌟  |           885 | 🌟  | 348115621205535 |
| [Day 6: Trash Compactor](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day06/main.go)      | 🌟  | 6209956042374 | 🌟  |  12608160008022 |
| [Day 7: Laboratories](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day07/main.go)         | 🌟  |          1711 | 🌟  |  36706966158365 |
| [Day 8: Playground](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day08/main.go)           | 🌟  |         69192 | 🌟  |      7264308110 |
| [Day 9: Movie Theater](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day09/main.go)        | 🌟  |    4772103936 | 🌟  |      1529675217 |
| [Day 10: Factory](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day10/main.go)             | 🌟  |           455 | 🌟  |           16978 |
| [Day 11: Reactor](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day11/main.go)             | 🌟  |           431 | 🌟  | 358458157650450 |
| [Day 12: Christmas Tree Farm](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day12/main.go) | 🌟  |           422 |     |                 |

## Benchmarks

Using Go's built-in benchmarking with the [testing](https://pkg.go.dev/testing#hdr-Benchmarks) package. Computer is a 2021 MacBook Pro M1 Pro, 32 GB RAM.

| Day |        #1 |        #2 | Improvement\*        |
| --- | --------: | --------: | -------------------- |
| 1   |    340532 |    338234 | `52,30%` / `52,86%`  |
| 2   |  39771972 | 778769562 | `-14,63%` / `46,44%` |
| 3   |    209025 |    276642 | `5,57%`              |
| 4   |    214451 |   5483916 |                      |
| 5   |    116258 |     68155 | `96,29%` / -         |
| 6   |    149919 |    986335 | `30,99%` / -         |
| 7   |    671799 |    508323 |                      |
| 8   |  66756016 |  64329155 |                      |
| 9   |    593637 | 375621306 | - / `90,43%`         |
| 10  | 256756802 |   Used Z3 |                      |
| 11  |    106015 |    311170 |                      |
| 12  |    274895 |         - |                      |

All values are ns/op. \* compared to first solution.

### Previous solutions

| Day |       #1 |         #2 | Improvement | Link                                                                                                                           |
| --: | -------: | ---------: | ----------: | ------------------------------------------------------------------------------------------------------------------------------ |
|   1 |   713866 |     717585 |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/53a02dfd3224b4c50893055608807375032416a2/go/2025/puzzles/day01/main.go) |
|   2 | 34694476 | 1454106125 |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/adfa07923db8b77f08edb19bb5f68435254517b1/go/2025/puzzles/day02/main.go) |
|   3 |   221365 |          - |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/84d03e23d56e216ae811d368f8dfad54f4eb21db/go/2025/puzzles/day03/main.go) |
|   5 |  3135281 |          - |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/0ebd61766589a47aba498000168f720633b7ee1a/go/2025/puzzles/day05/main.go) |
|   6 |   217234 |          - |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/987f84d6b6a172da55f724c519b7b225984fe492/go/2025/puzzles/day06/main.go) |
|   9 |        - | 3925386083 |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/7d21864ebe8ee554493f41879046d59f78c79282/go/2025/puzzles/day09/main.go) |

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
