package main

import (
	"fmt"
	"strings"

	"github.com/believer/aoc-2024/utils/files"
	"github.com/believer/aoc-2024/utils/grid"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	p := files.ReadParagraphs(name)
	warehouse := grid.New(p[0])

	// Get all moves from list
	moves := getMoves(p[1])

	// Find robot start position
	robot := warehouse.Find('@')

	for _, m := range moves {
		next := robot.Add(m)

		// Move into wall, do nothing
		if warehouse.Get(next) == '#' {
			continue
		}

		// Moving a box. Find last location that's not a
		// box and move our box there
		if warehouse.Get(next) == 'O' {
			tmp := next.Add(m)

			for {
				if warehouse.Get(tmp) != 'O' {
					break
				}

				tmp = tmp.Add(m)
			}

			// Don't move boxes into wall
			if warehouse.Get(tmp) == '#' {
				continue
			}

			warehouse.Update(tmp, 'O')
		}

		// Update robot position
		warehouse.Update(next, '@')
		warehouse.Update(robot, '.')
		robot = next
	}

	return calculateGps(warehouse, 'O')
}

func part2(name string) int {
	p := files.ReadParagraphs(name)
	warehouse := grid.New(p[0])
	doubleWarehouse := grid.FromSize(warehouse.Width*2, warehouse.Height)

	// Scale warehouse
	for y := range warehouse.Height {
		for x := range warehouse.Width {
			wp := grid.Point{X: x, Y: y}
			p := grid.Point{X: x * 2, Y: y}
			p2 := grid.Point{X: x*2 + 1, Y: y}

			switch warehouse.Get(wp) {
			case '.':
				doubleWarehouse.Update(p, '.')
				doubleWarehouse.Update(p2, '.')
			case '#':
				doubleWarehouse.Update(p, '#')
				doubleWarehouse.Update(p2, '#')
			case 'O':
				doubleWarehouse.Update(p, '[')
				doubleWarehouse.Update(p2, ']')
			case '@':
				doubleWarehouse.Update(p, '@')
				doubleWarehouse.Update(p2, '.')
			}
		}
	}

	// Get all moves from list
	moves := getMoves(p[1])

	// Find robot start position
	robot := doubleWarehouse.Find('@')

	// Use BFS to find boxes
moves:
	for _, m := range moves {
		queue := []grid.Point{robot}
		boxes := map[grid.Point]byte{}

		for len(queue) > 0 {
			current := queue[0]
			queue = queue[1:]

			if _, ok := boxes[current]; ok {
				continue
			}

			boxes[current] = doubleWarehouse.Get(current)
			next := current.Add(m)

			// Add the box part and it's corresponding other side
			switch doubleWarehouse.Get(next) {
			case '#':
				continue moves
			case ']':
				queue = append(queue, next.Add(grid.LEFT))
				queue = append(queue, next)
			case '[':
				queue = append(queue, next.Add(grid.RIGHT))
				queue = append(queue, next)
			}
		}

		// Reset old box positions to empty
		for p := range boxes {
			doubleWarehouse.Update(p, '.')
		}

		// Move boxes
		for p, b := range boxes {
			doubleWarehouse.Update(p.Add(m), b)
		}

		// Move robot
		next := robot.Add(m)
		doubleWarehouse.Update(next, '@')
		doubleWarehouse.Update(robot, '.')
		robot = next
	}

	return calculateGps(doubleWarehouse, '[')
}

func getMoves(list []string) []grid.Point {
	moves := []grid.Point{}

	for _, moveLine := range list {
		for _, m := range strings.Split(moveLine, "") {
			switch m {
			case "<":
				moves = append(moves, grid.LEFT)
			case "^":
				moves = append(moves, grid.UP)
			case ">":
				moves = append(moves, grid.RIGHT)
			case "v":
				moves = append(moves, grid.DOWN)
			}
		}
	}

	return moves
}

func calculateGps(warehouse grid.Grid, needle byte) int {
	gps := 0

	for _, box := range warehouse.FindAll(needle) {
		gps += 100*box.Y + box.X
	}

	return gps
}
