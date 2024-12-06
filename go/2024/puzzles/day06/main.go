package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils/files"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

type Position struct {
	r, c int
}

func part1(name string) int {
	lines := files.ReadLines(name)
	visitedLocations := make(map[Position]struct{})

	rows := len(lines)
	cols := len(lines[0])
	guardLocation := Position{0, 0}

	// Find initial guard location
outer:
	for r := range rows {
		for c := range cols {
			if string(lines[r][c]) == "^" {
				guardLocation = Position{r, c}
				break outer
			}
		}
	}

	dr := -1
	dc := 0

	for {
		r, c := guardLocation.r, guardLocation.c
		visitedLocations[guardLocation] = struct{}{}

		// Check bounds
		if r+dr < 0 || r+dr >= rows || c+dc < 0 || c+dc >= cols {
			break
		}

		// We always rotate to the right on obstacles
		// (-1,0) becomes (0,1) becomes (1,0) becomes (0,-1)
		if string(lines[r+dr][c+dc]) == "#" {
			dc, dr = -dr, dc
		} else {
			guardLocation = Position{r: r + dr, c: c + dc}
		}
	}

	return len(visitedLocations)
}

func part2(name string) int {
	return 0
}
