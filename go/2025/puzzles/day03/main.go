package main

import (
	"fmt"
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
	return 0
}
