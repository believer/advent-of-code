package bfs

type BFS[T comparable] struct {
	Queue   []T
	Visited map[T]bool
}

// Start a Breadth-first search (BFS) from a given start item,
// usually a grid.Point
func New[T comparable](start T) BFS[T] {
	return BFS[T]{
		Queue:   []T{start},
		Visited: map[T]bool{},
	}
}

// Report whether we still have items in the queue
func (b *BFS[T]) Loop() bool {
	return len(b.Queue) > 0
}

// Grab (and remove) the first item in the queue
func (b *BFS[T]) Pop() (current T) {
	current = b.Queue[0]
	b.Queue = b.Queue[1:]

	return
}

// Add a new item to the queue
func (b *BFS[T]) Push(p T) {
	b.Queue = append(b.Queue, p)
}

// Report whether we've visited a given item
func (b *BFS[T]) HasVisited(p T) (ok bool) {
	_, ok = b.Visited[p]

	return
}

// Set an item as visited
func (b *BFS[T]) Visit(p T) {
	b.Visited[p] = true
}
