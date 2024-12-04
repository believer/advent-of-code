package main

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/believer/aoc-2024/utils/files"
)

// Can be cleaned up a bunch, but it works!
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	xmas := 0
	forwards := regexp.MustCompile(`XMAS`)
	backwards := regexp.MustCompile(`SAMX`)

	// Check horizontal, forwards and backwards
	for _, l := range lines {
		t := len(forwards.FindAllString(l, -1))
		t2 := len(backwards.FindAllString(l, -1))

		xmas += t + t2
	}

	rows := len(lines)
	cols := len(lines[0])

	// Check vertical
	for col := 0; col < cols; col++ {
		vertical := ""

		for row := 0; row < rows; row++ {
			// Getting from line by row/col becomes an ASCII rune. Convert to string.
			vertical += string(lines[row][col])
		}

		t := len(forwards.FindAllString(vertical, -1))
		t2 := len(backwards.FindAllString(vertical, -1))

		xmas += t + t2
	}

	// Diagonal, top-left to bottom-right
	for start := 0; start < rows; start++ {
		diagonal := ""

		for i, j := start, 0; i < rows && j < cols; i, j = i+1, j+1 {
			diagonal += string(lines[i][j])
		}

		if len(diagonal) < 4 {
			break
		}

		t := len(forwards.FindAllString(diagonal, -1))
		t2 := len(backwards.FindAllString(diagonal, -1))

		xmas += t + t2
	}

	for start := 1; start < cols; start++ {
		diagonal := ""

		for i, j := 0, start; i < rows && j < cols; i, j = i+1, j+1 {
			diagonal += string(lines[i][j])
		}

		if len(diagonal) < 4 {
			break
		}

		t := len(forwards.FindAllString(diagonal, -1))
		t2 := len(backwards.FindAllString(diagonal, -1))

		xmas += t + t2
	}

	// Diagonal, top-right to bottom-left
	for start := 0; start < rows; start++ {
		diagonal := ""

		for i, j := start, cols-1; i < rows && j >= 0; i, j = i+1, j-1 {
			diagonal += string(lines[i][j])
		}

		if len(diagonal) < 4 {
			break
		}

		t := len(forwards.FindAllString(diagonal, -1))
		t2 := len(backwards.FindAllString(diagonal, -1))

		xmas += t + t2
	}

	for start := cols - 2; start >= 0; start-- {
		diagonal := ""

		for i, j := 0, start; i < rows && j >= 0; i, j = i+1, j-1 {
			diagonal += string(lines[i][j])
		}

		if len(diagonal) < 4 {
			break
		}

		t := len(forwards.FindAllString(diagonal, -1))
		t2 := len(backwards.FindAllString(diagonal, -1))

		xmas += t + t2
	}

	return xmas
}

func part2(name string) int {
	lines := files.ReadLines(name)
	xmas := 0

	rows := len(lines)
	cols := len(lines[0])

	for i, r := range lines {
		for j := range strings.Split(r, "") {
			// A's are always in the middle
			if string(r[j]) == "A" {
				// Check line above
				if i-1 >= 0 && j-1 >= 0 && i+1 < rows && j+1 < cols {
					diagonalTopLeft := string(lines[i-1][j-1])
					diagonalTopRight := string(lines[i-1][j+1])
					diagonalBottomLeft := string(lines[i+1][j-1])
					diagonalBottomRight := string(lines[i+1][j+1])

					if diagonalTopLeft == "M" && diagonalBottomRight == "S" && diagonalTopRight == "M" && diagonalBottomLeft == "S" {
						xmas++
					}

					if diagonalTopLeft == "M" && diagonalBottomRight == "S" && diagonalTopRight == "S" && diagonalBottomLeft == "M" {
						xmas++
					}

					if diagonalTopLeft == "S" && diagonalBottomRight == "M" && diagonalTopRight == "S" && diagonalBottomLeft == "M" {
						xmas++
					}

					if diagonalTopLeft == "S" && diagonalBottomRight == "M" && diagonalTopRight == "M" && diagonalBottomLeft == "S" {
						xmas++
					}
				}
			}
		}
	}

	return xmas
}
