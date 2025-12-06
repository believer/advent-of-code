package main

import (
	"fmt"
	"slices"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

// Was able to improve performance a bunch by doing less work (shocker!)
// For instance, using the same hash map structure of rules for both parts
// improved the benchmark by about 75%.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	parts := files.ReadParagraphs(name)
	total := 0
	orderingRules := createOrderingRules(parts[0])

	for _, u := range parts[1] {
		pages := strings.Split(u, ",")

		if isValidUpdate(pages, orderingRules) {
			total += middleValue(pages)
		}
	}

	return total
}

func part2(name string) int {
	parts := files.ReadParagraphs(name)
	total := 0
	orderingRules := createOrderingRules(parts[0])

	for _, update := range parts[1] {
		pages := strings.Split(update, ",")

		if !isValidUpdate(pages, orderingRules) {
			slices.SortFunc(pages, func(a, b string) int {
				for k, r := range orderingRules {
					if k == a && slices.Contains(r, b) {
						return -1
					}
				}

				return 1
			})

			total += middleValue(pages)
		}
	}

	return total
}

func isValidUpdate(pages []string, rules map[string][]string) bool {
	for i, page := range pages {
		before := pages[:i]

		for _, rule := range rules[page] {
			if slices.Contains(before, rule) {
				return false
			}
		}
	}

	return true
}

func createOrderingRules(rules []string) map[string][]string {
	orderingRules := map[string][]string{}

	for _, r := range rules {
		x, y, _ := strings.Cut(r, "|")

		orderingRules[x] = append(orderingRules[x], y)
	}

	return orderingRules
}

func middleValue(values []string) int {
	middle := values[len(values)/2]

	return utils.MustIntFromString(middle)
}
