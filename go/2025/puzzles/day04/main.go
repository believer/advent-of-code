package main

import (
	"fmt"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/grid"
)

// Love grid problems! My grid utility library saves the day again.
// Refactored to make it cleaner, no performance loss

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	department := grid.New(lines)

	return len(findAccessible(department))
}

func part2(name string) (removable int) {
	lines := files.ReadLines(name)
	department := grid.New(lines)

	for {
		accessible := findAccessible(department)

		// No more paper to remove
		if len(accessible) == 0 {
			break
		}

		// Remove the paper
		for _, p := range accessible {
			department.Update(p, '.')
		}

		removable += len(accessible)
	}

	return
}

func findAccessible(department grid.Grid) (accessible []grid.Point) {
	for y := range department.Height {
		for x := range department.Width {
			rolls := 0
			p := grid.Point{X: x, Y: y}

			// No paper here
			if department.Get(p) != '@' {
				continue
			}

			// Check all directions and increment if it has paper in it
			for _, dir := range grid.ALL_DIRECTIONS {
				check := p.Add(dir)

				// NOTE: I tried using TryGet to get bounds check and value
				// in one line, but it was significantly slower (almost twice as slow).
				// Even doing the two checks we have here in one if is slower.
				// From what I can gather, if short-circuiting and compiler
				// optimizations (like inlining) might be the reason that the
				// current order is the fastest.
				if !department.InBounds(check) {
					continue
				}

				if department.Get(check) == '@' {
					rolls += 1
				}
			}

			// Less than four means the paper is forklift accessible
			if rolls < 4 {
				accessible = append(accessible, p)
			}
		}
	}

	return
}
