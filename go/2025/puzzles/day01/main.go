package main

import (
	"fmt"

	"github.com/believer/aoc-2025/utils"
	"github.com/believer/aoc-2025/utils/files"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	password, _ := parts(name)
	return password
}

func part2(name string) int {
	_, password := parts(name)
	return password
}

func parts(name string) (int, int) {
	lines := files.ReadLines(name)
	position := 50
	passwordOne := 0
	passwordTwo := 0

	for _, l := range lines {
		direction := l[:1]
		value := utils.MustIntFromString(l[1:])

		switch direction {
		case "L":
			for range value {
				position -= 1

				if position == 0 {
					passwordTwo += 1
				}

				if position < 0 {
					position = 99
				}
			}

		case "R":
			for range value {
				position += 1

				if position > 99 {
					position = 0
					passwordTwo += 1
				}
			}
		}

		if position == 0 {
			passwordOne += 1
		}
	}

	return passwordOne, passwordTwo
}
