package main

import (
	"fmt"
	"iter"
	"strconv"
	"strings"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

// Instead of checking every ID in the ranges, we generate the invalid
// ones directly. An ID made of a repeated block is the block times a
// multiplier, e.g. 123123 = 123 * 1001 and 121212 = 12 * 10101.

func part1(name string) (invalid int) {
	for _, r := range idRanges(name) {
		for length := digits(r.start); length <= digits(r.end); length++ {
			if length%2 != 0 {
				continue
			}

			for id := range repeated(r, length, length/2) {
				invalid += id
			}
		}
	}

	return
}

func part2(name string) (invalid int) {
	// IDs like 1111 are both 1 x 4 and 11 x 2
	seen := map[int]bool{}

	for _, r := range idRanges(name) {
		for length := digits(r.start); length <= digits(r.end); length++ {
			for size := 1; size <= length/2; size++ {
				if length%size != 0 {
					continue
				}

				for id := range repeated(r, length, size) {
					if !seen[id] {
						seen[id] = true
						invalid += id
					}
				}
			}
		}
	}

	return
}

type idRange struct {
	start, end int
}

// All IDs in the range with the given length that consist
// of a block with the given size repeated
func repeated(r idRange, length, size int) iter.Seq[int] {
	return func(yield func(int) bool) {
		multiplier := (pow10(length) - 1) / (pow10(size) - 1)

		// Smallest and largest block that keeps the ID in range,
		// without leading zeroes or overflowing into a longer block
		lo := max(pow10(size-1), (r.start+multiplier-1)/multiplier)
		hi := min(pow10(size)-1, r.end/multiplier)

		for block := lo; block <= hi; block++ {
			if !yield(block * multiplier) {
				return
			}
		}
	}
}

func idRanges(name string) (ranges []idRange) {
	line := strings.TrimSpace(files.Read(name))

	for r := range strings.SplitSeq(line, ",") {
		s, e, _ := strings.Cut(r, "-")

		ranges = append(ranges, idRange{
			start: utils.MustIntFromString(s),
			end:   utils.MustIntFromString(e),
		})
	}

	return
}

func digits(n int) int {
	return len(strconv.Itoa(n))
}

func pow10(n int) int {
	result := 1

	for range n {
		result *= 10
	}

	return result
}
