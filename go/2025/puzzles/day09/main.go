package main

import (
	"fmt"
	"math"
	"slices"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/grid"
	"github.com/believer/aoc-utils/utils"
)

// My main issue was how to check if the rectangle was within the polygon.
// Then I found the line in the problem that says adjacent coordinates are
// on the same column or row, which made boundary calculations easier.

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) (largestArea int) {
	lines := files.ReadLines(name)
	coords := createCoordinates(lines)

	for i := range len(coords) {
		p1 := coords[i]

		for j := i + 1; j < len(coords); j++ {
			p2 := coords[j]

			// Add one to include the red corner tiles
			dx := math.Abs(float64(p2.X)-float64(p1.X)) + 1
			dy := math.Abs(float64(p2.Y)-float64(p1.Y)) + 1

			area := dx * dy

			if area > float64(largestArea) {
				largestArea = int(area)
			}
		}
	}

	return
}

type Rectangle struct {
	P1, P2 grid.Point
	Area   int
}

func part2(name string) (largestArea int) {
	lines := files.ReadLines(name)
	coords := createCoordinates(lines)
	boundaries := createBoundaries(coords)

	// Create all rectangles first. Might be bad for
	// performance, but I like how it looks
	rectangles := []Rectangle{}

	for i := range len(coords) {
		p1 := coords[i]

		for j := i + 1; j < len(coords); j++ {
			p2 := coords[j]

			// Add one to include the red corner tiles
			dx := math.Abs(float64(p2.X)-float64(p1.X)) + 1
			dy := math.Abs(float64(p2.Y)-float64(p1.Y)) + 1

			area := int(dx * dy)
			rectangles = append(rectangles, Rectangle{P1: p1, P2: p2, Area: area})
		}
	}

	// Sort to check from largest and down
	slices.SortFunc(rectangles, func(a, b Rectangle) int {
		return b.Area - a.Area
	})

	// Find the first valid
	for _, r := range rectangles {
		if isRectangleValid(r, boundaries) {
			largestArea = r.Area
			break
		}
	}

	return
}

func createCoordinates(lines []string) (coords []grid.Point) {
	for _, l := range lines {
		x, y, _ := strings.Cut(l, ",")

		coords = append(coords, grid.Point{
			X: utils.MustIntFromString(x),
			Y: utils.MustIntFromString(y),
		})
	}

	return
}

// This is slow because of, potentially, a lot of checks
func isRectangleValid(r Rectangle, boundaries map[int][2]int) bool {
	minX := min(r.P1.X, r.P2.X)
	maxX := max(r.P1.X, r.P2.X)
	minY := min(r.P1.Y, r.P2.Y)
	maxY := max(r.P1.Y, r.P2.Y)

	for y := minY; y <= maxY; y++ {
		bounds, ok := boundaries[y]

		if !ok {
			return false
		}

		if minX < bounds[0] || maxX > bounds[1] {
			return false
		}
	}

	return true
}

// Calculate boundaries
// The problem states that "Tiles that are adjacent in
// your list will always be on either the same row or the same column".
// So, we can find vertical and horizontal edges

// row ->  [min_x, max_x]
func createBoundaries(coords []grid.Point) map[int][2]int {
	boundaries := map[int][2]int{}

	for i, c := range coords {
		p1 := c
		p2 := coords[(i+1)%len(coords)] // List wraps around

		x1, y1 := p1.X, p1.Y
		x2, y2 := p2.X, p2.Y

		if x1 == x2 {
			// Same x, create vertical edge from min y to max y
			minY := min(y1, y2)
			maxY := max(y1, y2) + 1

			for y := minY; y < maxY; y++ {
				if _, ok := boundaries[y]; !ok {
					boundaries[y] = [2]int{x1, x1}
				} else {
					cx1 := boundaries[y][0]
					cx2 := boundaries[y][1]

					minX := min(cx1, x1)
					maxX := max(cx2, x1)

					boundaries[y] = [2]int{minX, maxX}
				}
			}
		} else if y1 == y2 {
			// same y, create horizontal edge from mix to max x

			if _, ok := boundaries[y1]; !ok {
				boundaries[y1] = [2]int{min(x1, x2), max(x1, x2)}
			} else {
				cx1 := boundaries[y1][0]
				cx2 := boundaries[y1][1]

				minX := min(min(cx1, x1), x2)
				maxX := max(max(cx2, x1), x2)

				boundaries[y1] = [2]int{minX, maxX}
			}
		}
	}

	return boundaries
}

// math.Min / math.Max use float64s so these helpers
// remove any need for casting
func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
