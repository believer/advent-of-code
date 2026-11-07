package interval

import (
	"cmp"
	"slices"

	"github.com/believer/aoc-utils/utils"
)

// Inclusive range of integers
type RangeOf[T utils.Integer] struct {
	Start T
	End   T
}

// Range of ints
type Range = RangeOf[int]

func (r RangeOf[T]) Contains(v T) bool {
	return v >= r.Start && v <= r.End
}

// Number of values in the range (both ends included)
func (r RangeOf[T]) Len() T {
	return r.End - r.Start + 1
}

// Report whether two ranges share at least one value
func (r RangeOf[T]) Overlaps(o RangeOf[T]) bool {
	return r.Start <= o.End && o.Start <= r.End
}

// Merge overlapping ranges into a sorted list of distinct ranges.
// Ranges that only touch (1-2 and 3-4) are kept separate.
func Merge[T utils.Integer](ranges []RangeOf[T]) []RangeOf[T] {
	if len(ranges) == 0 {
		return []RangeOf[T]{}
	}

	sorted := slices.Clone(ranges)
	slices.SortFunc(sorted, func(a, b RangeOf[T]) int {
		return cmp.Compare(a.Start, b.Start)
	})

	merged := []RangeOf[T]{sorted[0]}

	for _, r := range sorted[1:] {
		last := &merged[len(merged)-1]

		if r.Start > last.End {
			merged = append(merged, r)
		} else {
			last.End = max(last.End, r.End)
		}
	}

	return merged
}
