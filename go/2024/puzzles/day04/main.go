package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils/files"
)

// Changed part 1 to take a similar approach to what I did in part 2
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	xmas := 0

	rows := len(lines)
	cols := len(lines[0])

	for r := range rows {
		for c := range cols {
			// Skip if not an X
			if string(lines[r][c]) != "X" {
				continue
			}

			// Check all directions
			for _, dr := range []int{-1, 0, 1} {
				for _, dc := range []int{-1, 0, 1} {
					if dr == 0 && dc == 0 {
						continue
					}

					// Check bounds
					threeDown := r + 3*dr
					threeForwards := c + 3*dc
					hasSpaceDown := threeDown >= 0 && threeDown < rows
					hasSpaceForwards := threeForwards >= 0 && threeForwards < cols

					if !hasSpaceDown || !hasSpaceForwards {
						continue
					}

					// Check that the next three letters are MAS
					nextIsM := string(lines[r+dr][c+dc]) == "M"
					nextIsA := string(lines[r+2*dr][c+2*dc]) == "A"
					nextIsS := string(lines[r+3*dr][c+3*dc]) == "S"

					if nextIsM && nextIsA && nextIsS {
						xmas++
					}
				}
			}
		}
	}

	return xmas
}

func part2(name string) int {
	lines := files.ReadLines(name)
	xmas := 0

	rows := len(lines)
	cols := len(lines[0])

	// Skip checking edge characters since any A here can't result in a X-MAS
	for r := range rows - 1 {
		for c := range cols - 1 {
			// A's are always in the middle
			if string(lines[r][c]) != "A" {
				continue
			}

			// Check line above and below
			if r-1 >= 0 && c-1 >= 0 && r+1 < rows && c+1 < cols {
				diagonalTopLeft := string(lines[r-1][c-1])
				diagonalTopRight := string(lines[r-1][c+1])
				diagonalBottomLeft := string(lines[r+1][c-1])
				diagonalBottomRight := string(lines[r+1][c+1])

				// M.M
				// .A.
				// S.S
				if diagonalTopLeft == "M" && diagonalBottomRight == "S" && diagonalTopRight == "M" && diagonalBottomLeft == "S" {
					xmas++
				}

				// M.S
				// .A.
				// M.S
				if diagonalTopLeft == "M" && diagonalBottomRight == "S" && diagonalTopRight == "S" && diagonalBottomLeft == "M" {
					xmas++
				}

				// S.S
				// .A.
				// M.M
				if diagonalTopLeft == "S" && diagonalBottomRight == "M" && diagonalTopRight == "S" && diagonalBottomLeft == "M" {
					xmas++
				}

				// S.M
				// .A.
				// S.M
				if diagonalTopLeft == "S" && diagonalBottomRight == "M" && diagonalTopRight == "M" && diagonalBottomLeft == "S" {
					xmas++
				}
			}
		}
	}

	return xmas
}
