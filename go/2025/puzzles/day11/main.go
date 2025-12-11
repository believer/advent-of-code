package main

import (
	"fmt"
	"strings"

	"github.com/believer/aoc-utils/files"
)

// Pretty simple DP solution. Nice breather after yesterday.
// The thing I didn't know was how to create a good cache
// representation for multiple values and recursion. So, I
// (actually) asked Gemini about how to solve that part.

func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	paths := createPaths(name)

	// DAC and FFT are not needed in part 1, set to true
	return countPaths(paths, "you", true, true)
}

func part2(name string) int {
	paths := createPaths(name)

	return countPaths(paths, "svr", false, false)
}

type cacheKey struct {
	from string
	dac  bool
	fft  bool
}

func countPaths(paths map[string][]string, start string, dac, fft bool) int {
	cache := make(map[cacheKey]int)

	var solve func(from string, dac, fft bool) int

	solve = func(from string, dac, fft bool) int {
		key := cacheKey{from, dac, fft}

		if result, ok := cache[key]; ok {
			return result
		}

		switch from {
		case "out":
			// Has visited both DAC and FFT
			if dac && fft {
				return 1
			}
			return 0
		case "dac":
			dac = true
		case "fft":
			fft = true
		}

		sum := 0

		for _, to := range paths[from] {
			sum += solve(to, dac, fft)
		}

		cache[key] = sum

		return sum
	}

	return solve(start, dac, fft)
}

// Create a map of all devices and their outputs
func createPaths(name string) map[string][]string {
	lines := files.ReadLines(name)
	paths := map[string][]string{}

	for _, l := range lines {
		device, outputs, _ := strings.Cut(l, ": ")

		paths[device] = strings.Fields(outputs)
	}

	return paths
}
