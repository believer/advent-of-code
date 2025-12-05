package main

import (
	"fmt"
	"math"
	"slices"
	"strings"

	"github.com/believer/aoc-2025/utils"
	"github.com/believer/aoc-2025/utils/files"
	interval "github.com/believer/aoc-2025/utils/range"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	sections := files.ReadParagraphs(name)
	fresh := 0
	intervals := findIntervals(sections[0])

ingredients:
	for _, ingredient := range sections[1] {
		i := utils.MustIntFromString(ingredient)

		for _, r := range intervals {
			if r.Contains(i) {
				fresh += 1
				continue ingredients
			}
		}
	}

	return fresh
}

func part2(name string) int {
	sections := files.ReadParagraphs(name)
	fresh := 0

	for _, r := range findIntervals(sections[0]) {
		fresh += r.End - r.Start + 1
	}

	return fresh
}

func findIntervals(ranges []string) []interval.Range {
	intervals := []interval.Range{}

	// Find all intervals
	for _, r := range ranges {
		s, e, _ := strings.Cut(r, "-")
		start := utils.MustIntFromString(s)
		end := utils.MustIntFromString(e)

		intervals = append(intervals, interval.Range{Start: start, End: end})
	}

	// Sort by starting point for easier merging
	slices.SortFunc(intervals, func(a, b interval.Range) int {
		return a.Start - b.Start
	})

	// Compress intervals to remove overlaps
	merged := []interval.Range{intervals[0]}

	for _, r := range intervals[1:] {
		previous := len(merged) - 1
		p := merged[previous]

		if r.Start > p.End {
			// New range, add it
			merged = append(merged, r)
		} else {
			// Set end of previous to max of current or previous
			// For example:
			// 10-14
			// 12-18
			// Becomes: 10-18
			merged[previous].End = int(math.Max(float64(p.End), float64(r.End)))
		}
	}

	return merged
}
