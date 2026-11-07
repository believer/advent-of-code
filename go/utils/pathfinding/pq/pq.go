// Package pq is a generic priority queue built on container/heap
package pq

import "container/heap"

type Queue[T any] struct {
	h *items[T]
}

// New creates a queue where less decides which item is popped first.
// For a min-heap on a field, use `func(a, b T) bool { return a.X < b.X }`.
func New[T any](less func(a, b T) bool) *Queue[T] {
	return &Queue[T]{h: &items[T]{less: less}}
}

func (q *Queue[T]) Len() int { return q.h.Len() }

func (q *Queue[T]) Push(v T) { heap.Push(q.h, v) }

// Pop removes and returns the first item. Panics if the queue is empty.
func (q *Queue[T]) Pop() T { return heap.Pop(q.h).(T) }

// Peek returns the first item without removing it. Panics if the queue is empty.
func (q *Queue[T]) Peek() T { return q.h.data[0] }

type items[T any] struct {
	data []T
	less func(a, b T) bool
}

func (i items[T]) Len() int           { return len(i.data) }
func (i items[T]) Less(a, b int) bool { return i.less(i.data[a], i.data[b]) }
func (i items[T]) Swap(a, b int)      { i.data[a], i.data[b] = i.data[b], i.data[a] }
func (i *items[T]) Push(x any)        { i.data = append(i.data, x.(T)) }

func (i *items[T]) Pop() any {
	n := len(i.data)
	item := i.data[n-1]

	// Don't keep a reference around so the item can be collected
	var zero T
	i.data[n-1] = zero
	i.data = i.data[:n-1]

	return item
}
