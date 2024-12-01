# Advent of Code 2024

I've been enjoying Go this year and thought I might learn more by doing AoC with it too.

## Advent of Code 2024 Story

The Chief Historian is always present for the big Christmas sleigh launch, but nobody has seen him in months! Last anyone heard, he was visiting locations that are historically significant to the North Pole; a group of Senior Historians has asked you to accompany them as they check the places they think he was most likely to visit.

As each location is checked, they will mark it on their list with a star. They figure the Chief Historian must be in one of the first fifty places they'll look, so in order to save Christmas, you need to help them get fifty stars on their list before Santa takes off on December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                               | #1  | #1 Answer | #2  | #2 Answer |
| ----------------------------------------------------------------------------------------------------------------- | --- | --------: | --- | --------: |
| [Day 1: Historian Hysteria](https://github.com/believer/advent-of-code/blob/master/go/2024/puzzles/day01/main.go) | 🌟  |   1666427 | 🌟  |  24316233 |

## Benchmarks

| Day |           #1 |           #2 | Improvement\* |
| --- | -----------: | -----------: | ------------- |
| 1   | 120513 ns/op | 415683 ns/op |               |

Using Go's built-in benchmarking with the [testing](https://pkg.go.dev/testing#hdr-Benchmarks) package.. Computer is a 2021 MacBook Pro M1 Pro, 32 GB RAM.

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
