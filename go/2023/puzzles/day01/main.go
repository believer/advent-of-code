package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
	"unicode"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	total := 0

	for _, l := range lines {
		total += getCalibrationValue(l)
	}

	return total
}

func part2(name string) int {
	// The words overlay by at most one character at the start and the end.
	// We can simply keep these end values and set the digit in the middle.
	translations := map[string]string{
		"one":   "o1e",
		"two":   "t2o",
		"three": "t3e",
		"four":  "f4r",
		"five":  "f5e",
		"six":   "s6x",
		"seven": "s7n",
		"eight": "e8t",
		"nine":  "n9e",
	}

	lines := files.ReadLines(name)
	total := 0

	for _, l := range lines {
		for key, t := range translations {
			l = strings.ReplaceAll(l, key, t)
		}

		total += getCalibrationValue(l)
	}

	return total
}

func getCalibrationValue(l string) int {
	first := getFirstDigit(l)
	last := getLastDigit(l)

	calibrationValue := strconv.Itoa(first) + strconv.Itoa(last)
	return utils.MustIntFromString(calibrationValue)
}

// The rune is the ASCII value of the number, by subtracing with '0' (48)
// we get the real number. For example, 49 - 48 = 1
func getFirstDigit(line string) int {
	for _, r := range line {
		if unicode.IsDigit(r) {
			return int(r - '0')
		}
	}

	panic("No digit found")
}

func getLastDigit(line string) int {
	runes := []rune(line)

	slices.Reverse(runes)

	for _, r := range runes {
		if unicode.IsDigit(r) {
			return int(r - '0')
		}
	}

	panic("No digit found")
}
