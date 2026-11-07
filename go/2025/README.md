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

Using Go's built-in benchmarking with the [testing](https://pkg.go.dev/testing#hdr-Benchmarks) package. Computer is a 2021 MacBook Pro M1 Pro, 32 GB RAM. Go 1.27.3.

| Day |       #1 | #1 Memory |        #2 | #2 Memory | Time improvement\*  | Memory improvement\*  |
| --- | -------: | --------: | --------: | --------: | ------------------- | --------------------- |
| 1   |   183638 |  291.6 kB |    185994 |  291.6 kB | `74,28%` / `74,08%` | `-12,65%` / `-12,65%` |
| 2   |    18028 |    4.5 kB |     47607 |   41.8 kB | `99,95%` / `99,99%` | `99,97%` / `99,99%`   |
| 3   |    76212 |  220.4 kB |    110505 |  220.4 kB | `65,57%` / `58,05%` | `44,18%` / `61,93%`   |
| 4   |   236087 |  192.3 kB |   5574204 |  508.8 kB |                     |                       |
| 5   |   118341 |  142.7 kB |     71405 |  142.7 kB | `96,23%` / -        | `-9,30%` / -          |
| 6   |   153077 |  187.9 kB |     89268 |   25.3 kB | `29,53%` / `89,82%` | `35,80%` / `98,23%`   |
| 7   |   697391 |    1.3 MB |    524869 |  550.3 kB |                     |                       |
| 8   | 56495965 |   12.2 MB |  56413290 |   12.2 MB | `10,67%` / `10,74%` | `82,63%` / `82,64%`   |
| 9   |   176729 |   47.6 kB | 372113055 |   30.4 MB | `69,61%` / `90,52%` | `0,00%` / `13,31%`    |
| 10  |   531942 |  747.5 kB |   Used Z3 |         - | `99,79%`            | `99,98%`              |
| 11  |   107265 |  180.2 kB |    316487 |  331.1 kB |                     |                       |
| 12  |   257945 |  478.4 kB |         - |         - |                     |                       |

Times are ns/op and memory is B/op allocated (from `-benchmem`). \* compared to first solution. A negative memory improvement means the current solution allocates more than the first one did.

### Previous solutions

| Day |        #1 | #1 Memory |         #2 | #2 Memory | Improvement | Link                                                                                                                           |
| --: | --------: | --------: | ---------: | --------: | ----------: | ------------------------------------------------------------------------------------------------------------------------------ |
|   1 |    713866 |  258.9 kB |     717585 |  258.9 kB |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/53a02dfd3224b4c50893055608807375032416a2/go/2025/puzzles/day01/main.go) |
|   2 |  34694476 |   17.1 MB | 1454106125 |    2.0 GB |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/adfa07923db8b77f08edb19bb5f68435254517b1/go/2025/puzzles/day02/main.go) |
|   3 |    221365 |  394.7 kB |          - |         - |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/84d03e23d56e216ae811d368f8dfad54f4eb21db/go/2025/puzzles/day03/main.go) |
|   3 |         - |         - |     263429 |  578.8 kB |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/7ddbbc020d781055e2b18f9d9e7b677c042812ad/go/2025/puzzles/day03/main.go) |
|   5 |   3135281 |  130.5 kB |          - |         - |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/0ebd61766589a47aba498000168f720633b7ee1a/go/2025/puzzles/day05/main.go) |
|   6 |    217234 |  292.7 kB |          - |         - |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/987f84d6b6a172da55f724c519b7b225984fe492/go/2025/puzzles/day06/main.go) |
|   6 |         - |         - |     876634 |    1.4 MB |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/7ddbbc020d781055e2b18f9d9e7b677c042812ad/go/2025/puzzles/day06/main.go) |
|   8 |  63245626 |   70.3 MB |   63199803 |   70.3 MB |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/7ddbbc020d781055e2b18f9d9e7b677c042812ad/go/2025/puzzles/day08/main.go) |
|   9 |         - |         - | 3925386083 |   35.0 MB |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/7d21864ebe8ee554493f41879046d59f78c79282/go/2025/puzzles/day09/main.go) |
|   9 |    581487 |   47.6 kB |          - |         - |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/7ddbbc020d781055e2b18f9d9e7b677c042812ad/go/2025/puzzles/day09/main.go) |
|  10 | 247743677 |    4.2 GB |          - |         - |    Baseline | [Link](https://github.com/believer/advent-of-code/blob/7ddbbc020d781055e2b18f9d9e7b677c042812ad/go/2025/puzzles/day10/main.go) |

Memory for the older baselines was measured afterwards on Go 1.27.3, since only times were recorded originally.

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
