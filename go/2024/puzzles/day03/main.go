package main

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

// Wasted a couple of minutes when I didn't see that the test input had
// changed for part two and was wondering why my regex didn't find do's and don'ts...

// Changing the regex from three separate capture groups to one capture group with
// three cases made part 2 almost 39% faster.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	program := files.Read(name)
	re := regexp.MustCompile(`mul\(\d{1,3},\d{1,3}\)`)
	matches := re.FindAllString(program, -1)
	total := 0

	for _, m := range matches {
		total += calculateMultiplication(m)
	}

	return total
}

func part2(name string) int {
	program := files.Read(name)
	re := regexp.MustCompile(`(do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\))`)
	matches := re.FindAllString(program, -1)
	total := 0
	enabled := true

	for _, m := range matches {
		if strings.HasPrefix(m, "mul") && enabled {
			total += calculateMultiplication(m)
		}

		if strings.HasPrefix(m, "do(") {
			enabled = true
		}

		if strings.HasPrefix(m, "don") {
			enabled = false
		}
	}

	return total
}

func calculateMultiplication(m string) int {
	parts := strings.Split(m[4:len(m)-1], ",")
	lhs := utils.MustIntFromString(parts[0])
	rhs := utils.MustIntFromString(parts[1])

	return lhs * rhs
}
