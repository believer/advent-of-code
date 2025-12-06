package main

import (
	"fmt"
	"slices"
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

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) (grandTotal int) {
	lines := files.ReadLines(name)
	problems := [][]int{}
	signs := []string{}
	data := [][]string{}

	last := len(lines) - 1
	for l := range strings.SplitSeq(lines[last], " ") {
		if l == "" {
			continue
		}
		signs = append(signs, l)
	}

	for _, l := range lines[:last] {
		inner := []string{}

		for r := range strings.SplitSeq(l, " ") {
			if r == "" {
				continue
			}

			inner = append(inner, r)
		}

		data = append(data, inner)
	}

	for range data[0] {
		problems = append(problems, make([]int, len(data)))
	}

	for i, d := range data {
		for j, v := range d {
			problems[j][i] = utils.MustIntFromString(v)
		}
	}

	for i, p := range problems {
		sign := signs[i]
		total := 0

		for _, v := range p {
			switch sign {
			case "+":
				total += v
			case "*":
				if total == 0 {
					total = v
				} else {
					total *= v
				}
			}
		}

		grandTotal += total
	}

	return
}

func part2(name string) (grandTotal int) {
	lines := files.ReadLines(name)
	cols := [][]string{}

	// Transpose
	for i := range len(lines[0]) {
		col := []string{}

		for _, line := range lines {
			col = append(col, string(line[i]))
		}

		cols = append(cols, col)
	}

	// Reverse to get operation on last line
	slices.Reverse(cols)
	current := []int{}

	for _, v := range cols {
		last := len(v) - 1
		d := strings.TrimSpace(strings.Join(v[:last], ""))

		if d == "" {
			continue
		}

		digits := utils.MustIntFromString(d)
		current = append(current, digits)
		operation := v[last]

		// Perform calculation and reset current
		switch operation {
		case "+":
			grandTotal += utils.Sum(current)
			current = []int{}
		case "*":
			grandTotal += utils.Prod(current)
			current = []int{}
		}
	}

	return
}
