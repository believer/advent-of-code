package main

import (
	"fmt"
	"strings"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
	"github.com/believer/aoc-2024/utils/grid"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt", 101, 103))
	fmt.Println("Part 2: ", part2("input.txt", 101, 103))
}

type Robot struct {
	position grid.Point
	velocity grid.Point
}

func part1(name string, width, height int) int {
	lines := files.ReadLines(name)
	tiles := grid.FromSize(width, height)
	robots := parseRobots(lines)
	safetyFactor := 1
	time := 100

	// Move robots
	// Optimization: We can move straight to the final location
	// for each robot, instead of iterating 100 times.
	for _, robot := range robots {
		final := grid.Point{
			X: (robot.position.X + time*robot.velocity.X) % width,
			Y: (robot.position.Y + time*robot.velocity.Y) % height,
		}

		// Increment or set number of guard in next location
		if v := tiles.GetWrapped(final); v >= '0' && v <= '9' {
			tiles.UpdateWrapped(final, v+1)
		} else {
			tiles.UpdateWrapped(final, '1')
		}
	}

	// Get data from quadrants
	for _, q := range tiles.GetQuadrants() {
		robots := 0

		for _, b := range q {
			if b == '.' {
				continue
			}

			// It's some sort of number, remove 0 (byte 48) to get corresponding int
			robots += int(b - '0')
		}

		safetyFactor *= robots
	}

	return safetyFactor
}

func part2(name string, width, height int) int {
	lines := files.ReadLines(name)
	tiles := grid.FromSize(width, height)
	robots := parseRobots(lines)
	seconds := 0

	// Move robots
	for s := range 10000 {
		for i, robot := range robots {
			next := robot.position.Add(robot.velocity)

			// Increment or set number of guard in next location
			if v := tiles.GetWrapped(next); v >= '0' && v <= '9' {
				tiles.UpdateWrapped(next, v+1)
			} else {
				tiles.UpdateWrapped(next, '1')
			}

			// Decrement or set old position to empty
			if v := tiles.GetWrapped(robot.position); v > '1' && v <= '9' {
				tiles.UpdateWrapped(robot.position, v-1)
			} else {
				tiles.UpdateWrapped(robot.position, '.')
			}

			robots[i].position = next
		}

		// Assume that for a Christmas tree to appear there are no overlapping guards
		overlaps := true

		for _, d := range tiles.Data {
			if d != '1' && d != '.' {
				overlaps = false
			}
		}

		if overlaps {
			// Zero based indexing
			seconds = s + 1
			break
		}
	}

	return seconds
}

func parseRobots(lines []string) []Robot {
	robots := []Robot{}

	// Parse out robots
	for _, line := range lines {
		position, velocity, _ := strings.Cut(line, " ")
		x, y, _ := strings.Cut(position[2:], ",")
		vx, vy, _ := strings.Cut(velocity[2:], ",")

		robots = append(robots, Robot{
			position: grid.Point{X: utils.MustIntFromString(x), Y: utils.MustIntFromString(y)},
			velocity: grid.Point{X: utils.MustIntFromString(vx), Y: utils.MustIntFromString(vy)},
		})
	}

	return robots
}
