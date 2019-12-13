# Advent of Code 2019

This is my second attempt at Advent of Code 🎅🏻, my first time around I only lasted
a couple of days. My language of choice is ReasonML, same as last year.

## Advent of Code 2019 Story

Santa has become stranded at the edge of the Solar System while delivering presents to other planets! To accurately calculate his position in space, safely align his warp drive, and return to Earth in time to save Christmas, he needs you to bring him measurements from fifty stars.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Native Reason

I'm also making some of the solutions in native Reason to compare performance
and to have even more fun! Those solutions are [found
here](https://github.com/believer/advent-of-code/tree/master/advent-of-native)
or via the relevant native link in the table below.

## Days

| Day                                                                                                                                                    | #1  |  #2 | Performance       | Native                                                                                        |
| ------------------------------------------------------------------------------------------------------------------------------------------------------ | --- | --- | ----------------- | --------------------------------------------------------------------------------------------- |
| [Day 1: The Tyranny of the Rocket Equation](https://github.com/believer/advent-of-code/tree/master/2019/src/day-01-the-tyranny-of-the-rocket-equation) | 🌟  | 🌟  | 7 µs / 43 µs      | [Link](https://github.com/believer/advent-of-code/blob/master/advent-of-native/lib/DayOne.re) |
| [Day 2: 1202 Program Alarm](https://github.com/believer/advent-of-code/tree/master/2019/src/day-02-1202-program-alarm)                                 | 🌟  | 🌟  | 5 µs / 202 ms     | [Link](https://github.com/believer/advent-of-code/blob/master/advent-of-native/lib/DayTwo.re) |
| [Day 3: Crossed Wires](https://github.com/believer/advent-of-code/tree/master/2019/src/day-03-crossed-wires)                                           | 🌟  | 🌟  | 643 ms / 771 ms   |                                                                                               |
| [Day 4: Secure Container](https://github.com/believer/advent-of-code/tree/master/2019/src/day-04-secure-container)                                     | 🌟  | 🌟  | 1.483 s / 1.545 s |                                                                                               |
| [Day 5: Sunny with a Chance of Asteroids](https://github.com/believer/advent-of-code/tree/master/2019/src/day-05-sunny-with-a-chance-of-asteroids)     | 🌟  | 🌟  | 37 µs / 26 µs     |                                                                                               |
| [Day 6: Universal Orbit Map](https://github.com/believer/advent-of-code/tree/master/2019/src/day-06-universal-orbit-map)                               | 🌟  | 🌟  | 9456 µs / 338 µs  | [Link](https://github.com/believer/advent-of-code/blob/master/advent-of-native/lib/Day6.re)    |
| [Day 7: Amplification Circuit](https://github.com/believer/advent-of-code/tree/master/2019/src/day-07-amplification-circuit)                           | 🌟  |     | 597 µs            |                                                                                               |
| [Day 8: Space Image Format](https://github.com/believer/advent-of-code/tree/master/2019/src/day-08-space-image-format)                                 | 🌟  | 🌟  | 1.6 ms / 309 µs   |                                                                                               |
| [Day 10: Monitoring Station](https://github.com/believer/advent-of-code/tree/master/2019/src/day-10-monitoring-station)                                | 🌟  |     | 190 ms            |                                                                                               |
| [Day 12: The N-Body Problem](https://github.com/believer/advent-of-code/tree/master/2019/src/day-12-the-n-body-problem)                                | 🌟  | 🌟  | 1301 µs / 3.399 s | [Link](https://github.com/believer/advent-of-code/blob/master/advent-of-native/lib/Day12.re)  |

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
