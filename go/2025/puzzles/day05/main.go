package main

import (
	"fmt"
	"math"
	"slices"
	"strings"

	"github.com/believer/aoc-2025/utils"
	"github.com/believer/aoc-2025/utils/files"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	sections := files.ReadParagraphs(name)
	fresh := 0

ingredient:
	for _, r := range sections[1] {
		i := utils.MustIntFromString(r)

		for _, r := range sections[0] {
			s, e, _ := strings.Cut(r, "-")
			start := utils.MustIntFromString(s)
			end := utils.MustIntFromString(e)

			for i >= start && i <= end {
				fresh += 1

				continue ingredient
			}
		}
	}

	return fresh
}

type Range struct {
	start int
	end   int
}

func part2(name string) int {
	sections := files.ReadParagraphs(name)
	fresh := 0
	intervals := []Range{}

	// Find all intervals
	for _, r := range sections[0] {
		s, e, _ := strings.Cut(r, "-")
		start := utils.MustIntFromString(s)
		end := utils.MustIntFromString(e)

		intervals = append(intervals, Range{start, end})
	}

	// Sort by starting point for easier merging
	slices.SortFunc(intervals, func(a, b Range) int {
		return a.start - b.start
	})

	// Compress intervals to remove overlaps
	merged := []Range{intervals[0]}

	for _, r := range intervals[1:] {
		previous := len(merged) - 1
		p := merged[previous]

		if r.start > p.end {
			// New range, add it
			merged = append(merged, r)
		} else {
			// Set end of previous to max of current or previous
			// For example:
			// 10-14
			// 12-18
			// Becomes: 10-18
			merged[previous].end = int(math.Max(float64(p.end), float64(r.end)))
		}
	}

	for _, r := range merged {
		fresh += r.end - r.start + 1
	}

	return fresh
}
