package main

import (
	"fmt"
	"math"
	"slices"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

// Had a solution for part 1 that worked well and fast
// But, the changed solution for part is much cleaner and
// performance is the same so a much better overall.

func part1(name string) int {
	banks := createBanks(name)
	joltage := 0

	for _, batteries := range banks {
		joltage += jolt(2, batteries)
	}

	return joltage
}

func part2(name string) int {
	banks := createBanks(name)
	joltage := 0

	for _, batteries := range banks {
		joltage += jolt(12, batteries)
	}

	return joltage
}

// Convert the lines to lists integers to do math
// on them in following steps
func createBanks(name string) [][]int {
	lines := files.ReadLines(name)
	banks := make([][]int, len(lines))

	for i, l := range lines {
		batteries := make([]int, len(l))

		for j, battery := range strings.Split(l, "") {
			batteries[j] = utils.MustIntFromString(battery)
		}

		banks[i] = batteries
	}

	return banks
}

func jolt(n int, b []int) int {
	// Base case: Return max if only one digit
	if n == 1 {
		return slices.Max(b)
	}

	// Reduce digits and save largest index for next recursion
	n = n - 1
	maxDigit := slices.Max(b[:len(b)-n])
	index := slices.Index(b, maxDigit) + 1

	// Multiply the max digit with the n ** 10 to get the correct value
	// for the current position. Then, recurse with a shortened list of
	// remaining batteries.
	return maxDigit*int(math.Pow(10, float64(n))) + jolt(n, b[index:])
}
