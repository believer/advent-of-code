package set

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSet(t *testing.T) {
	s := New(1, 2, 2, 3)

	assert.Equal(t, 3, s.Len())
	assert.True(t, s.Has(2))
	assert.False(t, s.Has(4))

	s.Add(4)
	assert.True(t, s.Has(4))

	s.Remove(1)
	assert.False(t, s.Has(1))
	assert.ElementsMatch(t, []int{2, 3, 4}, s.Items())
}

func TestSetOperations(t *testing.T) {
	a := New(1, 2, 3)
	b := New(3, 4)

	assert.ElementsMatch(t, []int{1, 2, 3, 4}, a.Union(b).Items())
	assert.ElementsMatch(t, []int{3}, a.Intersect(b).Items())
	assert.ElementsMatch(t, []int{1, 2}, a.Difference(b).Items())
	assert.Equal(t, 3, a.Len(), "operations do not modify the receiver")
	assert.True(t, New(1).IsSubset(a))
	assert.False(t, b.IsSubset(a))
}

func TestSetWithStructs(t *testing.T) {
	type p struct{ x, y int }

	s := New(p{1, 2})

	assert.True(t, s.Has(p{1, 2}))
	assert.False(t, s.Has(p{2, 1}))
}
