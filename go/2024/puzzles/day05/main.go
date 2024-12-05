package main

import (
	"fmt"
	"slices"
	"strings"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

// Multiple things to improve and simplify, but both parts work
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

func part2(name string) int {
	parts := files.ReadParagraphs(name)
	orderingRules := map[string][]string{}
	orderingRulesSort := [][2]int{}
	updates := parts[1]
	total := 0

	for _, r := range parts[0] {
		x, y, _ := strings.Cut(r, "|")
		xx, yy := utils.MustIntFromString(x), utils.MustIntFromString(y)

		orderingRules[x] = append(orderingRules[x], y)
		orderingRulesSort = append(orderingRulesSort, [2]int{xx, yy})
	}

	invalid := [][]string{}

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

		if !isValid {
			invalid = append(invalid, pages)
		}
	}

	for _, inva := range invalid {
		slices.SortFunc(inva, func(a, b string) int {
			for _, r := range orderingRulesSort {
				if r[0] == utils.MustIntFromString(a) && r[1] == utils.MustIntFromString(b) {
					return -1
				}
			}

			return 1
		})

		middle := inva[len(inva)/2]

		total += utils.MustIntFromString(middle)
	}

	return total
}
