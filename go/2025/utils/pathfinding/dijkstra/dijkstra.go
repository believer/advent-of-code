package dijkstra

import (
	"container/heap"
	"math"

	"github.com/believer/aoc-2025/utils/grid"
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

type Queue []*Item

func (pq Queue) Len() int           { return len(pq) }
func (pq Queue) Less(i, j int) bool { return pq[i].Cost < pq[j].Cost }
func (pq Queue) Swap(i, j int)      { pq[i], pq[j] = pq[j], pq[i] }

func (pq *Queue) Push(x any) {
	*pq = append(*pq, x.(*Item))
}

func (pq *Queue) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]

	return item
}

type Dijkstra struct {
	Current *Item
	Seen    map[Node]int
	Score   int
	Queue   *Queue
}

func New(start, direction grid.Point) Dijkstra {
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
	d.Current = heap.Pop(d.Queue).(*Item)
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
	heap.Push(d.Queue, &Item{
		Cost: cost,
		Path: path,
		Node: node,
	})
}
