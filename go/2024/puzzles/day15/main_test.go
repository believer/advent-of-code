package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := 10092
		actual := part1("test-input.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart1Small(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := 2028
		actual := part1("test-input-small.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart2(t *testing.T) {
	t.Run("Part 2", func(t *testing.T) {
		expected := 9021
		actual := part2("test-input.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart2Small(t *testing.T) {
	t.Run("Part 2", func(t *testing.T) {
		expected := 618
		actual := part2("test-input-part2-small.txt")
		assert.Equal(t, expected, actual)
	})
}

func BenchmarkPart1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		part1("input.txt")
	}
}

func BenchmarkPart2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		part2("input.txt")
	}
}
