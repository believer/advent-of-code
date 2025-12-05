package main

import (
	"fmt"
	"strings"

	"github.com/believer/aoc-2025/utils"
	"github.com/believer/aoc-2025/utils/files"
	"github.com/believer/aoc-2025/utils/math"
)

// I had a small refactor to clean up the loops, but it was
// slower than my initial solution so I kept it as is.

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	dial := 50
	count := 0

	for _, turn := range turns(name) {
		dial = (dial + turn) % 100

		if dial == 0 {
			count += 1
		}
	}

	return count
}

// Learnt this neat trick that we can calculate how many
// full turns in either direction we will make. One full
// turn, of course, results in us passing 0.
func part2(name string) int {
	dial := 50
	count := 0

	for _, turn := range turns(name) {
		if turn < 0 {
			div, mod := math.Divmod(turn, -100)
			count += div

			// Check if the remainder would pass zero
			if dial != 0 && dial+mod <= 0 {
				count += 1
			}
		} else {
			div, mod := math.Divmod(turn, 100)
			count += div

			if dial+mod >= 100 {
				count += 1
			}
		}

		dial = (dial + turn) % 100
	}

	return count
}

func turns(name string) (values []int) {
	lines := files.ReadLines(name)

	for _, l := range lines {
		l = strings.Replace(l, "L", "-", 1)
		v := utils.MustIntFromString(strings.Replace(l, "R", "", 1))
		values = append(values, v)
	}

	return
}
