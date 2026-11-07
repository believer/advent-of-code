package counter

// Counts occurrences of comparable values. Missing keys count as zero.
type Counter[T comparable] map[T]int

func New[T comparable](items ...T) Counter[T] {
	c := Counter[T]{}

	for _, i := range items {
		c[i]++
	}

	return c
}

func (c Counter[T]) Add(v T) { c[v]++ }

func (c Counter[T]) AddN(v T, n int) { c[v] += n }

// Sum of all counts
func (c Counter[T]) Total() int {
	total := 0

	for _, n := range c {
		total += n
	}

	return total
}

// Key with the highest count and the count. Ties are broken arbitrarily.
// Empty counters give the zero value and 0.
func (c Counter[T]) MostCommon() (key T, count int) {
	first := true

	for k, n := range c {
		if first || n > count {
			key, count, first = k, n, false
		}
	}

	return
}

// Key with the lowest count and the count. Ties are broken arbitrarily.
// Empty counters give the zero value and 0.
func (c Counter[T]) LeastCommon() (key T, count int) {
	first := true

	for k, n := range c {
		if first || n < count {
			key, count, first = k, n, false
		}
	}

	return
}
