package main

import (
	"fmt"
	"math"
	"slices"
	"strings"

	"github.com/believer/aoc-2025/utils"
	"github.com/believer/aoc-2025/utils/files"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	banks := files.ReadLines(name)
	joltageOutput := 0

	for _, b := range banks {
		batteries := strings.Split(b, "")
		largest := 0
		largestIndex := -1
		nextLargest := 0

		for i, battery := range batteries {
			b := utils.MustIntFromString(battery)

			if b > largest && i != len(batteries)-1 {
				largest = b
				largestIndex = i
			}
		}

		for _, battery := range batteries[largestIndex+1:] {
			b := utils.MustIntFromString(battery)

			if b > nextLargest {
				nextLargest = b
			}
		}

		joltageOutput += largest*10 + nextLargest
	}

	return joltageOutput
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
