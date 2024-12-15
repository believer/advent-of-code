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
	moves := []grid.Point{}
	gps := 0

	// Get all moves from list
	for _, moveLine := range p[1] {
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

	for _, box := range warehouse.FindAll('O') {
		gps += 100*box.Y + box.X
	}

	return gps
}

func part2(name string) int {
	p := files.ReadParagraphs(name)
	warehouse := grid.New(p[0])
	doubleWarehouse := grid.FromSize(warehouse.Width*2, warehouse.Height)
	gps := 0

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

	doubleWarehouse.Debug()

	return gps
}
