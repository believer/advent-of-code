package main

import (
	"fmt"
	"slices"
	"strings"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

// There are things that can be improved. I think we can use
// the slices.SortFunc in some way for part 2 as well, but no
// time now.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	parts := files.ReadParagraphs(name)
	total := 0
	orderingRules := map[string][]string{}
	updates := parts[1]

	for _, r := range parts[0] {
		parts := strings.Split(r, "|")

		orderingRules[parts[0]] = append(orderingRules[parts[0]], parts[1])
	}

	for _, u := range updates {
		isValid := true
		pages := strings.Split(u, ",")

		for i, p := range pages {
			if i+1 > len(pages) {
				continue
			}

			before := pages[:i]
			rules := orderingRules[p]

			for _, r := range rules {
				if slices.Contains(before, r) {
					isValid = false
				}
			}
		}

		if isValid {
			middle := pages[len(pages)/2]

			total += utils.MustIntFromString(middle)
		}
	}

	return total
}
