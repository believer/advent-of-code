package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) (total int) {
	buyers := files.ReadLines(name)

	for _, buyer := range buyers {
		secret := utils.MustIntFromString(buyer)

		for range 2000 {
			secret = generateSecret(secret)
		}

		total += secret
	}

	return total
}

type Pattern [4]int

func part2(name string) (maxBananas int) {
	buyers := files.ReadLines(name)
	scores := map[Pattern]int{}

	for _, buyer := range buyers {
		secret := utils.MustIntFromString(buyer)
		patterns := [][2]int{}
		seen := map[Pattern]bool{}
		last := secret % 10

		// Generate all secrets and their deltas
		for range 2000 {
			secret = generateSecret(secret)
			temp := secret % 10
			patterns = append(patterns, [2]int{temp - last, temp})
			last = temp
		}

		// Go through the patterns four by four and add them to the total score
		for i := range len(patterns) - 4 {
			key := Pattern{
				patterns[i][0],
				patterns[i+1][0],
				patterns[i+2][0],
				patterns[i+3][0],
			}

			if _, ok := seen[key]; !ok {
				seen[key] = true
				scores[key] += patterns[i+3][1]
			}
		}
	}

	for _, b := range scores {
		maxBananas = max(maxBananas, b)
	}

	return maxBananas
}

func generateSecret(secret int) int {
	prune := 16777216

	// Step 1
	result := secret * 64
	secret ^= result
	secret %= prune

	// Step 2
	result = secret / 32
	secret ^= result
	secret %= prune

	// Step 3
	result = secret * 2048
	secret ^= result
	secret %= prune

	return secret
}
