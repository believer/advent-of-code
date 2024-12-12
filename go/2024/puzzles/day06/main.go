package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils/files"
	"github.com/believer/aoc-2024/utils/grid"
)

// Brute forced part 2 first by checking every position on the grid.
// It worked, but was slow. We can instead check only the path that
// the guard took during the first run which was more that 78% faster.
// Still over a second, but a lot better. Could most likely be
// parallelized, but I think it's good enough.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

type PointWithDirection struct {
	x, y, dx, dy int
}

func part1(name string) int {
	lines := files.ReadLines(name)
	guardMap := grid.New(lines)
	guardLocation := guardMap.Find('^')

	return len(getPath(guardMap, guardLocation))
}

func part2(name string) int {
	lines := files.ReadLines(name)
	guardMap := grid.New(lines)
	guardLocation := guardMap.Find('^')
	possibleLoops := 0

	guardPath := getPath(guardMap, guardLocation)

	for p := range guardPath {
		if guardMap.Get(p) != '.' {
			continue
		}

		guardMap.Update(p, '#')

		if isLoop(guardMap, guardLocation) {
			possibleLoops++
		}

		guardMap.Update(p, '.')
	}

	return possibleLoops
}

func getPath(guardMap grid.Grid, guard grid.Point) map[grid.Point]bool {
	visitedLocations := map[grid.Point]bool{}
	dx := 0
	dy := -1

	for {
		x, y := guard.X, guard.Y
		visitedLocations[guard] = true
		next := grid.Point{X: x + dx, Y: y + dy}

		// Check bounds
		if _, ok := guardMap.Contains(next); !ok {
			break
		}

		// We always rotate to the right on obstacles
		// (-1,0) becomes (0,1) becomes (1,0) becomes (0,-1)
		if guardMap.Get(next) == '#' {
			dx, dy = -dy, dx
		} else {
			guard = next
		}
	}

	return visitedLocations
}

func isLoop(guardMap grid.Grid, position grid.Point) bool {
	visitedLocations := map[PointWithDirection]bool{}
	x, y := position.X, position.Y
	dx := 0
	dy := -1

	for {
		// Save with direction as well to find when we're looping
		visitedLocations[PointWithDirection{x, y, dx, dy}] = true
		next := grid.Point{X: x + dx, Y: y + dy}

		// Check bounds
		if _, ok := guardMap.Contains(next); !ok {
			return false
		}

		// We always rotate to the right on obstacles
		// (-1,0) becomes (0,1) becomes (1,0) becomes (0,-1)
		if guardMap.Get(next) == '#' {
			dx, dy = -dy, dx
		} else {
			x += dx
			y += dy
		}

		if _, ok := visitedLocations[PointWithDirection{x, y, dx, dy}]; ok {
			return true
		}
	}
}
