package main

import (
	"fmt"

	"github.com/believer/aoc-2024/utils/files"
)

// I didn't understand much of the descriptions today. I went by
// looking at the examples to get a grasp of what to do.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

type Location struct {
	r, c int
}

func part1(name string) int {
	lines := files.ReadLines(name)
	rows := len(lines)
	cols := len(lines[0])

	towerLocations := findTowers(lines, rows, cols)

	// Find antinodes
	antinodes := make(map[Location]bool)

	for _, locations := range towerLocations {
		for i := 0; i < len(locations); i++ {
			for j := i + 1; j < len(locations); j++ {
				towerOne := locations[i]
				towerTwo := locations[j]

				dr := towerTwo.r - towerOne.r
				dc := towerTwo.c - towerOne.c

				antinodeOne := Location{r: towerOne.r - dr, c: towerOne.c - dc}
				antinodeTwo := Location{r: towerTwo.r + dr, c: towerTwo.c + dc}

				if inBounds(antinodeOne, rows, cols) {
					antinodes[antinodeOne] = true
				}

				if inBounds(antinodeTwo, rows, cols) {
					antinodes[antinodeTwo] = true
				}
			}
		}
	}

	return len(antinodes)
}

func part2(name string) int {
	lines := files.ReadLines(name)
	rows := len(lines)
	cols := len(lines[0])

	towerLocations := findTowers(lines, rows, cols)

	// Find antinodes
	antinodes := make(map[Location]bool)

	for _, locations := range towerLocations {
		for i := 0; i < len(locations); i++ {
			for j := i + 1; j < len(locations); j++ {
				towerOne := locations[i]
				towerTwo := locations[j]

				dr := towerTwo.r - towerOne.r
				dc := towerTwo.c - towerOne.c

				locationsInLineWithOne := findLocationsInLine(towerOne, dr, dc, rows, cols)
				locationsInLineWithTwo := findLocationsInLine(towerOne, dr*-1, dc*-1, rows, cols)

				for _, l := range locationsInLineWithOne {
					antinodes[l] = true
				}

				for _, l := range locationsInLineWithTwo {
					antinodes[l] = true
				}

				if len(locationsInLineWithOne) == 1 {
					antinodes[towerTwo] = true
				}

				if len(locationsInLineWithTwo) == 1 {
					antinodes[towerOne] = true
				}

				if len(locationsInLineWithOne) >= 2 || len(locationsInLineWithTwo) >= 2 {
					antinodes[towerOne] = true
					antinodes[towerTwo] = true
				}
			}
		}
	}

	return len(antinodes)
}

// Find location of all antenna towers
func findTowers(lines []string, rows, cols int) map[string][]Location {
	towerLocations := make(map[string][]Location)

	for r := range rows {
		for c := range cols {
			spot := string(lines[r][c])

			if spot != "." {
				towerLocations[spot] = append(towerLocations[spot], Location{r, c})
			}
		}
	}

	return towerLocations
}

func findLocationsInLine(location Location, dr, dc, rows, cols int) []Location {
	lastLocation := location
	var locations []Location

	for {
		lastLocation.r -= dr
		lastLocation.c -= dc

		if inBounds(lastLocation, rows, cols) {
			locations = append(locations, lastLocation)
		} else {
			break
		}
	}

	return locations
}

func inBounds(location Location, rows, cols int) bool {
	if location.c >= 0 && location.c < cols && location.r >= 0 && location.r < rows {
		return true
	}

	return false
}
