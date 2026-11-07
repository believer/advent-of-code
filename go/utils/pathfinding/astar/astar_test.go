package astar

import (
	"testing"

	"github.com/believer/aoc-utils/grid"
	"github.com/stretchr/testify/assert"
)

func TestAstarFindsShortestPath(t *testing.T) {
	g := grid.New([]string{
		"S..#",
		".#..",
		"...E",
	})
	start, end := g.MustFind('S'), g.MustFind('E')

	q := New(start, grid.RIGHT)

	for q.Len() > 0 {
		cur := q.Pop()

		if q.HasSeen() {
			continue
		}

		if q.At(end) {
			assert.Equal(t, 5, cur.Cost)
			return
		}

		for _, d := range grid.CARDINALS {
			next := cur.Node.Point.Add(d)

			if v, ok := g.TryGet(next); ok && v != '#' {
				q.Push(cur.Cost+1, nil, Node{Point: next, Direction: d}, end)
			}
		}
	}

	t.Fatal("end not reached")
}

func TestAstarCustomHeuristic(t *testing.T) {
	q := New(grid.Point{}, grid.RIGHT)
	q.Heuristic = func(a, b grid.Point) int { return 0 }
	q.Pop()

	q.Push(1, nil, Node{Point: grid.Point{X: 9}}, grid.Point{})
	q.Push(1, nil, Node{Point: grid.Point{X: 1}}, grid.Point{})

	// With a zero heuristic, ties are broken by the heap, not by distance.
	// What matters is that the heuristic is stored on the item.
	assert.Equal(t, 0, q.Pop().Heuristic)
	assert.Equal(t, 0, q.Pop().Heuristic)
}

func TestAstarDefaultHeuristicIsManhattan(t *testing.T) {
	q := New(grid.Point{}, grid.RIGHT)
	q.Pop()

	q.Push(0, nil, Node{Point: grid.Point{X: 1}}, grid.Point{X: 4, Y: 4})

	assert.Equal(t, 7, q.Pop().Heuristic)
}
