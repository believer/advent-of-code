package set

import "maps"

type Set[T comparable] map[T]struct{}

func New[T comparable](items ...T) Set[T] {
	s := make(Set[T], len(items))

	for _, i := range items {
		s[i] = struct{}{}
	}

	return s
}

func (s Set[T]) Add(v T)      { s[v] = struct{}{} }
func (s Set[T]) Remove(v T)   { delete(s, v) }
func (s Set[T]) Len() int     { return len(s) }
func (s Set[T]) Has(v T) bool { _, ok := s[v]; return ok }

// Items in no particular order
func (s Set[T]) Items() []T {
	items := make([]T, 0, len(s))

	for k := range s {
		items = append(items, k)
	}

	return items
}

// All items in either set. Returns a new set.
func (s Set[T]) Union(o Set[T]) Set[T] {
	result := maps.Clone(s)
	maps.Copy(result, o)

	return result
}

// Items in both sets. Returns a new set.
func (s Set[T]) Intersect(o Set[T]) Set[T] {
	result := Set[T]{}

	for k := range s {
		if o.Has(k) {
			result.Add(k)
		}
	}

	return result
}

// Items in s that are not in o. Returns a new set.
func (s Set[T]) Difference(o Set[T]) Set[T] {
	result := Set[T]{}

	for k := range s {
		if !o.Has(k) {
			result.Add(k)
		}
	}

	return result
}

// Report whether every item in s is also in o
func (s Set[T]) IsSubset(o Set[T]) bool {
	for k := range s {
		if !o.Has(k) {
			return false
		}
	}

	return true
}
