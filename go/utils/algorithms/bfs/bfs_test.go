package bfs

import (
	"testing"

	"github.com/believer/aoc-utils/grid"
	"github.com/stretchr/testify/assert"
)

func TestBFS(t *testing.T) {
	b := New(grid.Point{X: 0, Y: 0})

	assert.True(t, b.Loop())

	first := b.Pop()
	assert.Equal(t, grid.Point{X: 0, Y: 0}, first)
	assert.False(t, b.Loop())

	b.Push(grid.Point{X: 1, Y: 0})
	b.Push(grid.Point{X: 2, Y: 0})

	assert.Equal(t, grid.Point{X: 1, Y: 0}, b.Pop(), "FIFO order")

	assert.False(t, b.HasVisited(first))
	b.Visit(first)
	assert.True(t, b.HasVisited(first))
}
