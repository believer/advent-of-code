package astar

import (
	"math"

	"github.com/believer/aoc-utils/grid"
	"github.com/believer/aoc-utils/pathfinding/pq"
)

type Node struct {
	Point     grid.Point
	Direction grid.Point
}

type Item struct {
	Node      Node
	Cost      int
	Heuristic int
	Path      []grid.Point
}

type Queue = pq.Queue[*Item]

func newQueue() *Queue {
	return pq.New(func(a, b *Item) bool {
		return a.Cost+a.Heuristic < b.Cost+b.Heuristic
	})
}

type Astar struct {
	Current *Item
	Seen    map[Node]int
	Score   int
	Queue   *Queue

	// Estimates the cost from a point to the goal. Must never overestimate.
	// Defaults to Manhattan distance.
	Heuristic func(from, goal grid.Point) int
}

func New(start, direction grid.Point) Astar {
	queue := newQueue()
	startNode := Node{
		Point:     start,
		Direction: direction,
	}

	item := Item{
		Node: startNode,
		Cost: 0,
		Path: []grid.Point{start},
	}

	queue.Push(&item)

	return Astar{
		Current: &item,
		Seen:    map[Node]int{},
		Score:   math.MaxInt,
		Queue:   queue,

		Heuristic: grid.Point.ManhattanDistance,
	}
}

// Length of queue
func (a *Astar) Len() int {
	return a.Queue.Len()
}

// Grab the first item from the queue
func (a *Astar) Pop() *Item {
	a.Current = a.Queue.Pop()
	return a.Current
}

// Report whether the current item is more expensive
// than the lowest score
func (a *Astar) IsExpensive() bool {
	return a.Current.Cost > a.Score
}

// Report whether the current item has been seen
func (a *Astar) HasSeen() bool {
	if v, ok := a.Seen[a.Current.Node]; ok && v < a.Current.Cost {
		return true
	}

	a.Seen[a.Current.Node] = a.Current.Cost

	return false
}

// Report whether we've reach a given point (usually the end)
// and if the cost is lower or equal to the lowest score
func (a *Astar) At(point grid.Point) bool {
	return a.Current.Node.Point == point
}

// Add a new item to the queue
func (a *Astar) Push(cost int, path []grid.Point, node Node, goal grid.Point) {
	h := a.Heuristic(node.Point, goal)

	a.Queue.Push(&Item{
		Cost:      cost,
		Path:      path,
		Heuristic: h,
		Node:      node,
	})
}
