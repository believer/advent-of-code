package dsu

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDSU(t *testing.T) {
	d := New(5)

	assert.Equal(t, 5, d.Count)
	assert.NotEqual(t, d.Find(0), d.Find(1))

	assert.True(t, d.Union(0, 1))
	assert.False(t, d.Union(1, 0), "already joined")
	assert.Equal(t, d.Find(0), d.Find(1))
	assert.Equal(t, 4, d.Count)

	assert.True(t, d.Union(2, 3))
	assert.True(t, d.Union(1, 3))
	assert.Equal(t, 2, d.Count)
	assert.Equal(t, d.Find(0), d.Find(2))
	assert.Equal(t, 4, d.Size[d.Find(0)])
	assert.Equal(t, 1, d.Size[d.Find(4)])
}
