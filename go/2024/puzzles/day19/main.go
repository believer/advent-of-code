package main

import (
	"fmt"
	"strings"

	"github.com/believer/aoc-utils/files"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	possibleDesigns, _ := onsenTowelDesign(name)

	return possibleDesigns
}

func part2(name string) int {
	_, totalDesigns := onsenTowelDesign(name)

	return totalDesigns
}

func onsenTowelDesign(name string) (int, int) {
	lines := files.ReadParagraphs(name)
	designs := []string{}
	patterns := lines[1]
	possibleDesigns, totalPossibleDesigns := 0, 0

	designs = append(designs, strings.Split(lines[0][0], ", ")...)

	var ways func(string) int
	designLibrary := map[string]int{}

	// Check if desired designs start with one of our designs.
	// If we find a matching design, split it off and check recursively
	// until all patterns have been matched
	ways = func(pattern string) (n int) {
		if v, ok := designLibrary[pattern]; ok {
			return v
		}

		// Save to cache when cleaning up function
		defer func() { designLibrary[pattern] = n }()

		if pattern == "" {
			return 1
		}

		for _, design := range designs {
			if strings.HasPrefix(pattern, design) {
				n += ways(pattern[len(design):])
			}
		}

		return n
	}

	for _, p := range patterns {
		if w := ways(p); w >= 1 {
			possibleDesigns++
			totalPossibleDesigns += w
		}
	}

	return possibleDesigns, totalPossibleDesigns
}
