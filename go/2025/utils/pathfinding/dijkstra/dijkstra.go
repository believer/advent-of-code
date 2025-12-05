package dijkstra

import (
	"container/heap"

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
	Queue *Queue
}

func New(start, direction grid.Point) Dijkstra {
	queue := &Queue{}
	startNode := Node{
		Point:     start,
		Direction: direction,
	}

	heap.Init(queue)
	heap.Push(queue, &Item{
		Node: startNode,
		Cost: 0,
		Path: []grid.Point{start},
	})

	return Dijkstra{
		Queue: queue,
	}
}

func (d *Dijkstra) Len() int {
	return d.Queue.Len()
}

func (d *Dijkstra) Pop() *Item {
	return heap.Pop(d.Queue).(*Item)
}

func (d *Dijkstra) Push(cost int, path []grid.Point, node Node) {
	heap.Push(d.Queue, &Item{
		Cost: cost,
		Path: path,
		Node: node,
	})
}
