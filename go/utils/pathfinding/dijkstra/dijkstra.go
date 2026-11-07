package dijkstra

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
	Node Node
	Cost int
	Path []grid.Point
}

type Queue = pq.Queue[*Item]

func newQueue() *Queue {
	return pq.New(func(a, b *Item) bool { return a.Cost < b.Cost })
}

type Dijkstra struct {
	Current *Item
	Seen    map[Node]int
	Score   int
	Queue   *Queue
}

func New(start, direction grid.Point) Dijkstra {
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

	return Dijkstra{
		Current: &item,
		Seen:    map[Node]int{},
		Score:   math.MaxInt,
		Queue:   queue,
	}
}

// Length of queue
func (d *Dijkstra) Len() int {
	return d.Queue.Len()
}

// Grab the first item from the queue
func (d *Dijkstra) Pop() *Item {
	d.Current = d.Queue.Pop()
	return d.Current
}

// Report whether the current item is more expensive
// than the lowest score
func (d *Dijkstra) IsExpensive() bool {
	return d.Current.Cost > d.Score
}

// Report whether the current item has been seen
func (d *Dijkstra) HasSeen() bool {
	if v, ok := d.Seen[d.Current.Node]; ok && v < d.Current.Cost {
		return true
	}

	d.Seen[d.Current.Node] = d.Current.Cost

	return false
}

// Report whether we've reach a given point (usually the end)
// and if the cost is lower or equal to the lowest score
func (d *Dijkstra) At(point grid.Point) bool {
	return d.Current.Node.Point == point && d.Current.Cost <= d.Score
}

// Add a new item to the queue
func (d *Dijkstra) Push(cost int, path []grid.Point, node Node) {
	d.Queue.Push(&Item{
		Cost: cost,
		Path: path,
		Node: node,
	})
}

// Edge to another node in a graph with a cost
type Edge[N comparable] struct {
	To   N
	Cost int
}

// Search finds the lowest cost from start to any node where goal reports true,
// for any comparable node type. Reports false if no goal is reachable.
func Search[N comparable](start N, goal func(N) bool, neighbors func(N) []Edge[N]) (int, bool) {
	type state struct {
		node N
		cost int
	}

	queue := pq.New(func(a, b state) bool { return a.cost < b.cost })
	best := map[N]int{start: 0}

	queue.Push(state{start, 0})

	for queue.Len() > 0 {
		current := queue.Pop()

		if goal(current.node) {
			return current.cost, true
		}

		// A cheaper way to this node was already handled
		if current.cost > best[current.node] {
			continue
		}

		for _, e := range neighbors(current.node) {
			cost := current.cost + e.Cost

			if v, ok := best[e.To]; !ok || cost < v {
				best[e.To] = cost
				queue.Push(state{e.To, cost})
			}
		}
	}

	return 0, false
}
