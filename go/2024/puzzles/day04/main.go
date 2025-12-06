package main

import (
	"fmt"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/grid"
)

// Changed part 1 to take a similar approach to what I did in part 2
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	xmasGrid := grid.New(lines)
	xmas := 0

	for y := range xmasGrid.Height {
		for x := range xmasGrid.Width {
			point := grid.Point{X: x, Y: y}

			// Skip if not an X
			if xmasGrid.Get(point) != 'X' {
				continue
			}

			// Check all directions
			for _, d := range grid.ALL_DIRECTIONS {
				dx, dy := d.X, d.Y

				// Do we have enough items to check?
				offByThree := grid.Point{X: x + 3*dx, Y: y + 3*dy}

				if _, ok := xmasGrid.TryGet(offByThree); !ok {
					continue
				}

				offByOne := grid.Point{X: x + dx, Y: y + dy}
				offByTwo := grid.Point{X: x + 2*dx, Y: y + 2*dy}

				// Check that the next three letters are MAS
				isM := xmasGrid.Get(offByOne) == 'M'
				isA := xmasGrid.Get(offByTwo) == 'A'
				isS := xmasGrid.Get(offByThree) == 'S'

				if isM && isA && isS {
					xmas++
				}
			}
		}
	}

	return xmas
}

func part2(name string) int {
	lines := files.ReadLines(name)
	xmasGrid := grid.New(lines)
	xmas := 0

	// Skip checking edge characters since any A here can't result in a X-MAS
	for y := range xmasGrid.Height - 1 {
		for x := range xmasGrid.Width - 1 {
			point := grid.Point{X: x, Y: y}

			// A's are always in the middle
			if xmasGrid.Get(point) != 'A' {
				continue
			}

			if _, ok := xmasGrid.TryGet(point.Add(grid.TOPLEFT)); !ok {
				continue
			}

			if _, ok := xmasGrid.TryGet(point.Add(grid.BOTTOMRIGHT)); !ok {
				continue
			}

			// Check line above and below
			diagonalTopLeft := xmasGrid.Get(point.Add(grid.TOPLEFT))
			diagonalTopRight := xmasGrid.Get(point.Add(grid.TOPRIGHT))
			diagonalBottomLeft := xmasGrid.Get(point.Add(grid.BOTTOMLEFT))
			diagonalBottomRight := xmasGrid.Get(point.Add(grid.BOTTOMRIGHT))

			// M.M
			// .A.
			// S.S
			if diagonalTopLeft == 'M' && diagonalBottomRight == 'S' && diagonalTopRight == 'M' && diagonalBottomLeft == 'S' {
				xmas++
			}

			// M.S
			// .A.
			// M.S
			if diagonalTopLeft == 'M' && diagonalBottomRight == 'S' && diagonalTopRight == 'S' && diagonalBottomLeft == 'M' {
				xmas++
			}

			// S.S
			// .A.
			// M.M
			if diagonalTopLeft == 'S' && diagonalBottomRight == 'M' && diagonalTopRight == 'S' && diagonalBottomLeft == 'M' {
				xmas++
			}

			// S.M
			// .A.
			// S.M
			if diagonalTopLeft == 'S' && diagonalBottomRight == 'M' && diagonalTopRight == 'M' && diagonalBottomLeft == 'S' {
				xmas++
			}
		}
	}

	return xmas
}
