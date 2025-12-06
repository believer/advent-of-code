package main

import (
	"fmt"
	"math"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/grid"
	"github.com/believer/aoc-utils/utils"
)

// Part 1 is done with recursive BFS, but this is too slow for
// part 2. But a simple cache makes it fast.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	return runRobots(name, 3)
}

func part2(name string) int {
	return runRobots(name, 26)
}

type Node struct {
	Point grid.Point
	Path  string
}

func runRobots(name string, robots int) int {
	lines := files.ReadLines(name)
	numpad := grid.New([]string{"789", "456", "123", "X0A"})

	start, _ := numpad.Find('A')
	invalid, _ := numpad.Find('X')
	total := 0

	for _, line := range lines {
		distance := 0

		for _, digit := range line {
			target, _ := numpad.Find(byte(digit))
			distance += cheapestDistance(start, target, invalid, robots, 0)
			start = target
		}

		// Shortest distance * numeric value of code.
		// For example, for code 029A and distance 50:
		// 50 * 29 (A and 0 are gone) = 1450
		total += distance * utils.MustIntFromString(line[:len(line)-1])
	}

	return total
}

func robot(path string, robots int) int {
	if robots == 1 {
		return len(path)
	}

	d := 0
	directionPad := grid.New([]string{"X^A", "<v>"})
	start, _ := directionPad.Find('A')
	invalid, _ := directionPad.Find('X')

	for _, v := range path {
		target, _ := directionPad.Find(byte(v))
		d += cheapestDistance(start, target, invalid, robots, -1)
		start = target
	}

	return d
}

var cache = make(map[string]int)

func cheapestDistance(start, target, invalid grid.Point, robots int, decrement int) int {
	key := fmt.Sprintf("%v-%v-%d-%d", start, target, robots, decrement)
	queue := []Node{{Point: start, Path: ""}}
	distance := math.MaxInt

	if d, ok := cache[key]; ok {
		return d
	}

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]
		path := current.Path

		if current.Point == target {
			temp := robot(path+"A", robots+decrement)
			distance = int(math.Min(float64(distance), float64(temp)))
			cache[key] = distance

			continue
		}

		// No stepping on the empty space of the pads
		if current.Point == invalid {
			continue
		}

		x, y := current.Point.X, current.Point.Y
		tx, ty := target.X, target.Y

		if x < tx {
			next := current.Point.Add(grid.Point{X: 1, Y: 0})
			queue = append(queue, Node{Point: next, Path: path + ">"})
		} else if x > tx {
			next := current.Point.Add(grid.Point{X: -1, Y: 0})
			queue = append(queue, Node{Point: next, Path: path + "<"})
		}

		if y < ty {
			next := current.Point.Add(grid.Point{X: 0, Y: 1})
			queue = append(queue, Node{Point: next, Path: path + "v"})
		} else if y > ty {
			next := current.Point.Add(grid.Point{X: 0, Y: -1})
			queue = append(queue, Node{Point: next, Path: path + "^"})
		}
	}

	return distance
}
