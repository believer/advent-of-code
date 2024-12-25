package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils/files"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadParagraphs(name)
	locks := [][]int{}
	keys := [][]int{}

	for _, l := range lines {
		lockOrKey := make([]int, 5)

		// Locks with first row as #
		// Keys with last row as #
		if l[0][0] == '#' {
			for _, r := range l[1:] {
				for i, c := range r {
					if c == '#' {
						lockOrKey[i]++
					}
				}
			}

			locks = append(locks, lockOrKey)
		} else {
			for _, r := range l[:len(l)-1] {
				for i, c := range r {
					if c == '#' {
						lockOrKey[i]++
					}
				}
			}

			keys = append(keys, lockOrKey)
		}
	}

	matchingKeys := 0

	for _, lock := range locks {
		for _, key := range keys {
			c1 := lock[0]+key[0] <= 5
			c2 := lock[1]+key[1] <= 5
			c3 := lock[2]+key[2] <= 5
			c4 := lock[3]+key[3] <= 5
			c5 := lock[4]+key[4] <= 5

			// There are no overlaps if all column combinations
			// are less than or equal to five
			if c1 && c2 && c3 && c4 && c5 {
				matchingKeys++
			}
		}
	}

	return matchingKeys
}

func part2(name string) int {
	return 0
}
