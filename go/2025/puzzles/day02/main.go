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

func part1(name string) int {
	line := strings.TrimSpace(files.Read(name))
	ranges := strings.Split(line, ",")
	invalid := 0

	for _, r := range ranges {
		i, i2, _ := strings.Cut(r, "-")
		s := utils.MustIntFromString(i)
		e := utils.MustIntFromString(i2)

		for i := range e - s + 1 {
			v := s + i
			ss := strconv.Itoa(v)

			half := len(ss) / 2
			c := ss[:half]
			cc := ss[half:]

			if c == cc {
				invalid += v
			}
		}
	}

	return invalid
}

func part2(name string) int {
	line := strings.TrimSpace(files.Read(name))
	ranges := strings.Split(line, ",")
	invalid := 0
	m := map[int]int{}

	for _, r := range ranges {
		i, i2, _ := strings.Cut(r, "-")
		s := utils.MustIntFromString(i)
		e := utils.MustIntFromString(i2)

		for i := range e - s + 1 {
			v := s + i
			ss := strconv.Itoa(v)
			sa := strings.Split(ss, "")

			for c := range len(ss) - 1 {
				a := chunkBy(sa, c+1)
				allEqual := true
				firstChunk := a[0]

				if len(a[0]) != len(a[1]) {
					break
				}

				for _, d := range a {
					if strings.Join(d, "") != strings.Join(firstChunk, "") {
						allEqual = false
						break
					}
				}

				if allEqual {
					m[v] = v
				}
			}

		}
	}

	for _, v := range m {
		invalid += v
	}

	return invalid
}

func chunkBy[T any](items []T, chunkSize int) (chunks [][]T) {
	for chunkSize < len(items) {
		items, chunks = items[chunkSize:], append(chunks, items[0:chunkSize:chunkSize])
	}
	return append(chunks, items)
}
