package dijkstra

import (
	"testing"

	"github.com/believer/aoc-utils/grid"
	"github.com/stretchr/testify/assert"
)

// Walk right on a corridor and count steps
func TestDijkstraCorridor(t *testing.T) {
	g := grid.New([]string{"S...E"})
	start, end := g.MustFind('S'), g.MustFind('E')

	q := New(start, grid.RIGHT)

	for q.Len() > 0 {
		cur := q.Pop()

		if q.HasSeen() {
			continue
		}

		if q.At(end) {
			assert.Equal(t, 4, cur.Cost)
			assert.Equal(t, end, cur.Path[len(cur.Path)-1])
			return
		}

		next := cur.Node.Point.Add(grid.RIGHT)

		if g.InBounds(next) {
			q.Push(cur.Cost+1, append(append([]grid.Point{}, cur.Path...), next), Node{Point: next, Direction: grid.RIGHT})
		}
	}

	t.Fatal("end not reached")
}

func TestQueueOrdersByCost(t *testing.T) {
	q := New(grid.Point{}, grid.RIGHT)
	q.Pop()

	q.Push(5, nil, Node{Point: grid.Point{X: 5}})
	q.Push(1, nil, Node{Point: grid.Point{X: 1}})
	q.Push(3, nil, Node{Point: grid.Point{X: 3}})

	assert.Equal(t, 1, q.Pop().Cost)
	assert.Equal(t, 3, q.Pop().Cost)
	assert.Equal(t, 5, q.Pop().Cost)
}

func TestHasSeenAndExpensive(t *testing.T) {
	q := New(grid.Point{}, grid.RIGHT)
	q.Pop()
	assert.False(t, q.HasSeen())

	q.Push(10, nil, Node{Direction: grid.RIGHT})
	q.Pop()
	assert.True(t, q.HasSeen(), "same node at higher cost")

	q.Score = 5
	assert.True(t, q.IsExpensive())
}

func TestSearchGenericNodes(t *testing.T) {
	// Nodes are plain strings
	edges := map[string][]Edge[string]{
		"a": {{To: "b", Cost: 7}, {To: "c", Cost: 2}},
		"c": {{To: "b", Cost: 3}, {To: "d", Cost: 10}},
		"b": {{To: "d", Cost: 1}},
	}

	cost, ok := Search("a", func(n string) bool { return n == "d" }, func(n string) []Edge[string] {
		return edges[n]
	})

	assert.True(t, ok)
	assert.Equal(t, 6, cost)
}

func TestSearchUnreachable(t *testing.T) {
	_, ok := Search(0, func(n int) bool { return n == 5 }, func(n int) []Edge[int] { return nil })

	assert.False(t, ok)
}
