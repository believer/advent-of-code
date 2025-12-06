package main

import (
	"fmt"
	"math"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/grid"
	"github.com/believer/aoc-utils/utils"
)

type Node struct {
	Point grid.Point
	Steps int
}

// Another day, another grid puzzle that can be solved with BFS
// Part 2 is binary search to find the first byte that makes it
// impossible to navigate the maze
func main() {
	fmt.Println("Part 1: ", part1("input.txt", 70, 1024))
	fmt.Println("Part 2: ", part2("input.txt", 70))
}

func part1(name string, size, steps int) int {
	bytes := files.ReadLines(name)

	return findPath(bytes, size, steps)
}

func part2(name string, size int) string {
	bytes := files.ReadLines(name)
	low := 0
	high := len(bytes) - 1

	for {
		if low >= high {
			break
		}

		middle := (low + high) / 2
		minCost := findPath(bytes, size, middle+1)

		if minCost != math.MaxInt {
			low = middle + 1
		} else {
			high = middle
		}
	}

	return bytes[low]
}

func findPath(bytes []string, size, steps int) int {
	memory := grid.FromSize(size+1, size+1)
	visited := map[grid.Point]bool{}
	start := grid.Point{X: 0, Y: 0}
	end := grid.Point{X: size, Y: size}
	queue := []Node{{Point: start, Steps: 0}}
	minSteps := math.MaxInt

	// Add corrupted memory
	for i, b := range bytes {
		x, y, _ := strings.Cut(b, ",")

		if i >= steps {
			break
		}

		point := grid.Point{
			X: utils.MustIntFromString(x),
			Y: utils.MustIntFromString(y),
		}

		memory.Update(point, '#')
	}

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		// Found end with new minimum cost
		if current.Point == end && current.Steps < minSteps {
			minSteps = current.Steps
		}

		// We've seen this before
		if _, ok := visited[current.Point]; ok {
			continue
		}

		visited[current.Point] = true

		// Add next position to queue if not a corrupted memory space
		for _, direction := range grid.CARDINALS {
			next := current.Point.Add(direction)

			if value, ok := memory.TryGet(next); ok && value != '#' {
				queue = append(queue, Node{Point: next, Steps: current.Steps + 1})
			}
		}
	}

	return minSteps
}
