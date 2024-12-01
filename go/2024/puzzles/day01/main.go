package main

import (
	"fmt"
	"slices"
	"strings"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

// Part 1 couldn't be improved much in terms of performance.

// For part 2 we could use a hashmap to find how many times
// each number appears. This makes the lookup of for each
// item in the the left list O(1) instead of having to loop
// through the entire right list for each number.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	left, right := historicallySignificantLocations(name)

	slices.Sort(left)
	slices.Sort(right)

	total := 0

	for i, l := range left {
		total += utils.Abs(l - right[i])
	}

	return total
}

func part2(name string) int {
	left, right := historicallySignificantLocations(name)

	total := 0
	appears := map[int]int{}

	for _, r := range right {
		_, ok := appears[r]

		if ok {
			appears[r] += 1
		} else {
			appears[r] = 1
		}
	}

	for _, l := range left {
		v, ok := appears[l]

		if ok {
			total += l * v
		}
	}

	return total
}

func historicallySignificantLocations(name string) ([]int, []int) {
	lines := files.ReadLines(name)
	left := make([]int, len(lines))
	right := make([]int, len(lines))

	for _, line := range lines {
		// Split string by any number of whitespace
		row := strings.Fields(line)

		l := utils.MustIntFromString(row[0])
		r := utils.MustIntFromString(row[1])

		left = append(left, l)
		right = append(right, r)
	}

	return left, right
}
