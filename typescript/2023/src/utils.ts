import path from 'node:path'
import { fileURLToPath } from 'node:url'

export function numbersToSet(input: string) {
	return new Set(
		input
			.trim()
			.split(' ')
			.map((line) => parseInt(line.trim()))
			.filter(Boolean)
	)
}

export function setIntersection(set1: Set<number>, set2: Set<number>) {
	return new Set([...set1].filter((element) => set2.has(element)))
}

export function readInput(day: number) {
	const __filename = fileURLToPath(import.meta.url)
	const __dirname = path.dirname(__filename)

	return Bun.file(path.join(__dirname, `day${day}`, 'input.txt')).text()
}
