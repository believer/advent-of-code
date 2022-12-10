import {
  sum,
  readInput,
  sortDESC,
  splitLines,
  removeEmpty,
  map,
  reduce,
  add,
  pipe,
} from './utils.mjs'

const input = await readInput('01')

const parsedInput = input
  .split('\n\n')
  .flatMap(pipe(splitLines, removeEmpty, map(Number), reduce(add, 0)))
  .slice()
  .sort(sortDESC)

export const part1 = parsedInput.at(0)
export const part2 = parsedInput.slice(0, 3).reduce(sum, 0)
