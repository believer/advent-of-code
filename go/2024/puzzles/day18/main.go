package main

import (
	"fmt"
	"math"
	"strings"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
	"github.com/believer/aoc-2024/utils/grid"
)

type Node struct {
	Point grid.Point
	Cost  int
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
	latest := 0
	firstImpossible := len(bytes)

	for {
		if (firstImpossible - latest) <= 1 {
			break
		}

		middle := (firstImpossible + latest) / 2
		minCost := findPath(bytes, size, middle)

		if minCost != math.MaxInt {
			latest = middle
		} else {
			firstImpossible = middle
		}

	}

	return bytes[firstImpossible-1]
}

func findPath(bytes []string, size, steps int) int {
	memory := grid.FromSize(size+1, size+1)
	visited := map[grid.Point]bool{}
	start := grid.Point{X: 0, Y: 0}
	end := grid.Point{X: memory.Width - 1, Y: memory.Height - 1}
	queue := []Node{{Point: start, Cost: 0}}
	minCost := math.MaxInt

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
		if current.Point == end && current.Cost < minCost {
			minCost = current.Cost
		}

		// We've seen this before
		if _, ok := visited[current.Point]; ok {
			continue
		}

		visited[current.Point] = true

		// Add next position to queue if not a corrupted memory space
		for _, direction := range grid.CARDINALS {
			next := current.Point.Add(direction)

			if value, ok := memory.Contains(next); ok && value != '#' {
				queue = append(queue, Node{Point: next, Cost: current.Cost + 1})
			}
		}
	}

	return minCost
}
