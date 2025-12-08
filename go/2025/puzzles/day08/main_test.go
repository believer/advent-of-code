package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := 40
		actual := part1("test-input.txt", 10)
		assert.Equal(t, expected, actual)
	})
}

func TestPart2(t *testing.T) {
	t.Run("Part 2", func(t *testing.T) {
		expected := 25272
		actual := part2("test-input.txt")
		assert.Equal(t, expected, actual)
	})
}

func BenchmarkPart1(b *testing.B) {
	for b.Loop() {
		part1("input.txt", 1000)
	}
}

func BenchmarkPart2(b *testing.B) {
	for b.Loop() {
		part2("input.txt")
	}
}
