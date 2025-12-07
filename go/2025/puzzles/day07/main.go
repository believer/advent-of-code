package main

import (
	"fmt"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/grid"
)

// I spent too much time trying to make a flawed solution work.
// I thought of BFS (I've had it in the back of my mind that
// it would appear at some point) but kept working on the path I had taken ':D
// Eventually, I gave up and looked at my solution from day 18 2024 to remember.

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) (splits int) {
	lines := files.ReadLines(name)
	manifold := grid.New(lines)
	start, _ := manifold.Find('S')

	queue := []grid.Point{start}
	seen := map[grid.Point]bool{}

	for len(queue) > 0 {
		beam := queue[0]
		queue = queue[1:]

		// Keep track of what we've seen since beams
		// can split into the same position
		if _, ok := seen[beam]; ok {
			continue
		}

		if !manifold.InBounds(beam) {
			continue
		}

		seen[beam] = true

		if manifold.Get(beam) == '^' {
			splits += 1
			leftSplit := beam.Add(grid.LEFT)
			rightSplit := beam.Add(grid.RIGHT)

			queue = append(queue, leftSplit)
			queue = append(queue, rightSplit)
		} else {
			queue = append(queue, beam.Add(grid.DOWN))
		}
	}

	return
}

// Another thing I've had in mind would show up is Dynamic Programming (DP) which
// I first learnt about in 2023 day 12. Essentially, it's taking a problem and breaking
// it down into smaller subproblems and recursively solving them.
//
// I solved it without a seen cache first, which worked for the example, but it was
// immediately clear that didn't work for the real input. A simple change and it worked.
func part2(name string) int {
	lines := files.ReadLines(name)
	manifold := grid.New(lines)
	start, _ := manifold.Find('S')
	seen := map[grid.Point]int{}

	return timelines(manifold, start, seen)
}

func timelines(g grid.Grid, point grid.Point, seen map[grid.Point]int) int {
	if v, ok := seen[point]; ok {
		return v
	}

	if !g.InBounds(point) {
		return 1
	}

	if g.Get(point) == '^' {
		leftSplit := point.Add(grid.LEFT)
		rightSplit := point.Add(grid.RIGHT)

		return timelines(g, leftSplit, seen) + timelines(g, rightSplit, seen)
	}

	down := point.Add(grid.DOWN)
	result := timelines(g, down, seen)
	seen[down] = result

	return result
}
