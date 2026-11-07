package interval

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestContains(t *testing.T) {
	r := Range{Start: 3, End: 5}

	assert.False(t, r.Contains(2))
	assert.True(t, r.Contains(3))
	assert.True(t, r.Contains(5))
	assert.False(t, r.Contains(6))
}

func TestLen(t *testing.T) {
	assert.Equal(t, 3, Range{3, 5}.Len())
	assert.Equal(t, 1, Range{3, 3}.Len())
}

func TestOverlaps(t *testing.T) {
	assert.True(t, Range{1, 5}.Overlaps(Range{5, 9}))
	assert.True(t, Range{1, 9}.Overlaps(Range{3, 4}))
	assert.False(t, Range{1, 4}.Overlaps(Range{5, 9}))
}

func TestMerge(t *testing.T) {
	merged := Merge([]Range{{10, 14}, {1, 3}, {12, 18}, {16, 20}, {30, 31}})

	assert.Equal(t, []Range{{1, 3}, {10, 20}, {30, 31}}, merged)
	assert.Empty(t, Merge([]Range{}))
}

func TestMergeDoesNotModifyInput(t *testing.T) {
	in := []Range{{5, 6}, {1, 2}}
	Merge(in)

	assert.Equal(t, []Range{{5, 6}, {1, 2}}, in)
}

func TestRangeOtherIntegerTypes(t *testing.T) {
	r := RangeOf[int64]{Start: 1, End: 1_000_000_000_000}

	assert.True(t, r.Contains(5))
	assert.Equal(t, int64(1_000_000_000_000), r.Len())
}
