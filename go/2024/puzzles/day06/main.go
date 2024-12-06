package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils/files"
)

// Brute forced part 2 first by checking every position on the grid.
// It worked, but was slow. We can instead check only the path that
// the guard took during the first run which was more that 78% faster.
// Still over a second, but a lot better.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

type Position struct {
	r, c int
}

type PositionWithDirection struct {
	r, c, dr, dc int
}

func part1(name string) int {
	lines := files.ReadLines(name)
	guardLocation := initialGuardLocation(lines)

	return len(getPath(lines, guardLocation))
}

func part2(name string) int {
	lines := files.ReadLines(name)
	possibleLoops := 0
	guardLocation := initialGuardLocation(lines)

	// Create a grid to modify
	var grid [][]byte

	for _, l := range lines {
		grid = append(grid, []byte(l))
	}

	guardPath := getPath(lines, guardLocation)

	for p := range guardPath {
		if lines[p.r][p.c] != '.' {
			continue
		}

		grid[p.r][p.c] = '#'

		if isLoop(grid, guardLocation.r, guardLocation.c) {
			possibleLoops++
		}

		grid[p.r][p.c] = '.'
	}

	return possibleLoops
}

func initialGuardLocation(lines []string) Position {
	guardLocation := Position{0, 0}
	rows := len(lines)
	cols := len(lines[0])

outer:
	for r := range rows {
		for c := range cols {
			if lines[r][c] == '^' {
				guardLocation = Position{r, c}
				break outer
			}
		}
	}

	return guardLocation
}

func getPath(lines []string, guard Position) map[Position]struct{} {
	visitedLocations := make(map[Position]struct{})
	rows := len(lines)
	cols := len(lines[0])
	dr := -1
	dc := 0

	for {
		r, c := guard.r, guard.c
		visitedLocations[guard] = struct{}{}

		// Check bounds
		if r+dr < 0 || r+dr >= rows || c+dc < 0 || c+dc >= cols {
			break
		}

		// We always rotate to the right on obstacles
		// (-1,0) becomes (0,1) becomes (1,0) becomes (0,-1)
		if lines[r+dr][c+dc] == '#' {
			dc, dr = -dr, dc
		} else {
			guard = Position{r: r + dr, c: c + dc}
		}
	}

	return visitedLocations
}

func isLoop(grid [][]byte, r, c int) bool {
	visitedLocations := make(map[PositionWithDirection]struct{})
	dr := -1
	dc := 0
	rows := len(grid)
	cols := len(grid[0])

	for {
		// Save with direction as well to find when we're looping
		visitedLocations[PositionWithDirection{r, c, dr, dc}] = struct{}{}

		// Check bounds
		if r+dr < 0 || r+dr >= rows || c+dc < 0 || c+dc >= cols {
			return false
		}

		// We always rotate to the right on obstacles
		// (-1,0) becomes (0,1) becomes (1,0) becomes (0,-1)
		if grid[r+dr][c+dc] == '#' {
			dc, dr = -dr, dc
		} else {
			r += dr
			c += dc
		}

		if _, ok := visitedLocations[PositionWithDirection{r, c, dr, dc}]; ok {
			return true
		}
	}
}
