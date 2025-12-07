package bfs

import "github.com/believer/aoc-utils/grid"

type BFS struct {
	Queue   []grid.Point
	Visited map[grid.Point]bool
}

// Start a Breadth-first search (BFS) from a given start point
func New(start grid.Point) BFS {
	queue := []grid.Point{start}

	return BFS{
		Queue:   queue,
		Visited: map[grid.Point]bool{},
	}
}

// Report whether we still have items in the queue
func (b *BFS) Loop() bool {
	return len(b.Queue) > 0
}

// Grab (and remove) the first item in the queue and
func (b *BFS) Pop() (current grid.Point) {
	current = b.Queue[0]
	b.Queue = b.Queue[1:]

	return
}

// Add a new point to the queue
func (b *BFS) Push(p grid.Point) {
	b.Queue = append(b.Queue, p)
}

// Report whether we've visited a given point
func (b *BFS) HasVisited(p grid.Point) (ok bool) {
	_, ok = b.Visited[p]

	return
}

// Set a point as visited
func (b *BFS) Visit(p grid.Point) {
	b.Visited[p] = true
}
