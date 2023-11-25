package main

import (
	"strings"
	"testing"
)

var exampleData = `
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
  `

var elfs = strings.Split(strings.TrimSpace(exampleData), "\n\n")

func TestPart1(t *testing.T) {
	expected := 24000

	t.Run("Example data", func(t *testing.T) {
		if got := Part1(elfs); got != expected {
			t.Errorf("Part1() = %v, want %v", got, expected)
		}
	})
}

func TestPart2(t *testing.T) {
	expected := 45000

	t.Run("Example data", func(t *testing.T) {
		if got := Part2(elfs); got != expected {
			t.Errorf("Part2() = %v, want %v", got, expected)
		}
	})
}
