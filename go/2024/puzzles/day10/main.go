package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

// BFS
// Funniest day two, just had to remove the check that we've visited a spot before
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

type Location struct {
	r, c int
}

func part1(name string) int {
	lines := files.ReadLines(name)
	trailMap, trailheads := createTrailMap(lines)
	trailScoreTotal := 0

	for _, head := range trailheads {
		trailScoreTotal += findTrailScore(trailMap, head, true)
	}

	return trailScoreTotal
}

func part2(name string) int {
	lines := files.ReadLines(name)
	trailMap, trailheads := createTrailMap(lines)
	trailScoreTotal := 0

	for _, head := range trailheads {
		trailScoreTotal += findTrailScore(trailMap, head, false)
	}

	return trailScoreTotal
}

func createTrailMap(lines []string) (map[Location]int, []Location) {
	trailheads := []Location{}
	trailMap := map[Location]int{}

	rows := len(lines)
	cols := len(lines[0])

	for r := range rows {
		for c := range cols {
			value := lines[r][c]

			if value == '0' {
				trailheads = append(trailheads, Location{r, c})
			}

			trailMap[Location{r, c}] = utils.MustIntFromString(string(value))
		}
	}

	return trailMap, trailheads
}

func findTrailScore(trailMap map[Location]int, start Location, isDistinct bool) int {
	visited := map[Location]bool{}
	queue := []Location{start}
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
		if trailMap[current] == 9 {
			trailScore++
			continue
		}

		moves := []Location{}
		directions := []Location{
			{r: current.r - 1, c: current.c}, // Above
			{r: current.r, c: current.c + 1}, // Right
			{r: current.r + 1, c: current.c}, // Below
			{r: current.r, c: current.c - 1}, // Left
		}

		for _, direction := range directions {
			// Position exists and is _one_ bigger than current
			if value, ok := trailMap[direction]; ok && value == trailMap[current]+1 {
				moves = append(moves, direction)
			}
		}

		queue = append(queue, moves...)
	}

	return trailScore
}
