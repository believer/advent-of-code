import { expect, test } from 'bun:test'
import { parseInput, parseInputPart2, part1 } from './day1'

const exampleData = `1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`

const exampleData2 = `two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen`

test('day 1 / part 1 - example data passes', () => {
	const data = parseInput(exampleData)
	expect(part1(data)).toBe(142)
})

test('day 1 / part 2 - example data passes', () => {
	const data = parseInputPart2(exampleData2)
	expect(part1(data)).toBe(281)
})

test('handles extra everything', () => {
	expect(parseInputPart2('fourfourthreehnbhkmscqxdfksg64bvpppznkh')).toEqual([
		[4, 4],
	])
})
