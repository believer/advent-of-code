package main

import (
	"container/heap"
	"fmt"
	"math"

	"github.com/believer/aoc-2024/utils/files"
	"github.com/believer/aoc-2024/utils/grid"
)

type Node struct {
	Point     grid.Point
	Direction grid.Point
}

type Item struct {
	Node Node
	Cost int
	Path []grid.Point
}

// Implement a min-heap for the the nodes
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int           { return len(pq) }
func (pq PriorityQueue) Less(i, j int) bool { return pq[i].Cost < pq[j].Cost }
func (pq PriorityQueue) Swap(i, j int)      { pq[i], pq[j] = pq[j], pq[i] }

func (pq *PriorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(*Item))
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

// Lots of pathfinding this year so far! This time I went for Dijkstra's algorithm
// and had a go at implementing it myself (like I did last year in Rust). Got some help
// from ChatGPT in how to implement a min heap in Go.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	score, _ := dijkstra(name, false)

	return score
}

func part2(name string) int {
	_, tiles := dijkstra(name, true)

	return len(tiles)
}

func dijkstra(name string, findAll bool) (int, map[grid.Point]bool) {
	lines := files.ReadLines(name)
	score := math.MaxInt

	// Create maze and find start and end points
	maze := grid.New(lines)
	start := maze.Find('S')
	end := maze.Find('E')

	// Store visited nodes with coordinates _and_ direction
	seen := map[Node]int{}
	tiles := map[grid.Point]bool{}

	// Create priority queue
	queue := &PriorityQueue{}
	heap.Init(queue)
	heap.Push(queue, &Item{Node: Node{Point: start, Direction: grid.RIGHT}, Cost: 0, Path: []grid.Point{start}})

	for queue.Len() > 0 {
		current := heap.Pop(queue).(*Item)

		// This can never be the lowest cost path
		if current.Cost > score {
			continue
		}

		// We've seen this node at a lower cost
		if v, ok := seen[current.Node]; ok && v < current.Cost {
			continue
		}

		seen[current.Node] = current.Cost

		if current.Node.Point == end && current.Cost <= score {
			score = current.Cost

			// For part 1 we only need a score, skip getting all paths
			// Small change in performance, but it's something.
			if !findAll {
				return score, tiles
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
			heap.Push(queue, &Item{
				Node: Node{Point: next, Direction: direction},
				Cost: current.Cost + 1,
				// Copy current path. Using append(slice, next) causes problems since
				// multiple items might be modifying it.
				Path: append(append([]grid.Point{}, current.Path...), next),
			})
		}

		// Add rotations
		cost := current.Cost + 1000
		paths := current.Path

		if direction == grid.UP || direction == grid.DOWN {
			heap.Push(queue, &Item{Cost: cost, Path: paths, Node: Node{Point: point, Direction: grid.LEFT}})
			heap.Push(queue, &Item{Cost: cost, Path: paths, Node: Node{Point: point, Direction: grid.RIGHT}})
		} else {
			heap.Push(queue, &Item{Cost: cost, Path: paths, Node: Node{Point: point, Direction: grid.UP}})
			heap.Push(queue, &Item{Cost: cost, Path: paths, Node: Node{Point: point, Direction: grid.DOWN}})
		}
	}

	return score, tiles
}
