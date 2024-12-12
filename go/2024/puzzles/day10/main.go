package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils/files"
	"github.com/believer/aoc-2024/utils/grid"
)

// BFS
// Funniest day two, just had to remove the check that we've visited a spot before

// Backtracked and implemented the solution using the Grid/Point helpers
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	trailMap := grid.New(lines)
	trailheads := trailMap.FindAll('0')
	trailScoreTotal := 0

	for _, head := range trailheads {
		trailScoreTotal += findTrailScore(trailMap, head, true)
	}

	return trailScoreTotal
}

func part2(name string) int {
	lines := files.ReadLines(name)
	trailMap := grid.New(lines)
	trailheads := trailMap.FindAll('0')
	trailScoreTotal := 0

	for _, head := range trailheads {
		trailScoreTotal += findTrailScore(trailMap, head, false)
	}

	return trailScoreTotal
}

func findTrailScore(trailMap grid.Grid, start grid.Point, isDistinct bool) int {
	visited := map[grid.Point]bool{}
	queue := []grid.Point{start}
	trailScore := 0

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		// We've seen this before
		if _, ok := visited[current]; ok && isDistinct {
			continue
		}

		visited[current] = true

		// Reached a top
		if trailMap.Get(current) == '9' {
			trailScore++
			continue
		}

		for _, direction := range grid.CARDINALS {
			next := current.Add(direction)

			// Position exists and is _one_ bigger than current
			if value, ok := trailMap.Contains(next); ok && value == trailMap.Get(current)+1 {
				queue = append(queue, next)
			}
		}
	}

	return trailScore
}
