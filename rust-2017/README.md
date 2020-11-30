# Advent of Code 2017

Trying some solutions in Rust.

## Advent of Code 2017 Story

The night before Christmas, one of Santa's Elves calls you in a panic. "The printer's broken! We can't print the Naughty or Nice List!" By the time you make it to sub-basement 17, there are only a few minutes until midnight. "We have a big problem," she says; "there must be almost fifty bugs in this system, but nothing else can print The List. Stand in this square, quick! There's no time to explain; if you can convince them to pay you in stars, you'll be able to--" She pulls a lever and the world goes blurry.

When your eyes can focus again, everything seems a lot more pixelated than before. She must have sent you inside the computer! You check the system clock: 25 milliseconds until midnight. With that much time, you should be able to collect all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day millisecond in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                               | #1  |  #2 | Performance        |
| ------------------------------------------------------------------------------------------------- | --- | --- | ------------------ |
| [Day 1: Inverse Captcha](https://github.com/believer/advent-of-code/blob/master/rust-2017/day_01) | 🌟  | 🌟  | 3.49 µs / 15.01 µs |

## Performance

I've tested performance using `std::time::Instant` and taken the fastest time from a couple of runs with release build. This is not the most exact measure and times this fast can probably be wrong, but it's at least some form of measure of how fast Rust is.
