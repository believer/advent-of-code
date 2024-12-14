package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := 12
		actual := part1("test-input.txt", 11, 7)
		assert.Equal(t, expected, actual)
	})
}

// No test for part 2 since there is no way of getting
// a Christmas tree for so few guards.

func BenchmarkPart1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		part1("input.txt", 11, 7)
	}
}

func BenchmarkPart2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		part2("input.txt", 11, 7)
	}
}
