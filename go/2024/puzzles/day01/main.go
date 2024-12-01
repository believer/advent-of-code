package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

// Will come back later on today and see what I can improve on
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	left := []int{}
	right := []int{}
	total := 0

	for _, line := range lines {
		row := strings.Split(line, "   ")

		l, err := strconv.Atoi(row[0])

		if err != nil {
			panic(err)
		}

		r, err := strconv.Atoi(row[1])

		if err != nil {
			panic(err)
		}

		left = append(left, l)
		right = append(right, r)
	}

	slices.Sort(left)
	slices.Sort(right)

	for i, l := range left {
		total += utils.Abs(l - right[i])
	}

	return total
}

func part2(name string) int {
	lines := files.ReadLines(name)
	left := []int{}
	right := []int{}
	total := 0

	for _, line := range lines {
		row := strings.Split(line, "   ")

		l, err := strconv.Atoi(row[0])

		if err != nil {
			panic(err)
		}

		r, err := strconv.Atoi(row[1])

		if err != nil {
			panic(err)
		}

		left = append(left, l)
		right = append(right, r)
	}

	for _, l := range left {
		appears := 0
		for _, r := range right {
			if l == r {
				appears += 1
			}
		}

		total += l * appears
	}

	return total
}
