import fs from 'fs/promises'

export const sum = (acc, curr) => acc + curr

export const sortDESC = (a, b) => b - a

export const readInput = async (day, parser) => {
  return await fs.readFile(`./inputs/day${day}.txt`, 'utf-8')
}
