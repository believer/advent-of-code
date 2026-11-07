package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := 1227775554
		actual := part1("test-input.txt")
		assert.Equal(t, expected, actual)
	})

	t.Run("Part 1 - edge cases", func(t *testing.T) {
		expected := 224838
		actual := part1("test-input-edge.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart2(t *testing.T) {
	t.Run("Part 2", func(t *testing.T) {
		expected := 4174379265
		actual := part2("test-input.txt")
		assert.Equal(t, expected, actual)
	})

	t.Run("Part 2 - edge cases", func(t *testing.T) {
		expected := 10225947
		actual := part2("test-input-edge.txt")
		assert.Equal(t, expected, actual)
	})
}

func BenchmarkPart1(b *testing.B) {
	for b.Loop() {
		part1("input.txt")
	}
}

func BenchmarkPart2(b *testing.B) {
	for b.Loop() {
		part2("input.txt")
	}
}
