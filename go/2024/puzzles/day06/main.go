package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils/files"
)

type Direction int

const (
	UP Direction = iota
	RIGHT
	DOWN
	LEFT
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	visitedLocations := make(map[[2]int]struct{})

	rows := len(lines)
	cols := len(lines[0])
	guardLocation := [2]int{}
	direction := UP

	// Find initial location
	for r := range rows {
		for c := range cols {
			if string(lines[r][c]) == "^" {
				guardLocation = [2]int{r, c}
				visitedLocations[guardLocation] = struct{}{}
			}
		}
	}

	for {
		currentLocation := guardLocation

		if direction == UP {
			r := currentLocation[0] - 1
			c := currentLocation[1]

			if r < 0 {
				break
			}

			if string(lines[r][c]) == "#" {
				direction = RIGHT
				continue
			}

			guardLocation = [2]int{r, c}
			visitedLocations[guardLocation] = struct{}{}
		}

		if direction == RIGHT {
			r := currentLocation[0]
			c := currentLocation[1] + 1

			if c >= cols {
				break
			}

			if string(lines[r][c]) == "#" {
				direction = DOWN
				continue
			}

			guardLocation = [2]int{r, c}
			visitedLocations[guardLocation] = struct{}{}
		}

		if direction == DOWN {
			r := currentLocation[0] + 1
			c := currentLocation[1]

			if r >= rows {
				break
			}

			if string(lines[r][c]) == "#" {
				direction = LEFT
				continue
			}

			guardLocation = [2]int{r, c}
			visitedLocations[guardLocation] = struct{}{}
		}

		if direction == LEFT {
			r := currentLocation[0]
			c := currentLocation[1] - 1

			if c < 0 {
				break
			}

			if string(lines[r][c]) == "#" {
				direction = UP
				continue
			}

			guardLocation = [2]int{r, c}
			visitedLocations[guardLocation] = struct{}{}
		}
	}

	return len(visitedLocations)
}

func part2(name string) int {
	return 0
}
