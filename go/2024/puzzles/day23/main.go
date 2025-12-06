package main

import (
	"fmt"
	"slices"
	"sort"
	"strings"

	"github.com/believer/aoc-utils/files"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	tComputers, _ := lanParty(name)

	return tComputers
}

func part2(name string) string {
	_, largestLan := lanParty(name)

	return largestLan
}

func lanParty(name string) (int, string) {
	connectionsList := files.ReadLines(name)
	computers := map[string][]string{}
	trios := map[string]bool{}
	largestLan := []string{}

	// Map all connections
	for _, c := range connectionsList {
		first, second, _ := strings.Cut(c, "-")

		computers[first] = append(computers[first], second)
		computers[second] = append(computers[second], first)
	}

	for computer, connections := range computers {
		lan := []string{computer}

		for _, c1 := range connections {
			a := computers[c1]
			include := true

			// Create a list of all connections for current computer
			for _, name := range lan {
				if !slices.Contains(a, name) {
					include = false
					break
				}
			}

			if include {
				lan = append(lan, c1)
			}

			for _, c2 := range connections {
				if c1 == c2 {
					continue
				}

				b := computers[c2]

				// Found a connected trio, add it to the list
				if slices.Contains(a, c2) && slices.Contains(b, c1) {
					trio := []string{computer, c1, c2}
					sort.Strings(trio)

					trios[strings.Join(trio, ",")] = true
				}
			}

			slices.Sort(lan)

			// Found a new mega LAN
			if len(lan) > len(largestLan) {
				largestLan = lan
			}
		}
	}

	// Find computers starting with t
	tComputers := 0

	for t := range trios {
		if t[0] == 't' || t[3] == 't' || t[6] == 't' {
			tComputers++
		}
	}

	return tComputers, strings.Join(largestLan, ",")
}
