# Advent of Code 2020

This year I'm doing the solutions in Rust. I've started liking it a lot during the year and want to see what type of performance I can get.

## Advent of Code 2020 Story

After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.

The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.

To save your vacation, you need to get all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                             | #1            |  #2              |
| ----------------------------------------------------------------------------------------------- | ------------- | ---------------- |
| [Day 1: Report Repair](https://github.com/believer/advent-of-code/blob/master/rust/2020/day_01) | **898299** 🌟 | **143933922** 🌟 |

## Performance

With the help of [cargo-aoc](https://github.com/gobanos/cargo-aoc) I get automatic benchmarking using [Criterion](https://github.com/bheisler/criterion.rs). Computer is a MacBook Pro 2,6 GHz 6-Core i7, 32 GB RAM, Radeon 5300M 4 GB.

| Day | #1      | #2      | Note                              |
| --- | ------- | ------- | --------------------------------- |
| 1   | 1.21 µs | 16.7 µs | Naive solution 16.06 µs / 4.38 ms |
