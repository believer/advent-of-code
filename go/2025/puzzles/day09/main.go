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

// Edit: Found a faster solution using slice bounds instead of a map

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
	bounds, minY := createBoundariesSlice(coords)

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
		if isRectangleValid(r, bounds, minY) {
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
func isRectangleValid(r Rectangle, boundaries []Bounds, minY int) bool {
	minX := min(r.P1.X, r.P2.X)
	maxX := max(r.P1.X, r.P2.X)
	loY := min(r.P1.Y, r.P2.Y)
	hiY := max(r.P1.Y, r.P2.Y)

	for y := loY; y <= hiY; y++ {
		index := y - minY

		if index < 0 || index >= len(boundaries) {
			return false
		}

		b := boundaries[index]

		if !b.Ok || minX < b.Min || maxX > b.Max {
			return false
		}
	}

	return true
}

// Calculate boundaries
// The problem states that "Tiles that are adjacent in
// your list will always be on either the same row or the same column".
// So, we can find vertical and horizontal edges

type Bounds struct {
	Min, Max int
	Ok       bool
}

// First solution had a map of minX, maxX. But, this was slow when checking
// for valid rectangles. This returns a slice and minY for faster lookup with y-minY.
func createBoundariesSlice(coords []grid.Point) ([]Bounds, int) {
	if len(coords) == 0 {
		return nil, 0
	}

	// Find global Y-range
	minY := coords[0].Y
	maxY := coords[0].Y

	for _, c := range coords {
		minY = min(c.Y, minY)
		maxY = max(c.Y, maxY)
	}

	// Allocate bounds
	size := maxY - minY + 1
	bounds := make([]Bounds, size)

	// Build similar to your map version
	for i := range coords {
		p1 := coords[i]
		p2 := coords[(i+1)%len(coords)]

		x1, y1 := p1.X, p1.Y
		x2, y2 := p2.X, p2.Y

		if x1 == x2 {
			// Same x, create vertical edge from min y to max y
			lo := min(y1, y2)
			hi := max(y1, y2)

			for y := lo; y <= hi; y++ {
				idx := y - minY

				if !bounds[idx].Ok {
					bounds[idx] = Bounds{Min: x1, Max: x1, Ok: true}
				} else {
					b := bounds[idx]
					b.Min = min(x1, b.Min)
					b.Max = max(x1, b.Max)
					bounds[idx] = b
				}
			}
		} else if y1 == y2 {
			// same y, create horizontal edge from mix to max x
			row := y1 - minY

			if !bounds[row].Ok {
				bounds[row] = Bounds{Min: min(x1, x2), Max: max(x1, x2), Ok: true}
			} else {
				b := bounds[row]
				b.Min = min(min(x1, x2), b.Min)
				b.Max = max(max(x1, x2), b.Max)
				bounds[row] = b
			}
		}
	}

	return bounds, minY
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
