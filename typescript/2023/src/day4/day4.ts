import { numbersToSet, readInput, setIntersection } from '../utils'

type Input = ReturnType<typeof parseInput>

export function parseInput(input: string) {
	return input
		.split('\n')
		.filter(Boolean)
		.map((line) => {
			const [_, numbers] = line.split(': ')
			const [winning, my] = numbers.split(' | ')

			return [numbersToSet(winning), numbersToSet(my)]
		})
}

export function part1(input: Input) {
	return input.reduce((acc, [winning, myCard]) => {
		const myWinningNumbers = setIntersection(winning, myCard)

		// If we run the formula on a set with no elements, we get 2^-1, which is 0.5
		if (myWinningNumbers.size === 0) {
			return acc
		}

		return acc + 2 ** (myWinningNumbers.size - 1)
	}, 0)
}

export function part2(input: Input) {
	// Create our starting cards, one for each id
	const cards = Array.from({ length: input.length }).map(() => 1)

	for (const [i, [winning, myCard]] of input.entries()) {
		const winningNumbers = setIntersection(winning, myCard).size

		// If the winning numbers is 4, and we are on card 2
		// add one to card 3, 4, 5, 6
		for (let x = i + 1; x <= winningNumbers + i; x++) {
			cards[x] += cards[i]
		}
	}

	return cards.reduce((acc, card) => acc + card, 0)
}

export async function day4() {
	const input = await readInput(4)
	const data = parseInput(input)

	console.time('part1')
	console.log(`Day 4 / Part 1: ${part1(data)}`)
	console.timeEnd('part1')

	console.time('part2')
	console.log(`Day 4 / Part 2: ${part2(data)}`)
	console.timeEnd('part2')
}
