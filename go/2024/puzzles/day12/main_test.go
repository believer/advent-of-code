package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := 140
		actual := part1("test-input.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart1Medium(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := 772
		actual := part1("test-input-medium.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart1Large(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := 1930
		actual := part1("test-input-large.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart2(t *testing.T) {
	t.Run("Part 2", func(t *testing.T) {
		expected := 80
		actual := part2("test-input.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart2MediumFromPart1(t *testing.T) {
	t.Run("Part 1", func(t *testing.T) {
		expected := 436
		actual := part2("test-input-medium.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart2EShape(t *testing.T) {
	t.Run("Part 2", func(t *testing.T) {
		expected := 236
		actual := part2("test-input-e.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart2Medium(t *testing.T) {
	t.Run("Part 2", func(t *testing.T) {
		expected := 368
		actual := part2("test-input-medium-part2.txt")
		assert.Equal(t, expected, actual)
	})
}

func TestPart2Large(t *testing.T) {
	t.Run("Part 2", func(t *testing.T) {
		expected := 1206
		actual := part2("test-input-large.txt")
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
