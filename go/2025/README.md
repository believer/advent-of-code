# Advent of Code 2025

Go again like last year for main solutions.

## Advent of Code 2025 Story

The Elves have good news and bad news.

The good news is that they've discovered project management! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.

The bad news is that they've realized they have a different emergency: according to their resource planning, none of them have any time left to decorate the North Pole!

To save Christmas, the Elves need you to finish decorating the North Pole by December 12th.

Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                            | #1  |  #1 Answer | #2  |   #2 Answer |
| -------------------------------------------------------------------------------------------------------------- | --- | ---------: | --- | ----------: |
| [Day 1: Secret Entrance](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day01/main.go) | 🌟  |        997 | 🌟  |        5978 |
| [Day 2: Gift Shop](https://github.com/believer/advent-of-code/blob/master/go/2025/puzzles/day02/main.go)       | 🌟  | 9188031749 | 🌟  | 11323661261 |

## Benchmarks

Using Go's built-in benchmarking with the [testing](https://pkg.go.dev/testing#hdr-Benchmarks) package. Computer is a 2021 MacBook Pro M1 Pro, 32 GB RAM.

| Day |     #1 |     #2 | Improvement\* |
| --- | -----: | -----: | ------------- |
| 1   | 713866 | 717585 |               |

\* compared to first solution

### Previous solutions

| Day |  #1 |  #2 | Improvement | Link |
| --: | --: | --: | ----------: | ---- |
|   x |     |     |             |      |

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
