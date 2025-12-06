package main

import (
	"fmt"
	"slices"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

// Took me some time to wrap my head around solving part 2
// In the end, I just created every possible report and tested them all.
// Might be a bit brute force, but it's an easy solution at least.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	reports := parseReports(name)
	safeReports := 0

	for _, report := range reports {
		if isSafeReport(report) {
			safeReports++
		}
	}

	return safeReports
}

func part2(name string) int {
	reports := parseReports(name)
	safeReports := 0

outer:
	for _, report := range reports {
		for i := range report {
			// Check every permutation of the report
			clonedReport := slices.Clone(report)
			clonedReport = append(clonedReport[:i], clonedReport[i+1:]...)

			if isSafeReport(clonedReport) {
				safeReports++
				continue outer
			}
		}
	}

	return safeReports
}

func parseReports(name string) [][]int {
	lines := files.ReadLines(name)
	reports := make([][]int, len(lines))

	for i, row := range lines {
		levels := strings.Fields(row)
		report := make([]int, len(levels))

		for j, level := range levels {
			report[j] = utils.MustIntFromString(level)
		}

		reports[i] = report
	}

	return reports
}

// A safe report is:
//
// - At most 3 and at least 1 in difference between each number
// - Continually increasing or decreasing in number order
func isSafeReport(levels []int) bool {
	isDecreasing := levels[0] > levels[1]

	for _, w := range utils.SlidingWindow(levels, 2) {
		diff := utils.Abs(w[0] - w[1])

		if diff < 1 || diff > 3 {
			return false
		}

		isCurrentlyDecreasing := w[0] > w[1]

		if isCurrentlyDecreasing != isDecreasing {
			return false
		}
	}

	return true
}
