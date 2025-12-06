package astar

import (
	"container/heap"
	"math"

	"github.com/believer/aoc-utils/grid"
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

type Queue []*Item

func (pq Queue) Len() int { return len(pq) }
func (pq Queue) Less(i, j int) bool {
	return pq[i].Cost+pq[i].Heuristic < pq[j].Cost+pq[j].Heuristic
}
func (pq Queue) Swap(i, j int) { pq[i], pq[j] = pq[j], pq[i] }
func (pq *Queue) Push(x any)   { *pq = append(*pq, x.(*Item)) }

func (pq *Queue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]

	return item
}

type Astar struct {
	Current *Item
	Seen    map[Node]int
	Score   int
	Queue   *Queue
}

func New(start, direction grid.Point) Astar {
	queue := &Queue{}
	startNode := Node{
		Point:     start,
		Direction: direction,
	}

	item := Item{
		Node: startNode,
		Cost: 0,
		Path: []grid.Point{start},
	}

	heap.Init(queue)
	heap.Push(queue, &item)

	return Astar{
		Current: &item,
		Seen:    map[Node]int{},
		Score:   math.MaxInt,
		Queue:   queue,
	}
}

// Length of queue
func (a *Astar) Len() int {
	return a.Queue.Len()
}

// Grab the first item from the queue
func (a *Astar) Pop() *Item {
	a.Current = heap.Pop(a.Queue).(*Item)
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
	h := node.Point.ManhattanDistance(goal)

	heap.Push(a.Queue, &Item{
		Cost:      cost,
		Path:      path,
		Heuristic: h,
		Node:      node,
	})
}
