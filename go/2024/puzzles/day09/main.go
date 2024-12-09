package main

import (
	"fmt"
	"slices"
	"strconv"
	"strings"

	"github.com/believer/aoc-2024/utils"
	"github.com/believer/aoc-2024/utils/files"
)

// Did string manipulation for part 1 at first, but of course that became
// hard to manage with file IDs bigger than 9 like the example input.
// Changing to a slice didn't change the code much and even simplified some parts.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	line := strings.TrimSpace(files.Read(name))
	blocked := []string{}
	fileId := 0
	fileBlocks := 0

	// Build files and free space
	for i, digit := range strings.Split(line, "") {
		digitAsInt := utils.MustIntFromString(digit)

		for range digitAsInt {
			if i%2 == 0 {
				blocked = append(blocked, strconv.Itoa(fileId))
				fileBlocks += 1
			} else {
				blocked = append(blocked, ".")
			}
		}

		if i%2 == 0 {
			fileId++
		}
	}

	// Compact file system
	for i := 1; i < len(blocked); i++ {
		rune := blocked[len(blocked)-i]

		if rune == "." {
			continue
		}

		firstEmpty := slices.Index(blocked, ".")

		// The blocks have been compacted
		if firstEmpty == fileBlocks {
			break
		}

		blocked[firstEmpty] = rune
		blocked[len(blocked)-i] = "."
	}

	// Calculate checksum
	checksum := 0

	for i, rune := range blocked {
		if rune == "." {
			break
		}

		digit := utils.MustIntFromString(string(rune))

		checksum += i * digit
	}

	return checksum
}

func part2(name string) int {
	return 0
}
