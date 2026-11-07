package graph

import (
	"cmp"
	"slices"
)

type EdgeOf[D cmp.Ordered] struct {
	P1, P2   int
	Distance D
}

type EdgesOf[D cmp.Ordered] []EdgeOf[D]

// Sort interface
func (e EdgesOf[D]) Len() int           { return len(e) }
func (e EdgesOf[D]) Swap(i, j int)      { e[i], e[j] = e[j], e[i] }
func (e EdgesOf[D]) Less(i, j int) bool { return e[i].Distance < e[j].Distance }

// Sort in place, shortest distance first
func (e EdgesOf[D]) Sort() {
	slices.SortFunc(e, func(a, b EdgeOf[D]) int {
		return cmp.Compare(a.Distance, b.Distance)
	})
}

// Edges with int distances
type Edge = EdgeOf[int]
type Edges = EdgesOf[int]
