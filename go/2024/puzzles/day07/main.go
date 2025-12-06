package main

import (
	"fmt"
	"strconv"
	"strings"
	"sync"

	"github.com/believer/aoc-utils/files"
	"github.com/believer/aoc-utils/utils"
)

// Parallelization in Go is so nice and easy and provided a major speed-up
// in this case.

// Also reducing the amount of type conversions in the permutation
// generation made some improvement. We used to give it a []string and get back []string.
// The strings needed to be broken down into individual parts for calculations, so we
// could instead return a [][]string directly from the permutations.
func main() {
	fmt.Println("Part 1: ", part1("input.txt"))
	fmt.Println("Part 2: ", part2("input.txt"))
}

func part1(name string) int {
	lines := files.ReadLines(name)
	operators := []string{"+", "*"}

	return processLines(lines, operators)
}

func part2(name string) int {
	lines := files.ReadLines(name)
	operators := []string{"+", "*", "||"}

	return processLines(lines, operators)
}

// Each line is an independent expression, we can calculate them in parallel
func processLines(lines, operators []string) int {
	results := make(chan int, len(lines))

	var wg sync.WaitGroup

	for _, line := range lines {
		wg.Add(1)

		go func(line string) {
			defer wg.Done()
			calibrationResult := evaluateLine(line, operators)
			results <- calibrationResult
		}(line)
	}

	go func() {
		wg.Wait()
		close(results)
	}()

	total := 0

	for result := range results {
		total += result
	}

	return total
}

func evaluateLine(line string, operators []string) int {
	resultAsString, valuesAsString, _ := strings.Cut(line, ":")
	values := strings.Split(strings.TrimSpace(valuesAsString), " ")
	result := utils.MustIntFromString(resultAsString)

	permutations := allPermutations(values, operators)

	for _, expression := range permutations {
		total := 0
		operation := "+"

		for _, v := range expression {
			switch v {
			case "+", "*", "||":
				operation = v
				continue
			}

			num := utils.MustIntFromString(v)

			switch operation {
			case "+":
				total += num
			case "*":
				total *= num
			case "||":
				total = utils.MustIntFromString(strconv.Itoa(total) + v)
			}
		}

		if total == result {
			return total
		}
	}

	return 0
}

func allPermutations(nums []string, ops []string) [][]string {
	if len(nums) == 0 {
		return nil
	}

	result := [][]string{}
	generateExpressions(nums, ops, []string{nums[0]}, 1, &result)
	return result
}

func generateExpressions(nums []string, ops []string, current []string, index int, result *[][]string) {
	if index == len(nums) {
		*result = append(*result, current)
		return
	}

	for _, op := range ops {
		next := append(current, op, nums[index])
		generateExpressions(nums, ops, next, index+1, result)
	}
}
