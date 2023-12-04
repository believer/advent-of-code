import { match } from 'ts-pattern'
import { readInput } from '../utils'

type Input = ReturnType<typeof parseInput>

// biome-ignore format: Looks better this way
const numbers = [
	'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine',
  '1', '2', '3', '4', '5', '6', '7', '8', '9',
]

export function parseInput(input: string) {
	return input.split('\n').map((line) =>
		line
			.split('')
			.filter(Number)
			.map((x) => parseInt(x))
	)
}

function parseNumber(number: string) {
	return match(number)
		.with('one', '1', () => 1)
		.with('two', '2', () => 2)
		.with('three', '3', () => 3)
		.with('four', '4', () => 4)
		.with('five', '5', () => 5)
		.with('six', '6', () => 6)
		.with('seven', '7', () => 7)
		.with('eight', '8', () => 8)
		.with('nine', '9', () => 9)
		.otherwise(() => 0)
}

function findNumber(line: string, lookFrom: 'start' | 'end') {
	return (
		numbers
			.map((number) => {
				let index = line.indexOf(number)

				if (lookFrom === 'end') {
					index = line.lastIndexOf(number)
				}

				if (index === -1) {
					return null
				}

				return [index, number] as const
			})
			.filter(Boolean)
			.sort(([a, _x], [b, _y]) => a - b)
			.at(lookFrom === 'start' ? 0 : -1) ?? []
	)
}

export function parseInputPart2(input: string) {
	return input.split('\n').map((line) => {
		const [, first] = findNumber(line, 'start')
		const [, last] = findNumber(line, 'end')

		if (!first || !last) {
			return []
		}

		return [parseNumber(first), parseNumber(last)]
	})
}

export function part1(input: Input) {
	let sum = 0

	for (const digits of input) {
		const first = digits.at(0) ?? 0
		const last = digits.at(-1) ?? first

		sum += first * 10 + last
	}

	return sum
}

export async function day1() {
	const input = await readInput(1)
	const data = parseInput(input)
	const data2 = parseInput(input)

	console.time('part1')
	console.log(`Day 1 / Part 1: ${part1(data)}`)
	console.timeEnd('part1')

	console.time('part2')
	console.log(`Day 1 / Part 2: ${part1(data2)}`)
	console.timeEnd('part2')
}
