package main

import (
	"fmt"
	"math"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/grid"
	"github.com/believer/aoc-utils/utils"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) (largestArea int) {
	lines := files.ReadLines(name)
	coords := []grid.Point{}

	for _, l := range lines {
		x, y, _ := strings.Cut(l, ",")

		coords = append(coords, grid.Point{
			X: utils.MustIntFromString(x),
			Y: utils.MustIntFromString(y),
		})
	}

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

func part2(name string) (largestArea int) {
	lines := files.ReadLines(name)
	coords := []grid.Point{}

	for _, l := range lines {
		x, y, _ := strings.Cut(l, ",")

		coords = append(coords, grid.Point{
			X: utils.MustIntFromString(x),
			Y: utils.MustIntFromString(y),
		})
	}

	return
}
