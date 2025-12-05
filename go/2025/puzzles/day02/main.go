package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/believer/aoc-2025/utils"
	"github.com/believer/aoc-2025/utils/files"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

// Lost a bit of performance on part 1 after refactor, but I think
// the increased readability makes up for it.
//
// Part 2 performance increased after refactor though.

func part1(name string) (invalid int) {
	for _, v := range inclusiveRange(name) {
		valueAsString := strconv.Itoa(v)
		half := len(valueAsString) / 2

		if valueAsString[:half] == valueAsString[half:] {
			invalid += v
		}
	}

	return
}

func part2(name string) (invalid int) {
	invalids := map[int]bool{}

	for _, v := range inclusiveRange(name) {
		values := strings.Split(strconv.Itoa(v), "")

		// Already seen this ID
		if _, ok := invalids[v]; ok {
			continue
		}

	chunk:
		for c := range len(values) - 1 {
			chunkSize := c + 1

			// Continue if it can't be evenly chunked
			if len(values)%chunkSize != 0 {
				continue
			}

			chunks := chunkBy(values, c+1)
			firstChunk := strings.Join(chunks[0], "")

			// Check if all chunks are the same
			for _, c := range chunks {
				if strings.Join(c, "") != firstChunk {
					continue chunk
				}
			}

			// Save invalid value
			invalids[v] = true
		}
	}

	// Calculate sum
	for v := range invalids {
		invalid += v
	}

	return
}

func inclusiveRange(name string) (output []int) {
	line := strings.TrimSpace(files.Read(name))
	ranges := strings.Split(line, ",")

	for _, r := range ranges {
		s, e, _ := strings.Cut(r, "-")
		start := utils.MustIntFromString(s)
		end := utils.MustIntFromString(e)

		for i := range end - start + 1 {
			output = append(output, start+i)
		}
	}

	return
}

func chunkBy[T any](items []T, chunkSize int) (chunks [][]T) {
	for chunkSize < len(items) {
		items, chunks = items[chunkSize:], append(chunks, items[0:chunkSize:chunkSize])
	}

	return append(chunks, items)
}
