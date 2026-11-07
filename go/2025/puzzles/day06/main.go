package main

import (
	"fmt"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

// Holy crap did I overcomplicate part 2. I had so many loops just
// trying to get the values in the correct order. The spacing really
// threw me off. I looked into potentially getting values using
// regex, but no. Started over from scratch and tried just getting
// the values column by column. Once I did that it turned
// out it wasn't so hard after all...
//
// Part 1 is also a bit of a mess :')
//
// Performance is good enough to never go back and try
// to improve this problem again.
//
// Edit: Part 1 is cleaner (and faster even though I wasn't supposed to)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) (grandTotal int) {
	lines := files.ReadLines(name)
	data := [][]string{}
	last := len(lines) - 1

	for _, line := range lines[:last] {
		data = append(data, strings.Fields(line))
	}

	// Basically a zip
	problems := make([][]int, len(data[0]))

	for _, d := range data {
		for j, v := range d {
			problems[j] = append(problems[j], utils.MustIntFromString(v))
		}
	}

	// Do calculations
	for i, sign := range strings.Fields(lines[last]) {
		switch sign {
		case "+":
			grandTotal += utils.Sum(problems[i])
		case "*":
			grandTotal += utils.Prod(problems[i])
		}
	}

	return
}

func part2(name string) (grandTotal int) {
	lines := files.ReadLines(name)
	last := len(lines) - 1
	current := []int{}

	// Read column by column from the right to get
	// the operation after the numbers it applies to
	for i := len(lines[0]) - 1; i >= 0; i-- {
		col := make([]byte, 0, last)

		for _, line := range lines[:last] {
			col = append(col, line[i])
		}

		d := strings.TrimSpace(string(col))

		if d == "" {
			continue
		}

		current = append(current, utils.MustIntFromString(d))

		// Perform calculation and reset current
		switch lines[last][i] {
		case '+':
			grandTotal += utils.Sum(current)
			current = current[:0]
		case '*':
			grandTotal += utils.Prod(current)
			current = current[:0]
		}
	}

	return
}
