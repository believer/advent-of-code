package main

import (
	"fmt"

	"github.com/believer/aoc-2025/utils/files"
	"github.com/believer/aoc-2025/utils/grid"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	department := grid.New(lines)
	accessible := 0

	for y := range department.Height {
		for x := range department.Width {
			rolls := 0
			p := grid.Point{X: x, Y: y}

			// No paper here
			if department.Get(p) != '@' {
				continue
			}

			for _, dir := range grid.ALL_DIRECTIONS {
				check := grid.Point{X: p.X + dir.X, Y: p.Y + dir.Y}

				if _, ok := department.Contains(check); !ok {
					continue
				}

				if department.Get(check) == '@' {
					rolls += 1
				}
			}

			if rolls < 4 {
				accessible += 1
			}
		}
	}

	return accessible
}

func part2(name string) int {
	return 0
}
