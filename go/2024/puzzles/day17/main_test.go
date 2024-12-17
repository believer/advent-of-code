package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1Small(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := "0,1,2"
		actual := part1("test-input-small.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart1SmallTwo(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := "4,2,5,6,7,7,7,7,3,1,0"
		actual := part1("test-input-small-two.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart1(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := "4,6,3,5,6,3,5,2,1,0"
		actual := part1("test-input.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart2(t *testing.T) {
	t.Run("Part 2", func(t *testing.T) {
		expected := 117440
		actual := part2("test-input-part2.txt")
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
