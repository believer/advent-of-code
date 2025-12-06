package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

// Part 1 worked fine with a simple array, but part 2 got way out of hand.
// Should've seen this coming when it was a "do this X number of times" requirement.
// Changed it to a hashmap to store how many times we've seen each number and it worked fine.
// TODO: Add benchmarks from M1 computer to have all from same
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	return calculateNumberOfStones(name, 25)
}

func part2(name string) int {
	return calculateNumberOfStones(name, 75)
}

func calculateNumberOfStones(name string, iterations int) int {
	line := files.Read(name)
	stones := map[int]int{}

	for _, stone := range strings.Split(strings.TrimSpace(line), " ") {
		stones[utils.MustIntFromString(stone)] = 1
	}

	for range iterations {
		stones = blink(stones)
	}

	numberOfStones := 0

	for _, count := range stones {
		numberOfStones += count
	}

	return numberOfStones
}

func blink(stones map[int]int) map[int]int {
	temporaryStones := map[int]int{}

	add := func(key, count int) {
		if _, ok := temporaryStones[key]; !ok {
			temporaryStones[key] = 0
		}
		temporaryStones[key] += count
	}

	for stone, count := range stones {
		// Rule 1: Stone 0 -> 1
		if stone == 0 {
			add(1, count)
			continue
		}

		// Rule 2: Stone of even digits -> two stones with digits split in middle
		// 2024 -> 20 24
		stoneAsString := strconv.Itoa(stone)
		stoneLength := len(stoneAsString)

		if stoneLength%2 == 0 {
			left := utils.MustIntFromString(stoneAsString[:stoneLength/2])
			right := utils.MustIntFromString(stoneAsString[stoneLength/2:])

			add(left, count)
			add(right, count)
			continue
		}

		// Catch-all rule: Stone -> Stone multiplied by 2024
		add(stone*2024, count)
	}

	return temporaryStones
}
