import { sum, readInput, sortDESC } from './utils.mjs'

const input = await readInput('01')

const parsedInput = input
  .split('\n\n')
  .flatMap(group =>
    group
      .split('\n')
      .filter(l => l)
      .map(Number)
      .reduce(sum, 0)
  )
  .slice()
  .sort(sortDESC)

const part1 = parsedInput.at(0)
const part2 = parsedInput.slice(0, 3).reduce(sum, 0)

console.log('Part 1:', part1)
console.log('Part 2:', part2)
