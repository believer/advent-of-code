# Advent of Code 2019

This is my second attempt at Advent of Code 🎅🏻, my first time around I only lasted
a couple of days. My language of choice is ReasonML, same as last year.

## Advent of Code 2019 Story

Santa has become stranded at the edge of the Solar System while delivering presents to other planets! To accurately calculate his position in space, safely align his warp drive, and return to Earth in time to save Christmas, he needs you to bring him measurements from fifty stars.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Days

| Day                                                                                                                                                   | #1  |  #2 | Performance       |
| ----------------------------------------------------------------------------------------------------------------------------------------------------- | --- | --- | ----------------- |
| [Day 1: The Tyranny of the Rocket Equation](https://github.com/believer/advent-of-code/tree/master/2019/src/day-1-the-tyranny-of-the-rocket-equation) | 🌟  | 🌟  | 7 µs / 43 µs      |
| [Day 2: 1202 Program Alarm](https://github.com/believer/advent-of-code/tree/master/2019/src/day-2-1202-program-alarm)                                 | 🌟  | 🌟  | 5 µs / 202 ms     |
| [Day 3: Crossed Wires](https://github.com/believer/advent-of-code/tree/master/2019/src/day-3-crossed-wires)                                           | 🌟  | 🌟  | 643 ms / 771 ms   |
| [Day 4: Secure Container](https://github.com/believer/advent-of-code/tree/master/2019/src/day-4-secure-container)                                     | 🌟  | 🌟  | 1483 ms / 1545 ms |
| [Day 5: Sunny with a Chance of Asteroids](https://github.com/believer/advent-of-code/tree/master/2019/src/day-5-sunny-with-a-chance-of-asteroids)     | 🌟  | 🌟  | 37 µs / 26 µs     |
| [Day 6: Universal Orbit Map](https://github.com/believer/advent-of-code/tree/master/2019/src/day-6-universal-orbit-map)                               | 🌟  |     | 118 ms            |
| [Day 8: Space Image Format](https://github.com/believer/advent-of-code/tree/master/2019/src/day-8-space-image-format)                                 | 🌟  | 🌟  | 1.6 ms / 309 µs   |

## Performance

I've tested performance using `performance.now()` and taken the fastest time
from a couple of runs.

## Running code and tests

```bash
$ git clone https://github.com/believer/advent-of-code.git
$ cd advent-of-code/2019
$ npm install
$ npm run build && npm run test:ci
```
