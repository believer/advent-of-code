package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils/files"
	"github.com/believer/aoc-2024/utils/grid"
	"github.com/believer/aoc-2024/utils/pathfinding/dijkstra"
)

// Lots of pathfinding this year so far! This time I went for Dijkstra's algorithm
// and had a go at implementing it myself (like I did last year in Rust). Got some help
// from ChatGPT in how to implement a min heap in Go.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	score, _ := solve(name, false)

	return score
}

func part2(name string) int {
	_, tiles := solve(name, true)

	return len(tiles)
}

func solve(name string, findAll bool) (int, map[grid.Point]bool) {
	lines := files.ReadLines(name)

	// Create maze and find start and end points
	maze := grid.New(lines)
	start := maze.Find('S')
	end := maze.Find('E')

	// Store visited nodes with coordinates _and_ direction
	tiles := map[grid.Point]bool{}

	// Create priority queue
	queue := dijkstra.New(start, grid.RIGHT)

	for queue.Len() > 0 {
		current := queue.Pop()

		// This can never be the lowest cost path
		if queue.IsExpensive() {
			continue
		}

		// We've seen this node at a lower cost
		if queue.HasSeen() {
			continue
		}

		if queue.At(end) {
			queue.Score = current.Cost

			// For part 1 we only need a score, skip getting all paths
			// Small change in performance, but it's something.
			if !findAll {
				return queue.Score, tiles
			}

			// Set all unique tiles
			for _, p := range current.Path {
				tiles[p] = true
			}

			continue
		}

		point := current.Node.Point
		direction := current.Node.Direction
		next := point.Add(direction)

		// Walk forwards
		if maze.Get(next) != '#' {
			queue.Push(
				current.Cost+1,
				// Copy current path. Using append(slice, next) causes problems since
				// multiple items might be modifying it.
				append(append([]grid.Point{}, current.Path...), next),
				dijkstra.Node{Point: next, Direction: direction},
			)
		}

		// Add rotations
		cost := current.Cost + 1000
		paths := current.Path

		if direction == grid.UP || direction == grid.DOWN {
			queue.Push(cost, paths, dijkstra.Node{Point: point, Direction: grid.LEFT})
			queue.Push(cost, paths, dijkstra.Node{Point: point, Direction: grid.RIGHT})
		} else {
			queue.Push(cost, paths, dijkstra.Node{Point: point, Direction: grid.UP})
			queue.Push(cost, paths, dijkstra.Node{Point: point, Direction: grid.DOWN})
		}
	}

	return queue.Score, tiles
}
