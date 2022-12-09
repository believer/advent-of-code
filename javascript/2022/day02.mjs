import {
  readInput,
  sum,
  splitWhitespace,
  splitLines,
  removeEmpty,
  pipe,
  map,
  add,
  reduce,
} from './utils.mjs'

const input = await readInput('02')
const parsedInput = pipe(splitLines, removeEmpty, map(splitWhitespace))(input)

const scores = {
  win: 6,
  loss: 0,
  draw: 3,
  rock: 1,
  paper: 2,
  scissors: 3,
}

const player1 = {
  A: 'rock',
  B: 'paper',
  C: 'scissors',
}

const player2 = {
  X: 'rock',
  Y: 'paper',
  Z: 'scissors',
}

const outcome = {
  X: 'loss',
  Y: 'draw',
  Z: 'win',
}

const part1 = pipe(
  map(([p1, p2]) => [player1[p1], player2[p2]]),
  map(([p1, p2]) => {
    if (p1 === p2) return scores.draw + scores[p2]
    if (p1 === 'rock' && p2 === 'paper') return scores.win + scores.paper
    if (p1 === 'rock' && p2 === 'scissors') return scores.loss + scores.scissors
    if (p1 === 'paper' && p2 === 'rock') return scores.loss + scores.rock
    if (p1 === 'paper' && p2 === 'scissors') return scores.win + scores.scissors
    if (p1 === 'scissors' && p2 === 'rock') return scores.win + scores.rock
    if (p1 === 'scissors' && p2 === 'paper') return scores.loss + scores.paper
  }),
  reduce(add, 0)
)(parsedInput)

const part2 = pipe(
  map(([p1, p2]) => [player1[p1], outcome[p2]]),
  map(([p1, p2]) => {
    if (p1 === 'rock' && p2 === 'loss') return scores.loss + scores.scissors
    if (p1 === 'rock' && p2 === 'draw') return scores.draw + scores.rock
    if (p1 === 'rock' && p2 === 'win') return scores.win + scores.paper
    if (p1 === 'paper' && p2 === 'loss') return scores.loss + scores.rock
    if (p1 === 'paper' && p2 === 'draw') return scores.draw + scores.paper
    if (p1 === 'paper' && p2 === 'win') return scores.win + scores.scissors
    if (p1 === 'scissors' && p2 === 'loss') return scores.loss + scores.paper
    if (p1 === 'scissors' && p2 === 'draw') return scores.draw + scores.scissors
    if (p1 === 'scissors' && p2 === 'win') return scores.win + scores.rock
  }),
  reduce(add, 0)
)(parsedInput)

console.log('Part 1:', part1)
console.log('Part 2:', part2)
