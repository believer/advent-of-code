package bfs

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBFSWithOtherTypes(t *testing.T) {
	b := New("a")
	b.Push("b")

	assert.Equal(t, "a", b.Pop())
	assert.Equal(t, "b", b.Pop())
	assert.False(t, b.Loop())

	b.Visit("a")
	assert.True(t, b.HasVisited("a"))
	assert.False(t, b.HasVisited("b"))
}
