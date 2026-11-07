package graph

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestEdgesSort(t *testing.T) {
	edges := Edges{{0, 1, 5}, {1, 2, 1}, {2, 3, 3}}

	sort.Sort(edges)

	assert.Equal(t, Edges{{1, 2, 1}, {2, 3, 3}, {0, 1, 5}}, edges)
}

func TestEdgesSortMethod(t *testing.T) {
	edges := Edges{{0, 1, 5}, {1, 2, 1}}
	edges.Sort()

	assert.Equal(t, Edges{{1, 2, 1}, {0, 1, 5}}, edges)
}

func TestEdgesWithFloatDistances(t *testing.T) {
	edges := EdgesOf[float64]{{0, 1, 2.5}, {1, 2, 0.5}}
	edges.Sort()

	assert.Equal(t, 0.5, edges[0].Distance)
}
