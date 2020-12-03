# Advent of Code 2020

Trying out some solutions in ReScript to compare performance against my main
solutions in [Rust](/rust-2020) this year.

## Advent of Code 2020 Story

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                           | #1  |  #2 | Performance   |
| ------------------------------------------------------------------------------------------------------------- | --- | --- | ------------- |
| [Day 1: Report Repair](https://github.com/believer/advent-of-code/blob/master/rescript/2020/src/day-01)       | 🌟  | 🌟  | 5 ms / 6 ms   |
| [Day 2: Password Philosophy](https://github.com/believer/advent-of-code/blob/master/rescript/2020/src/day-02) | 🌟  | 🌟  | 13 ms / 7 ms  |
| [Day 3: Toboggan Trajectory](https://github.com/believer/advent-of-code/blob/master/rescript/2020/src/day-03) | 🌟  | 🌟  | 8 ms / 0 ms\* |

## Performance

I've tested performance using `performance.now()` and taken the fastest time
from a couple of runs. Computer is a MacBook 2,6 GHz 6-Core i7, 32 GB RAM, Radeon 5300M 4 GB.

\* `0 ms` when performance is microseconds (I guess).
