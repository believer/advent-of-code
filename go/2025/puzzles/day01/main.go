package main

import (
	"fmt"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	dial := 50
	count := 0

	for _, turn := range turns(name) {
		dial = wrap(dial + turn)

		if dial == 0 {
			count++
		}
	}

	return count
}

// Every full turn of 100 passes 0 once. Moving right, that's simply
// how many hundreds we pass. Moving left, we can count the distance
// to the next 0 as if we were moving right from the mirrored position.
func part2(name string) int {
	dial := 50
	count := 0

	for _, turn := range turns(name) {
		if turn >= 0 {
			count += (dial + turn) / 100
		} else {
			count += (wrap(-dial) - turn) / 100
		}

		dial = wrap(dial + turn)
	}

	return count
}

// Go's % keeps the sign of the dividend, so -18 % 100 is -18, not 82
func wrap(n int) int {
	return (n%100 + 100) % 100
}

func turns(name string) []int {
	lines := files.ReadLines(name)
	values := make([]int, 0, len(lines))

	for _, l := range lines {
		v := utils.MustIntFromString(l[1:])

		if l[0] == 'L' {
			v = -v
		}

		values = append(values, v)
	}

	return values
}
