import fs from 'fs/promises'

export const add = (a, b) => a + b
export const sum = add

export const sortDESC = (a, b) => b - a

export const readInput = async (day, parser) => {
  return await fs.readFile(`./inputs/day${day}.txt`, 'utf-8')
}

export const splitLines = input => input.split('\n')
export const splitWhitespace = input => input.split(' ')
export const removeEmpty = input => input.filter(l => l)

export const pipe =
  (...fns) =>
  x =>
    fns.reduce((acc, fn) => fn(acc), x)

export const map = fn => array => array.map(fn)

export const reduce = (reducer, initialArg) => array =>
  array.reduce(reducer, initialArg)
